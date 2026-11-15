"""C.R.I.S — Jezero data pipeline.

Builds (a) the 10 m planning grids the Bevy sim embeds (sim/assets/jezero/), (b) the 1 km detail tiles the
sim streams while the camera gets close (public/data/jezero/tiles/), and (c) small JSON files for the web app
(public/data/jezero/). Stages: selftest, terrain (runs color), color, ortho2, rocks, traverse, labels, autolabel,
hazard, radiation, thermal.

Run from the repo root:  python notebooks/jezero_pipeline.py <stage>
Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
"""

import argparse
import csv
import datetime as dt
import io
import json
import math
import os
import re
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

import numpy as np
import requests
from PIL import Image, ImageDraw

Image.MAX_IMAGE_PIXELS = None   # the ortho mosaic is ~260 Mpx

R = 3396190.0                      # Mars 2000 sphere, m
M_PER_DEG = 59274.6975233062185    # from the DTM .lbl "Scale"
CENTER_LAT, CENTER_LON = 18.4447, 77.4508   # Octavia E. Butler landing site
SIZE_M = 10000.0                   # region side
GRID = 1000                        # planning grid: 10 m cells
CELL_M = SIZE_M / GRID
FINE_M = 2.0                       # fine analysis grid (slope, roughness, hazard patches)
FN = int(SIZE_M / FINE_M)          # 5000
TILE_M = 1000.0                    # streamed detail tile side
NT = int(SIZE_M / TILE_M)          # 10 x 10 tiles
TILE_VERTS = 251                   # 4 m vertex spacing
TILE_ORTHO_PX = 1024               # ~1 m/px
TILE_ORTHO2_PX = 2048              # near-camera tiles, ~0.5 m/px
ORTHO2_Z = 17                      # Trek max zoom for the 25 cm layer: ~0.32 m/px source
ORTHO2_NOTE = "; near-camera tiles from the same orthomosaic at Trek zoom 17 (~0.32 m/px)"
DEM_URL = "https://asc-pds-services.s3.us-west-2.amazonaws.com/mosaic/mars2020_trn/HiRISE/JEZ_hirise_soc_006_DTM_MOLAtopography_DeltaGeoid_1m_Eqc_latTs0_lon0_blend40.tif"
DEM_START = 172609                 # .lbl Core StartByte (1-based) - 1
DEM_SAMPLES, DEM_UL_X, DEM_UL_Y = 21400, 4577366.0, 1106618.0   # float32 LE, 1 m/px, equirect clat 0
TREK_TILE = "https://trek.nasa.gov/tiles/Mars/EQ/{layer}/1.0.0/default/default028mm/{z}/{row}/{col}.png"
ORTHO_LAYER = "JEZ_hirise_soc_006_orthoMosaic_25cm_Eqc_latTs0_lon0_first_dd"
# Mars 2020 colour basemap (HiRISE RGB + CTX, NASA/JPL-Caltech/UArizona): web-mercator TMS tiles, y flipped.
COLOR_TILE = "https://mars.nasa.gov/mmgis-maps/M20/Layers/M20_HiRISE_RGB_CTX_mosaic_25cm_v7_Merge_LERC_clip/{z}/{x}/{y}.png"
COLOR_Z = 16                       # ~1.2 m/px at Jezero; only the chroma is used
# Mars 2020 Science Team photogeologic map (Stack et al. 2020, SSR 216:127), electronic supplement 3.
ROCKS_URL = ("https://static-content.springer.com/esm/art%3A10.1007%2Fs11214-020-00739-x/MediaObjects/"
             "11214_2020_739_MOESM3_ESM.zip")
ROCKS_SHP = "M2020_JEZ_geology_edited_equi0_allquads_v6_archive.shp"
# In situ geologic map from Perseverance observations (Crumpler et al. 2023, JGR Planets, doi:10.1029/2022JE007444;
# M2020 Science Team), one shapefile per unit, lon/lat on the Mars 2000 sphere. CC BY 4.0.
INSITU_FILE = "https://zenodo.org/api/records/7545299/files/{name}/content"
INSITU_ROUGH = ("Coarse Regolith", "Mixed Regolith", "Float-Lag")   # loose clasts/blocks underfoot
INSITU_BLOCKS = "Crater Ejecta Blocks"                              # individual blocks (points)
INSITU_RIPPLES = "Ripples"
# Perseverance drive path (NASA/JPL-Caltech, Mars 2020 MMGIS): LineStrings in lon/lat.
DRIVE_PATH_URL = "https://mars.nasa.gov/mmgis-maps/M20/Layers/json/M20_traverse.json"
GEO_URL = "https://services.arcgis.com/v01gqwM5QqNysAAi/arcgis/rest/services/Jezero_FeatureServiceFromPro/FeatureServer/8/query"
CITATION_URL = ("https://services.arcgis.com/v01gqwM5QqNysAAi/arcgis/rest/services/PGM_Map_Database_MARS_INTERACTIVE2/"
                "FeatureServer/1/query?where=USGSMapNumber%20LIKE%20%27%253464%25%27&outFields=MapTitle,SuggestedCitation&f=json")
CITATION_FALLBACK = ("USGS SIM 3464, Geologic map of Jezero crater and the Nili Planum region, Mars "
                     "(https://doi.org/10.3133/sim3464)")
THEMIS_DAY = "THEMIS_DayIR_ControlledMosaics_100m_v2_oct2018"
THEMIS_NIGHT = "THEMIS_NightIR_ControlledMosaics_100m_v2_oct2018"
GIBS_TILE = "https://gibs.earthdata.nasa.gov/wmts/epsg4326/best/{layer}/default/{date}/1km/{z}/{row}/{col}.png"
GIBS_CMAP = "https://gibs.earthdata.nasa.gov/colormaps/v1.3/MODIS_Land_Surface_Temp.xml"
EARTH_LAT, EARTH_LON = -24.5, -69.25   # Atacama Desert
INSIGHT_URL = "https://api.nasa.gov/insight_weather/"
WAYPOINTS_URL = "https://mars.nasa.gov/mmgis-maps/M20/Layers/json/M20_waypoints.json"

ROOT = Path(__file__).resolve().parent.parent
RAW = ROOT / "data/raw/jezero"
PROC = ROOT / "data/processed/jezero"
ASSETS = ROOT / "sim/assets/jezero"
WEB = ROOT / "public/data/jezero"
TILES = WEB / "tiles"
LABELS_CSV = ROOT / "notebooks/labels/jezero_patches.csv"
NASA_LABELS_CSV = ROOT / "notebooks/labels/jezero_patches_nasa.csv"   # written by `autolabel`, not by hand
PATCH = 24          # fine cells (48 m)
N_GEO = 12          # geology one-hot width
SLOPE_SCALE = 0.25  # deg per u8 step
ROUGH_SCALE = 0.02  # m per u8 step

_session = requests.Session()
_session.headers["User-Agent"] = "cris-planner (NASA Space Apps 2026)"
_session.mount("https://", requests.adapters.HTTPAdapter(pool_maxsize=16))


# ---------- geometry ----------

def bbox():
    dlat = SIZE_M / M_PER_DEG
    dlon = SIZE_M / (M_PER_DEG * math.cos(math.radians(CENTER_LAT)))
    return {"lat_min": CENTER_LAT - dlat / 2, "lat_max": CENTER_LAT + dlat / 2,
            "lon_min": CENTER_LON - dlon / 2, "lon_max": CENTER_LON + dlon / 2}


def tile(z, lat, lon):
    """Trek EQ tile (row, col) containing lat/lon. 256 px tiles, origin (-180, 90)."""
    deg = 180 / 2 ** z
    return math.floor((90 - lat) / deg), math.floor((lon + 180) / deg)


def gibs_index(lat, lon, z=6):
    """GIBS EPSG:4326 tile (z, row, col, px, py); 512 px tiles, level 0 = 0.5625 deg/px."""
    res = 0.5625 / 2 ** z
    x, y = (lon + 180) / res, (90 - lat) / res
    return z, int(y // 512), int(x // 512), int(x % 512), int(y % 512)


def world_xz(lat, lon, bb):
    """World metres: x east, z south, origin at the region centre."""
    x = (lon - bb["lon_min"]) / (bb["lon_max"] - bb["lon_min"]) * SIZE_M - SIZE_M / 2
    z = (bb["lat_max"] - lat) / (bb["lat_max"] - bb["lat_min"]) * SIZE_M - SIZE_M / 2
    return x, z


# ---------- io helpers ----------

def fetch(url, cache: Path, **kw):
    """GET with on-disk cache; returns bytes or None when the server has no such object (403/404)."""
    if cache.exists():
        return cache.read_bytes()
    r = _session.get(url, timeout=120, **kw)
    if r.status_code in (403, 404):   # S3/CloudFront answer 403 for tiles outside a layer's footprint
        return None
    r.raise_for_status()
    cache.parent.mkdir(parents=True, exist_ok=True)
    cache.write_bytes(r.content)
    return r.content


def trek_mosaic(layer, z, bb, kind):
    """Mosaic the L channel of Trek tiles covering bb (parallel fetch, cached) and crop to it."""
    r0, c0 = tile(z, bb["lat_max"], bb["lon_min"])
    r1, c1 = tile(z, bb["lat_min"], bb["lon_max"])
    jobs = [(r, c) for r in range(r0, r1 + 1) for c in range(c0, c1 + 1)]
    get = lambda rc: fetch(TREK_TILE.format(layer=layer, z=z, row=rc[0], col=rc[1]),
                           RAW / f"tiles/{kind}/{z}_{rc[0]}_{rc[1]}.png")
    with ThreadPoolExecutor(12) as ex:
        datas = list(ex.map(get, jobs))
    mosaic = Image.new("L", ((c1 - c0 + 1) * 256, (r1 - r0 + 1) * 256))
    missing = 0
    for (r, c), data in zip(jobs, datas):
        if data is None:
            missing += 1
            continue
        mosaic.paste(Image.open(io.BytesIO(data)).convert("L"), ((c - c0) * 256, (r - r0) * 256))
    if missing:
        print(f"warning: {missing}/{len(jobs)} {kind} tiles missing (404)")
    px = 256 * 2 ** z / 180   # px per degree
    box = ((bb["lon_min"] + 180) * px - c0 * 256, (90 - bb["lat_max"]) * px - r0 * 256,
           (bb["lon_max"] + 180) * px - c0 * 256, (90 - bb["lat_min"]) * px - r0 * 256)
    return mosaic.crop(tuple(round(v) for v in box))


def load_meta():
    return json.loads((ASSETS / "meta.json").read_text())


def save_meta(meta):
    for d in (ASSETS, WEB):
        d.mkdir(parents=True, exist_ok=True)
        (d / "meta.json").write_text(json.dumps(meta, indent=1, ensure_ascii=False) + "\n")


def set_credit(meta, layer, text):
    meta["credits"] = [c for c in meta["credits"] if c["layer"] != layer] + [{"layer": layer, "text": text}]


def save_u8(name, arr):
    Image.fromarray(np.clip(np.round(arr), 0, 255).astype(np.uint8), "L").save(ASSETS / name)


def resize_f(a, size, resample):
    return np.asarray(Image.fromarray(np.ascontiguousarray(a, dtype=np.float32), "F").resize(size, resample))


def box_mean(a, k):
    """k x k moving average with edge padding (integral image)."""
    p = k // 2
    # float64 and mean-removed: a float32 integral image of ~-2500 m heights loses metres of precision
    m = float(np.mean(a))
    s = np.pad(np.asarray(a, dtype=np.float64) - m, p, mode="edge").cumsum(0).cumsum(1)
    s = np.pad(s, ((1, 0), (1, 0)))
    return (s[k:, k:] - s[:-k, k:] - s[k:, :-k] + s[:-k, :-k]) / (k * k) + m


def blocks(a, k):
    """(H, W) -> (H/k, W/k, k*k) view of non-overlapping k x k blocks."""
    h, w = a.shape
    return a.reshape(h // k, k, w // k, k).swapaxes(1, 2).reshape(h // k, w // k, k * k)


# ---------- stages ----------

def cmd_selftest(_):
    assert tile(9, 18.4447, 77.4508) == (203, 732), tile(9, 18.4447, 77.4508)
    bb = bbox()
    exp = {"lat_min": 18.360347, "lat_max": 18.529053, "lon_min": 77.361878, "lon_max": 77.539722}
    for k, v in exp.items():
        assert abs(bb[k] - v) < 1e-4, (k, bb[k], v)
    assert gibs_index(EARTH_LAT, EARTH_LON) == (6, 25, 24, 312, 227), gibs_index(EARTH_LAT, EARTH_LON)
    assert world_xz(CENTER_LAT, CENTER_LON, bb) == (0.0, 0.0) or max(map(abs, world_xz(CENTER_LAT, CENTER_LON, bb))) < 1e-6
    a = np.arange(36, dtype=float).reshape(6, 6)
    assert np.allclose(box_mean(a, 3)[2, 2], a[1:4, 1:4].mean())
    assert blocks(a, 3).shape == (2, 2, 9) and blocks(a, 3)[1, 1].max() == 35
    print("selftest ok")


def fetch_dem(bb):
    r0 = math.floor(DEM_UL_Y - M_PER_DEG * bb["lat_max"])
    r1 = math.ceil(DEM_UL_Y - M_PER_DEG * bb["lat_min"])
    c0 = math.floor(M_PER_DEG * bb["lon_min"] - DEM_UL_X)
    c1 = math.ceil(M_PER_DEG * bb["lon_max"] - DEM_UL_X)
    print(f"DEM window rows {r0}-{r1} cols {c0}-{c1}")
    cache = RAW / f"dem_rows_{r0}_{r1}.bin"
    size = (r1 - r0) * DEM_SAMPLES * 4
    if not cache.exists() or cache.stat().st_size != size:
        start = DEM_START + r0 * DEM_SAMPLES * 4
        print(f"fetching {size / 1e6:.0f} MB of DEM rows in 8 parallel ranges ...")
        cache.parent.mkdir(parents=True, exist_ok=True)
        tmp = cache.with_suffix(".part")
        with open(tmp, "wb") as f:
            f.truncate(size)
        bounds = np.linspace(0, size, 9).astype(np.int64)

        def part(i):
            a, b = int(bounds[i]), int(bounds[i + 1])
            with _session.get(DEM_URL, headers={"Range": f"bytes={start + a}-{start + b - 1}"}, stream=True,
                              timeout=900) as r:
                r.raise_for_status()
                with open(tmp, "r+b") as f:
                    f.seek(a)
                    for chunk in r.iter_content(1 << 20):
                        f.write(chunk)

        with ThreadPoolExecutor(8) as ex:
            list(ex.map(part, range(8)))
        tmp.rename(cache)
    a = np.array(np.memmap(cache, dtype="<f4", mode="r").reshape(-1, DEM_SAMPLES)[:, c0:c1])
    if (a < -1e30).any() or a.min() < -2700 or a.max() > -1900:
        sys.exit("DEM window invalid")
    return a


def geology(bb):
    params = {"where": "1=1", "geometry": f'{bb["lon_min"]},{bb["lat_min"]},{bb["lon_max"]},{bb["lat_max"]}',
              "geometryType": "esriGeometryEnvelope", "inSR": 4326, "spatialRel": "esriSpatialRelIntersects",
              "outFields": "*", "outSR": 4326, "f": "geojson"}
    r = _session.get(GEO_URL, params=params, timeout=120)
    r.raise_for_status()
    feats = r.json().get("features", [])
    if not feats:
        sys.exit("geology query returned no features")
    props = feats[0]["properties"]
    key = next((k for k in ["UnitSymbol", "Unit", "UNIT", "Symbol", "symbol", "UnitName", "unit_name"] if k in props),
               next(k for k, v in props.items() if isinstance(v, str)))
    print(f"geology unit key: {key} ({len(feats)} features)")

    def label(p):   # symbol plus the unit name when the layer has one, e.g. "Njf — Jezero floor unit"
        return f'{p[key]} — {p["name"]}' if p.get("name") and p.get("name") != p[key] else str(p[key])

    labels = sorted({label(f["properties"]) for f in feats if f["properties"].get(key) is not None})
    ids = {lab: i + 1 for i, lab in enumerate(labels)}

    def to_px(ring):
        return [((x - bb["lon_min"]) / (bb["lon_max"] - bb["lon_min"]) * GRID,
                 (bb["lat_max"] - y) / (bb["lat_max"] - bb["lat_min"]) * GRID) for x, y in ring]

    polys = []
    for f in feats:
        g = f.get("geometry")
        if f["properties"].get(key) is None or not g:
            continue
        rings = [g["coordinates"][0]] if g["type"] == "Polygon" else [p[0] for p in g["coordinates"]]
        for ring in rings:
            pts = to_px(ring)
            area = abs(sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1]))) / 2
            polys.append((area, ids[label(f["properties"])], pts))
    img = Image.new("L", (GRID, GRID), 0)
    draw = ImageDraw.Draw(img)
    for _, uid, pts in sorted(polys, key=lambda p: -p[0]):
        draw.polygon(pts, fill=uid)
    img.save(ASSETS / "geology.png")

    citation = CITATION_FALLBACK
    try:
        rows = _session.get(CITATION_URL, timeout=60).json().get("features", [])
        if rows and rows[0]["attributes"].get("SuggestedCitation"):
            citation = rows[0]["attributes"]["SuggestedCitation"]
    except (requests.RequestException, ValueError) as e:
        print(f"warning: citation lookup failed ({e}); using fallback")
    return [{"id": i, "label": lab} for lab, i in ids.items()], citation


def write_tiles(h2, h_min, h_max):
    """Detail tile heights: 251x251 u16 (4 m spacing, shared edges) per 1 km tile. Orthos: cmd_color."""
    from scipy.ndimage import map_coordinates
    TILES.mkdir(parents=True, exist_ok=True)
    nv = NT * (TILE_VERTS - 1) + 1                     # 2501 global vertices per side
    pos = np.arange(nv) * (TILE_M / (TILE_VERTS - 1))  # metres from the west/north edge
    idx = np.clip(pos / FINE_M - 0.5, 0, FN - 1)       # fine-grid cell-centre coordinates
    rr, cc = np.meshgrid(idx, idx, indexing="ij")
    hv = map_coordinates(h2, [rr, cc], order=1)
    q = np.round((hv - h_min) / (h_max - h_min) * 65535).astype(np.uint16)
    step = TILE_VERTS - 1
    for ty in range(NT):
        for tx in range(NT):
            Image.fromarray(q[ty * step:ty * step + TILE_VERTS, tx * step:tx * step + TILE_VERTS]).save(
                TILES / f"h_{ty}_{tx}.png")
    print(f"wrote {NT * NT} detail tile heights to {TILES}")


def color_ratio(bb, gray4):
    """Chroma (RGB / grey) on the grid of gray4 from the Mars 2020 colour basemap. Outside its footprint
    (it stops ~1.4 km short of the east edge and ~0.5 km short of the north edge) the chroma comes from a
    grey-level lookup fitted where both images exist, feathered over ~25 cells at the seam."""
    from scipy.ndimage import map_coordinates
    n = 2 ** COLOR_Z

    def merc_px(lat):   # global XYZ pixel row, 256 px tiles
        r = np.radians(lat)
        return (1 - np.log(np.tan(r) + 1 / np.cos(r)) / np.pi) / 2 * n * 256

    x0, x1 = (int((bb[k] + 180) / 360 * n) for k in ("lon_min", "lon_max"))
    y0, y1 = (int(merc_px(bb[k]) // 256) for k in ("lat_max", "lat_min"))
    jobs = [(x, y) for y in range(y0, y1 + 1) for x in range(x0, x1 + 1)]
    get = lambda xy: fetch(COLOR_TILE.format(z=COLOR_Z, x=xy[0], y=n - 1 - xy[1]),
                           RAW / f"tiles/color/{COLOR_Z}_{xy[0]}_{xy[1]}.png")
    with ThreadPoolExecutor(12) as ex:
        datas = list(ex.map(get, jobs))
    mosaic = np.zeros(((y1 - y0 + 1) * 256, (x1 - x0 + 1) * 256, 4), np.uint8)
    for (x, y), data in zip(jobs, datas):
        if data is not None:
            mosaic[(y - y0) * 256:(y - y0 + 1) * 256, (x - x0) * 256:(x - x0 + 1) * 256] = \
                np.asarray(Image.open(io.BytesIO(data)).convert("RGBA"))
    print(f"colour tiles: {sum(d is not None for d in datas)}/{len(jobs)} present")

    m = gray4.shape[0]
    t = (np.arange(m) + 0.5) / m
    rows = merc_px(bb["lat_max"] - t * (bb["lat_max"] - bb["lat_min"])) - y0 * 256 - 0.5
    cols = (bb["lon_min"] + t * (bb["lon_max"] - bb["lon_min"]) + 180) / 360 * n * 256 - x0 * 256 - 0.5
    rr, cc = np.meshgrid(rows, cols, indexing="ij")
    rgba = np.stack([map_coordinates(mosaic[..., k], [rr, cc], order=1, output=np.float32) for k in range(4)], -1)
    del mosaic, rr, cc
    valid = (rgba[..., 3] > 250) & (rgba[..., :3].max(-1) > 8)
    ratio = rgba[..., :3] / np.maximum(rgba[..., :3].mean(-1), 1)[..., None]
    del rgba

    g = np.clip(np.round(gray4), 0, 255).astype(np.int32)
    gv, rv = g[valid], ratio[valid]
    fallback = np.median(rv, 0)
    lut = np.array([np.median(rv[gv == v], 0) if (gv == v).sum() > 100 else fallback for v in range(256)],
                   np.float32)
    fill = lut[g]
    w = box_mean(valid.astype(np.float32), 25)[..., None]
    ratio = w * np.where(valid[..., None], ratio, fill) + (1 - w) * fill
    print(f"colour basemap covers {valid.mean() * 100:.1f}% of the region")
    return ratio.astype(np.float32)


def colorize(gray, ratio):
    """Grey HiRISE pixels times the chroma ratio resampled to the same size -> RGB uint8."""
    h, w = gray.shape
    r = np.stack([resize_f(ratio[..., k], (w, h), Image.BILINEAR) for k in range(3)], -1)
    return Image.fromarray(np.clip(gray[..., None] * r, 0, 255).astype(np.uint8), "RGB")


def cmd_terrain(_):
    for d in (RAW, PROC, ASSETS, WEB):
        d.mkdir(parents=True, exist_ok=True)
    bb = bbox()

    a = fetch_dem(bb)
    h2 = resize_f(a, (FN, FN), Image.BOX)   # 2 m fine grid
    del a
    h_min, h_max = float(h2.min()), float(h2.max())
    print(f"h_min {h_min:.2f} h_max {h_max:.2f}")
    np.save(PROC / "height2.npy", h2)

    gy, gx = np.gradient(h2, FINE_M)
    slope2 = np.degrees(np.arctan(np.hypot(gx, gy))).astype(np.float32)
    del gx, gy
    rough2 = np.abs(h2 - box_mean(h2, 5)).astype(np.float32)   # local relief residual, m
    np.save(PROC / "slope2.npy", slope2)
    np.save(PROC / "rough2.npy", rough2)

    k = int(CELL_M / FINE_M)   # 5 fine cells per planning cell
    h10 = blocks(h2, k).mean(-1)
    v = np.round((h10 - h_min) / (h_max - h_min) * 65535).astype(np.uint16)
    Image.fromarray(v).save(ASSETS / "height.png")
    # 90th percentile of the 2 m slopes inside each 10 m cell: conservative, robust to single-pixel noise.
    slope10 = np.percentile(blocks(slope2, k), 90, axis=-1)
    rough10 = np.percentile(blocks(rough2, k), 90, axis=-1)
    save_u8("slope.png", slope10 / SLOPE_SCALE)
    save_u8("rough.png", rough10 / ROUGH_SCALE)
    print(f"slope10 p50 {np.median(slope10):.1f} p99 {np.percentile(slope10, 99):.1f} deg; "
          f"rough10 p99 {np.percentile(rough10, 99):.2f} m")

    ortho = trek_mosaic(ORTHO_LAYER, 16, bb, "ortho")
    print(f"ortho crop {ortho.size}")
    ortho1 = np.asarray(ortho.resize((int(SIZE_M), int(SIZE_M)), Image.BILINEAR))   # 1 m/px
    del ortho
    Image.fromarray(ortho1).save(PROC / "ortho_1m.png")
    # Optical texture: local contrast (std of the 1 m HiRISE pixels in each 10 m cell); bouldery/rough
    # surfaces are speckled, smooth sand/regolith is uniform. Relative, scaled to the region's p99.
    tex = blocks(ortho1.astype(np.float32), int(CELL_M)).std(-1)
    tex_scale = float(np.percentile(tex, 99)) / 255
    save_u8("texture.png", tex / tex_scale)

    write_tiles(h2, h_min, h_max)
    units, citation = geology(bb)

    for name in ("hazard.png", "thermal.png", "radiation.png", "rocks.png"):
        p = ASSETS / name
        if not p.exists() or Image.open(p).size != (GRID, GRID):
            save_u8(name, np.zeros((GRID, GRID)))

    old = load_meta() if (ASSETS / "meta.json").exists() else {}
    if old.get("size_m") != SIZE_M:   # region changed: grids from the old region are invalid
        old = {k: v for k, v in old.items() if k == "credits"}
    meta = {"site": "Jezero crater — Octavia E. Butler landing site and surroundings",
            "center": {"lat": CENTER_LAT, "lon": CENTER_LON}, "bbox": bb, "size_m": SIZE_M,
            "grid": {"w": GRID, "h": GRID, "cell_m": CELL_M},
            "tiles": {"n": NT, "size_m": TILE_M, "verts": TILE_VERTS, "ortho_px": TILE_ORTHO_PX, "path": "tiles"},
            "h_min": h_min, "h_max": h_max, "slope_scale_deg": SLOPE_SCALE, "rough_scale_m": ROUGH_SCALE,
            "texture": {"type": "HiRISE local contrast", "scale": tex_scale},
            "geology_units": units,
            "hazard_model": old.get("hazard_model", {"type": "none"}),
            "radiation": old.get("radiation", {"type": "none"}),
            "thermal": old.get("thermal", {"type": "none"}),
            "credits": [c for c in old.get("credits", [])
                        if c["layer"] not in ("elevation", "ortho", "geology", "roughness", "texture")]}
    set_credit(meta, "elevation", "HiRISE DTM mosaic (NASA/JPL/University of Arizona/USGS) via JPL Mars Trek")
    set_credit(meta, "roughness", "Derived from HiRISE DTM: local relief residual (2 m), p90 per 10 m cell")
    set_credit(meta, "texture", "Derived from HiRISE orthomosaic: local brightness contrast per 10 m cell (relative)")
    set_credit(meta, "geology", citation)
    save_meta(meta)
    print(f"geology units: {len(units)}; wrote {ASSETS} and {WEB / 'meta.json'}")
    cmd_color(None)


def _chroma(bb):
    """1 m grey ortho and the per-pixel chroma ratio of the colour basemap over the region."""
    ortho1 = np.asarray(Image.open(PROC / "ortho_1m.png").convert("L"))
    gray4 = np.asarray(Image.fromarray(ortho1).resize((2500, 2500), Image.BOX), dtype=np.float32)
    return ortho1, color_ratio(bb, gray4)


def cmd_color(_):
    """Coloured orthos: grey HiRISE (Trek, ~0.6 m/px source) for detail, chroma from the Mars 2020 colour basemap.
    Writes the region ortho (sim/assets) and the 1 km detail tile orthos."""
    bb = bbox()
    ortho1, ratio = _chroma(bb)
    colorize(np.asarray(Image.fromarray(ortho1).resize((2048, 2048), Image.LANCZOS)), ratio).save(
        ASSETS / "ortho.jpg", quality=85)
    TILES.mkdir(parents=True, exist_ok=True)
    px, rk = int(TILE_M), ratio.shape[0] // NT   # ortho1 is 1 m/px
    for ty in range(NT):
        for tx in range(NT):
            colorize(ortho1[ty * px:(ty + 1) * px, tx * px:(tx + 1) * px],
                     ratio[ty * rk:(ty + 1) * rk, tx * rk:(tx + 1) * rk]).resize(
                (TILE_ORTHO_PX, TILE_ORTHO_PX), Image.LANCZOS).save(TILES / f"o_{ty}_{tx}.jpg", quality=80)
    meta = load_meta()
    set_credit(meta, "ortho", "HiRISE orthomosaic (NASA/JPL/University of Arizona) via JPL Mars Trek; colour from the "
                              "Mars 2020 HiRISE RGB + CTX basemap (NASA/JPL-Caltech/University of Arizona) via "
                              "Mars 2020 MMGIS, extended by grey-level matching outside its footprint")
    save_meta(meta)
    print(f"wrote coloured ortho.jpg and {NT * NT} tile orthos")

def cmd_ortho2(_):
    """Near-camera 1 km orthos (o2_*.jpg) from Trek zoom 17, coloured like the o_* tiles. Fetched per chunk: a
    full z17 mosaic of the region is ~1 GB in memory. Chunks mostly outside the z17 footprint are skipped and the
    sim keeps the 1 m tile there."""
    bb = bbox()
    _, ratio = _chroma(bb)
    TILES.mkdir(parents=True, exist_ok=True)
    dlat, dlon = bb["lat_max"] - bb["lat_min"], bb["lon_max"] - bb["lon_min"]
    rk = ratio.shape[0] // NT
    wrote = 0
    for ty in range(NT):
        for tx in range(NT):
            cb = {"lat_max": bb["lat_max"] - ty * dlat / NT, "lat_min": bb["lat_max"] - (ty + 1) * dlat / NT,
                  "lon_min": bb["lon_min"] + tx * dlon / NT, "lon_max": bb["lon_min"] + (tx + 1) * dlon / NT}
            g = trek_mosaic(ORTHO_LAYER, ORTHO2_Z, cb, "ortho")
            if (np.asarray(g) == 0).mean() > 0.25:
                print(f"skip o2_{ty}_{tx}: outside z17 footprint")
                continue
            g = np.asarray(g.resize((TILE_ORTHO2_PX,) * 2, Image.LANCZOS), np.float32)
            colorize(g, ratio[ty * rk:(ty + 1) * rk, tx * rk:(tx + 1) * rk]).save(TILES / f"o2_{ty}_{tx}.jpg",
                                                                               quality=80)
            wrote += 1
            print(f"o2_{ty}_{tx} ({wrote})", flush=True)
    meta = load_meta()
    meta["tiles"]["ortho2_px"] = TILE_ORTHO2_PX
    meta["tiles"]["ortho2_dist_m"] = 400
    old = next(c["text"] for c in meta["credits"] if c["layer"] == "ortho")
    if ORTHO2_NOTE not in old:
        set_credit(meta, "ortho", old + ORTHO2_NOTE)
    save_meta(meta)
    print(f"wrote {wrote} near-camera orthos")



def fine_raster(geoms, unit, all_touched=False):
    """Burn geometries into a uint8 mask on the 2 m fine grid (row 0 = north). `unit` = map units per degree:
    M_PER_DEG for Mars2000 equidistant-cylindrical metres (clon 0), 1 for longitude/latitude degrees."""
    from rasterio.features import rasterize
    from rasterio.transform import from_bounds
    bb = bbox()
    geoms = [g for g in geoms if g is not None and not g.is_empty]
    if not geoms:
        return np.zeros((FN, FN), np.uint8)
    tf = from_bounds(bb["lon_min"] * unit, bb["lat_min"] * unit, bb["lon_max"] * unit, bb["lat_max"] * unit, FN, FN)
    return rasterize(((g, 1) for g in geoms), out_shape=(FN, FN), transform=tf, dtype="uint8", all_touched=all_touched)


def stack_map():
    """Mars 2020 Science Team photogeologic map (Stack et al. 2020) as a GeoDataFrame in eqc metres."""
    import zipfile
    import geopandas as gpd
    zpath = RAW / "stack2020_photogeologic_map.zip"
    if fetch(ROCKS_URL, zpath) is None:
        sys.exit("photogeologic map download failed")
    shp_dir = RAW / "stack2020"
    if not (shp_dir / ROCKS_SHP).exists():
        zipfile.ZipFile(zpath).extractall(shp_dir)
    g = gpd.read_file(shp_dir / ROCKS_SHP)
    return g[g.geometry.notna()]


def rocky_units(g):
    """Talus, or blocky bedrock where the surface unit (what you walk on) shows no or minor cover."""
    exposed = g.SurfUName.isna() | g.SurfUName.eq("Minor Cover Us")
    return g[g.SurfUName.eq("Talus") | (exposed & g.BedrkUName.str.contains("blocky", case=False, na=False))]


def cmd_rocks(_):
    """Rock-covered ground from the Mars 2020 Science Team photogeologic map (Stack et al. 2020): polygons
    mapped as talus or as exposed blocky bedrock. rocks.png = fraction of each 10 m cell (0..255)."""
    rocky = rocky_units(stack_map())
    fine = fine_raster(rocky.geometry, M_PER_DEG)
    frac = blocks(fine.astype(np.float32), int(CELL_M / FINE_M)).mean(-1)
    save_u8("rocks.png", frac * 255)
    units = sorted({f"{s} / {b}" for s, b in zip(rocky.SurfUName.fillna("—"), rocky.BedrkUName.fillna("—"))})
    print(f"rocky units (surface / bedrock): {units}")
    print(f"rock-covered: {(frac > 0).mean() * 100:.1f}% of cells touched, {frac.mean() * 100:.1f}% of the area")
    meta = load_meta()
    set_credit(meta, "rocks", "NASA Mars 2020 Science Team photogeologic map on HiRISE (Stack et al. 2020, Space Sci. "
                              "Rev. 216:127, doi:10.1007/s11214-020-00739-x): talus and exposed blocky-bedrock units, "
                              "area fraction per 10 m cell")
    save_meta(meta)


def cmd_traverse(_):
    """Perseverance waypoints inside the region (links places to the sol of its raw images)."""
    bb = bbox()
    data = json.loads(fetch(WAYPOINTS_URL, RAW / "M20_waypoints.json"))
    pts = []
    for f in data["features"]:
        p = f["properties"]
        if not (bb["lat_min"] <= p["lat"] <= bb["lat_max"] and bb["lon_min"] <= p["lon"] <= bb["lon_max"]):
            continue
        x, z = world_xz(p["lat"], p["lon"], bb)
        pts.append({"sol": int(p["sol"]), "lat": p["lat"], "lon": p["lon"], "x": round(x, 2), "z": round(z, 2),
                    "dist_total_m": round(float(p.get("dist_total_m") or 0), 1)})
    if not pts:
        sys.exit("no Perseverance waypoints inside the region")
    out = {"source": "Perseverance waypoints (NASA/JPL-Caltech) via mars.nasa.gov", "points": pts}
    for d in (ASSETS, WEB):
        (d / "traverse.json").write_text(json.dumps(out) + "\n")
    meta = load_meta()
    set_credit(meta, "traverse", "Perseverance rover waypoints (NASA/JPL-Caltech) via mars.nasa.gov")
    save_meta(meta)
    print(f"traverse: {len(pts)} waypoints, sols {pts[0]['sol']}..{pts[-1]['sol']}")


def load_fine():
    slope2 = np.load(PROC / "slope2.npy", mmap_mode="r")
    rough2 = np.load(PROC / "rough2.npy", mmap_mode="r")
    ortho1 = np.asarray(Image.open(PROC / "ortho_1m.png").convert("L"))
    geo = np.asarray(Image.open(ASSETS / "geology.png"))
    return slope2, rough2, ortho1, geo


def ortho_crop(ortho1, r0, c0, n=PATCH):
    """Ortho (1 m/px) pixels under fine cells r0..r0+n, c0..c0+n."""
    s = FINE_M
    return ortho1[round(r0 * s):round((r0 + n) * s), round(c0 * s):round((c0 + n) * s)]


def labels_filled():
    if not LABELS_CSV.exists():
        return 0
    with open(LABELS_CSV) as f:
        return sum(1 for r in csv.DictReader(f) if r["label"].strip())


def cmd_labels(_):
    slope2, _, ortho1, _ = load_fine()
    nb = FN // PATCH   # non-overlapping candidate blocks on the 2 m grid
    means = blocks(np.asarray(slope2[:nb * PATCH, :nb * PATCH]), PATCH).mean(-1).ravel()
    quart = np.digitize(means, np.percentile(means, [25, 50, 75]))
    rng = np.random.default_rng(0)
    picks = np.sort(np.concatenate([rng.choice(np.flatnonzero(quart == q), 100, replace=False) for q in range(4)]))
    patches = [(k, (i // nb) * PATCH + PATCH // 2, (i % nb) * PATCH + PATCH // 2) for k, i in enumerate(picks)]

    out = PROC / "label_sheets"
    out.mkdir(parents=True, exist_ok=True)
    side = 192
    for s in range(0, len(patches), 20):
        sheet = Image.new("L", (5 * (side + 8), 4 * (side + 8)), 255)
        draw = ImageDraw.Draw(sheet)
        for n, (pid, r, c) in enumerate(patches[s:s + 20]):
            crop = ortho_crop(ortho1, r - PATCH // 2, c - PATCH // 2)
            im = Image.fromarray(crop.astype(np.uint8)).resize((side, side), Image.NEAREST)
            x, y = (n % 5) * (side + 8) + 4, (n // 5) * (side + 8) + 4
            sheet.paste(im, (x, y))
            draw.rectangle((x, y, x + 34, y + 14), fill=0)
            draw.text((x + 2, y + 2), str(pid), fill=255)
        sheet.save(out / f"sheet_{s // 20:02d}.png")
    print(f"wrote {len(patches) // 20} contact sheets to {out} (48 m patches, rows/cols on the 2 m grid)")

    if labels_filled():
        print(f"{LABELS_CSV} has team labels; not overwritten")
        return
    LABELS_CSV.parent.mkdir(parents=True, exist_ok=True)
    with open(LABELS_CSV, "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["id", "row", "col", "label"])
        w.writerows([(pid, r, c, "") for pid, r, c in patches])
    print(f"wrote {LABELS_CSV} (labels: 0 safe, 1 rough/rocky, 2 impassable)")


def cmd_autolabel(_):
    """Hazard labels for every 48 m patch from NASA data and published NASA criteria: no per-patch judgement by a
    person or an AI. The rules below were written with SI (docs/SI_USAGE.md); the team can edit them. The last
    matching rule wins (impassable > rough > safe):
      0 safe        Perseverance drove through it (M20 drive path, NASA/JPL-Caltech), or >= half the patch is mapped
                    as undifferentiated smooth surface (Stack et al. 2020).
      1 rough       >= half the patch mapped as talus, exposed blocky bedrock or large aeolian bedforms (Stack et al.
                    2020; walking on sand costs 1.6-2.5x the work, NASA HRP NTRS 20220004017) or as coarse/mixed
                    regolith or float-lag in situ, or it holds a mapped ejecta block (Crumpler et al. 2023).
      2 impassable  p90 slope >= 20° (NASA suited-EVA limit, NTRS 20210026296 / EVA-EXP-0042; JPL rover limit on
                    rock, NTRS 20160008265), or >= half the patch mapped as ripples in situ (Crumpler et al. 2023;
                    JPL: avoid ripples >= 10 cm at any slope, NTRS 20160008265).
    Writes NASA_LABELS_CSV (id,row,col,label,rule); `hazard` trains on it while the team CSV has < 150 labels."""
    import geopandas as gpd
    from urllib.parse import quote
    nb = FN // PATCH
    n = nb * PATCH

    def frac(mask):
        return blocks(mask[:n, :n].astype(np.float32), PATCH).mean(-1)

    def touched(mask):
        return blocks(mask[:n, :n], PATCH).max(-1) > 0

    insitu = {}
    for name in (*INSITU_ROUGH, INSITU_BLOCKS, INSITU_RIPPLES):
        for ext in ("shp", "shx", "dbf", "prj"):
            if fetch(INSITU_FILE.format(name=quote(f"{name}.{ext}")), RAW / "crumpler2023" / f"{name}.{ext}") is None:
                sys.exit(f"in situ map file missing: {name}.{ext}")
        insitu[name] = list(gpd.read_file(RAW / "crumpler2023" / f"{name}.shp").geometry)
    if fetch(DRIVE_PATH_URL, RAW / "M20_traverse.json") is None:
        sys.exit("drive path download failed")
    drive = list(gpd.read_file(RAW / "M20_traverse.json").geometry)
    g = stack_map()
    slope2 = np.load(PROC / "slope2.npy", mmap_mode="r")
    sp90 = np.percentile(blocks(np.asarray(slope2[:n, :n]), PATCH), 90, axis=-1)

    stack_rough = list(rocky_units(g).geometry) + list(g[g.SurfUName.eq("Aeolian bedforms, large")].geometry)
    rough = frac(np.maximum(fine_raster(stack_rough, M_PER_DEG),
                            fine_raster([x for k in INSITU_ROUGH for x in insitu[k]], 1)))
    rules = (
        (touched(fine_raster(drive, 1, all_touched=True)), 0, "perseverance_drove"),
        (frac(fine_raster(g[g.SurfUName.eq("Undifferentiated smooth")].geometry, M_PER_DEG)) >= 0.5, 0, "smooth_unit"),
        (rough >= 0.5, 1, "rough_unit"),
        (touched(fine_raster(insitu[INSITU_BLOCKS], 1, all_touched=True)), 1, "ejecta_block"),
        (frac(fine_raster(insitu[INSITU_RIPPLES], 1)) >= 0.5, 2, "ripples"),
        (sp90 >= 20, 2, "slope_p90_ge_20"),
    )
    label = np.full((nb, nb), -1, np.int8)
    why = np.full((nb, nb), "", dtype=object)
    for cond, lab, name in rules:
        label[cond], why[cond] = lab, name

    ii, jj = np.nonzero(label >= 0)
    NASA_LABELS_CSV.parent.mkdir(parents=True, exist_ok=True)
    with open(NASA_LABELS_CSV, "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["id", "row", "col", "label", "rule"])
        w.writerows((k, i * PATCH + PATCH // 2, j * PATCH + PATCH // 2, int(label[i, j]), why[i, j])
                    for k, (i, j) in enumerate(zip(ii, jj)))
    counts = {name: int((why == name).sum()) for _, _, name in rules}
    print(f"labelled {len(ii)}/{nb * nb} patches; per class {np.bincount(label[label >= 0], minlength=3).tolist()}; "
          f"final rule counts {counts}; wrote {NASA_LABELS_CSV}")


def patch_features(slope2, rough2, ortho1, geo, r0, c0):
    s = np.asarray(slope2[r0:r0 + PATCH, c0:c0 + PATCH])
    ro = np.asarray(rough2[r0:r0 + PATCH, c0:c0 + PATCH])
    o = ortho_crop(ortho1, r0, c0).astype(np.float32)
    gy, gx = np.gradient(o)
    k = int(CELL_M / FINE_M)
    g = geo[r0 // k:(r0 + PATCH) // k + 1, c0 // k:(c0 + PATCH) // k + 1]
    onehot = np.zeros(N_GEO)
    onehot[min(int(np.bincount(g.ravel()).argmax()), N_GEO - 1)] = 1
    return np.concatenate([[s.mean(), np.percentile(s, 90), s.max(), ro.mean(), np.percentile(ro, 90),
                            o.mean(), o.std(), np.percentile(o, 10), np.percentile(o, 90),
                            np.hypot(gx, gy).mean()], onehot])


def cmd_hazard(_):
    from sklearn.metrics import confusion_matrix
    from sklearn.neural_network import MLPClassifier
    from sklearn.pipeline import make_pipeline
    from sklearn.preprocessing import StandardScaler

    team = labels_filled() >= 150   # team labels win once there are enough; else the `autolabel` rule labels
    src = LABELS_CSV if team else NASA_LABELS_CSV
    rows = []
    if src.exists():
        with open(src) as f:
            rows = [r for r in csv.DictReader(f) if r["label"].strip() in ("0", "1", "2")]
    y = np.array([int(r["label"]) for r in rows], dtype=int)
    if len(rows) < 150 or np.bincount(y, minlength=3).min() < 10:
        sys.exit(f"need >=150 labels and >=10 per class: label {LABELS_CSV.name} by hand or run `autolabel`")
    if not team:   # rule labels are dominated by the big mapped units: same number per class
        rng = np.random.default_rng(0)
        cap = min(int(np.bincount(y, minlength=3).min()), 2000)
        keep = np.sort(np.concatenate([rng.permutation(np.flatnonzero(y == c))[:cap] for c in range(3)]))
        rows, y = [rows[i] for i in keep], y[keep]
    print(f"training on {len(rows)} {'team' if team else 'NASA-rule'} labels from {src.name}; "
          f"per class {np.bincount(y, minlength=3).tolist()}")

    slope2, rough2, ortho1, geo = load_fine()
    X = np.array([patch_features(slope2, rough2, ortho1, geo, int(r["row"]) - PATCH // 2, int(r["col"]) - PATCH // 2)
                  for r in rows])

    def model():
        return make_pipeline(StandardScaler(),
                             MLPClassifier(hidden_layer_sizes=(32, 16), max_iter=2000, random_state=0))

    # Spatial hold-out: whole 1 km tiles, so neighbouring (near-duplicate) patches never straddle the split.
    tile_of = np.array([int(int(r["row"]) * FINE_M // TILE_M) * NT + int(int(r["col"]) * FINE_M // TILE_M)
                        for r in rows])
    test = np.isin(tile_of, np.random.default_rng(1).choice(NT * NT, NT * NT // 5, replace=False))
    clf = model().fit(X[~test], y[~test])
    pred = clf.predict(X[test])
    acc = float((pred == y[test]).mean())
    cm = confusion_matrix(y[test], pred, labels=[0, 1, 2])
    print(f"spatial hold-out accuracy {acc:.3f} (n={int(test.sum())}, 20 of 100 tiles)")
    print("confusion matrix (rows true 0/1/2, cols predicted):")
    print(cm)
    clf = model().fit(X, y)

    # Non-overlapping 48 m windows over the 2 m grid, then nearest-upsampled to the 10 m planning grid.
    nb = FN // PATCH
    P = np.zeros((nb, nb, 3))
    for i in range(nb):
        feats = np.array([patch_features(slope2, rough2, ortho1, geo, i * PATCH, j * PATCH) for j in range(nb)])
        P[i] = clf.predict_proba(feats)
    haz_w = np.where(P.argmax(-1) == 2, 255, np.round(254 * (0.5 * P[..., 1] + P[..., 2])))
    k = int(CELL_M / FINE_M)
    ri = np.minimum(np.arange(GRID) * k // PATCH, nb - 1)
    haz = haz_w[np.ix_(ri, ri)]
    save_u8("hazard.png", haz)

    meta = load_meta()
    meta["hazard_model"] = {"type": "MLPClassifier", "labels": "team" if team else "nasa_rules",
                            "n_labels": len(rows), "holdout_accuracy": round(acc, 3),
                            "holdout": "spatial: 20 of 100 1 km tiles", "holdout_confusion": cm.tolist()}
    set_credit(meta, "hazard", (
        "Team-labelled HiRISE patches" if team else
        "Patch labels from NASA data by fixed rules (written with SI, see docs/SI_USAGE.md): slope >= 20° (NASA "
        "EVA limit), Mars 2020 photogeologic and in situ maps (Stack et al. 2020; Crumpler et al. 2023, CC BY 4.0), "
        "Perseverance drive path (NASA/JPL-Caltech)") +
        "; MLP classifier on HiRISE DTM slope/roughness and orthomosaic texture (relative hazard, not a rock count)")
    save_meta(meta)
    print(f"hazard.png written; impassable cells {(haz == 255).mean() * 100:.1f}%")


def cmd_radiation(_):
    h2 = np.load(PROC / "height2.npy", mmap_mode="r")
    k = int(CELL_M / FINE_M)
    hp = blocks(np.asarray(h2), k).mean(-1)   # 10 m cells
    n = hp.shape[0]
    rr, cc = np.mgrid[0:n, 0:n]
    sin_sum = np.zeros((n, n))
    steps = int(1000 / CELL_M)   # horizon search out to 1000 m
    for a in np.arange(16) * (2 * np.pi / 16):
        max_tan = np.zeros((n, n))   # gamma clipped at >= 0
        for s in range(1, steps + 1):
            r = np.round(rr + s * np.sin(a)).astype(int)
            c = np.round(cc + s * np.cos(a)).astype(int)
            ok = (r >= 0) & (r < n) & (c >= 0) & (c < n)
            if not ok.any():
                break
            t = np.full((n, n), -np.inf)
            t[ok] = (hp[r[ok], c[ok]] - hp[ok]) / (s * CELL_M)
            max_tan = np.maximum(max_tan, t)
        sin_sum += np.sin(np.arctan(max_tan))
    svf = 1 - sin_sum / 16
    save_u8("radiation.png", 255 * np.clip(svf, 0, 1))
    meta = load_meta()
    meta["radiation"] = {"type": "sky-view factor", "min": float(svf.min()), "max": float(svf.max())}
    set_credit(meta, "radiation", "Derived from HiRISE DTM; relative GCR/SPE exposure proxy, not a dose")
    save_meta(meta)
    print(f"radiation.png written; SVF {svf.min():.3f}..{svf.max():.3f}")


def themis_index(bb):
    day = np.asarray(trek_mosaic(THEMIS_DAY, 9, bb, "themis_day"), dtype=np.float32)
    night = np.asarray(trek_mosaic(THEMIS_NIGHT, 9, bb, "themis_night"), dtype=np.float32)

    def norm(x):
        return (x - x.min()) / (x.max() - x.min()) if x.max() > x.min() else np.zeros_like(x)

    idx = norm(day) - norm(night)
    print(f"THEMIS crop {day.shape}")
    if idx.max() == idx.min():
        return np.full((GRID, GRID), 128.0), True
    idx = (idx - idx.min()) / (idx.max() - idx.min()) * 255
    return np.clip(resize_f(idx, (GRID, GRID), Image.BILINEAR), 0, 255), False


def insight_sols():
    r = _session.get(INSIGHT_URL, params={"api_key": os.environ.get("NASA_API_KEY", "DEMO_KEY"),
                                          "feedtype": "json", "ver": "1.0"}, timeout=60)
    r.raise_for_status()
    d = r.json()
    sols = []
    for s in d.get("sol_keys", []):
        e = d.get(s, {})
        if "AT" not in e:
            continue
        sols.append({"sol": int(s), "min_c": e["AT"]["mn"], "max_c": e["AT"]["mx"],
                     "swing_c": round(e["AT"]["mx"] - e["AT"]["mn"], 3),
                     "pressure_pa": e.get("PRE", {}).get("av"), "wind_ms": e.get("HWS", {}).get("av"),
                     "first_utc": e.get("First_UTC")})
    return sols


def lst_colormap():
    xml = fetch(GIBS_CMAP, RAW / "gibs/MODIS_Land_Surface_Temp.xml").decode()
    lut = {}
    for m in re.finditer(r"<ColorMapEntry\b([^>]*)/?>", xml):
        attrs = dict(re.findall(r'(\w+)="([^"]*)"', m.group(1)))
        if attrs.get("transparent") == "true" or "value" not in attrs:
            continue
        nums = [float(v) for v in re.findall(r"-?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?", attrs["value"])]
        nums = [v for v in nums if math.isfinite(v)]
        if not nums:
            continue
        lut[tuple(int(v) for v in attrs["rgb"].split(","))] = sum(nums) / len(nums)
    if not lut:
        sys.exit("could not parse MODIS LST colormap")
    return lut


def earth_days(lut, sat):
    z, row, col, px, py = gibs_index(EARTH_LAT, EARTH_LON)
    days = []
    for i in range(30):
        date = (dt.date(2024, 1, 1) + dt.timedelta(days=i)).isoformat()
        vals = {}
        for part in ("Day", "Night"):
            layer = f"MODIS_{sat}_Land_Surface_Temp_{part}"
            data = fetch(GIBS_TILE.format(layer=layer, date=date, z=z, row=row, col=col),
                         RAW / f"gibs/{layer}_{date}.png")
            if data is None:
                break
            r, g, b, a = Image.open(io.BytesIO(data)).convert("RGBA").getpixel((px, py))
            if a == 0 or (r, g, b) not in lut:
                break
            vals[part] = lut[(r, g, b)] - 273.15
        if len(vals) == 2:
            days.append({"date": date, "day_c": round(vals["Day"], 2), "night_c": round(vals["Night"], 2),
                         "swing_c": round(vals["Day"] - vals["Night"], 2)})
    return days


def cmd_thermal(_):
    thermal, uniform = themis_index(bbox())
    save_u8("thermal.png", thermal)
    meta = load_meta()
    meta["thermal"] = {"type": "THEMIS day-night relative swing", "uniform": uniform}
    set_credit(meta, "thermal", "THEMIS day/night IR controlled mosaics 100 m (NASA/JPL/ASU) via JPL Mars Trek; "
                                "relative day-night swing index, not a temperature")
    save_meta(meta)

    sols = insight_sols()
    if not sols:
        sys.exit("InSight API returned no sols with air temperature")
    lut = lst_colormap()
    days = earth_days(lut, "Terra")
    if len(days) < 5:
        print("fewer than 5 Terra samples; trying Aqua")
        days = earth_days(lut, "Aqua")
    if len(days) < 5:
        sys.exit("not enough MODIS LST samples")

    pres = [s["pressure_pa"] for s in sols if s["pressure_pa"] is not None]
    cond = {"mars": {"source": "NASA InSight APSS/TWINS via api.nasa.gov (Elysium Planitia, 2020)", "sols": sols,
                     "mean_swing_c": round(float(np.mean([s["swing_c"] for s in sols])), 2),
                     "max_swing_c": round(float(max(s["swing_c"] for s in sols)), 2),
                     "mean_pressure_pa": round(float(np.mean(pres)), 2) if pres else None},
            "earth": {"source": "MODIS Terra/Aqua LST via NASA GIBS/Worldview", "name": "Atacama Desert",
                      "lat": EARTH_LAT, "lon": EARTH_LON, "days": days,
                      "mean_swing_c": round(float(np.mean([d["swing_c"] for d in days])), 2)},
            "earth_pressure_pa": 101325,
            "generated_at": dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds")}
    WEB.mkdir(parents=True, exist_ok=True)
    (WEB / "conditions.json").write_text(json.dumps(cond, indent=1) + "\n")
    print(f"thermal.png written (uniform={uniform}); conditions.json: {len(sols)} InSight sols, "
          f"{len(days)} Earth days")


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    for name in ("selftest", "terrain", "color", "ortho2", "rocks", "traverse", "labels", "autolabel", "hazard",
                 "radiation", "thermal"):
        sub.add_parser(name).set_defaults(fn=globals()[f"cmd_{name}"])
    args = ap.parse_args()
    args.fn(args)


if __name__ == "__main__":
    main()
