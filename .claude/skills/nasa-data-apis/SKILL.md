---
name: nasa-data-apis
description: Catálogo práctico de APIs y fuentes de datos abiertas de NASA y JPL (api.nasa.gov, Earthdata/CMR, GIBS, FIRMS, POWER, EONET, DONKI, Horizons, SBDB, Exoplanet Archive, Image Library, OSDR, Trek, IRSA) con endpoints, autenticación y ejemplos. Úsala siempre que haya que obtener datos de NASA, elegir una fuente, depurar una llamada a una API de NASA o decidir si algo se consume en vivo o se pre-procesa.
---

# APIs y datos de NASA

**Regla 0:** primero revisa qué recursos lista el desafío en `docs/challenges/<desafío>.md`. El jurado valora que uses *esos* datos. Esta skill es para conseguirlos de forma programática.

**Regla 1:** las keys van en el servidor (Route Handler de Next.js o script). Nunca uses `NEXT_PUBLIC_` para una key. El patrón está en `src/app/api/nasa/apod/route.ts`.

**Regla 2:** si el dato pesa más de unos MB o necesita procesarse (HDF5, NetCDF, GeoTIFF, FITS), **pre-procésalo una vez** con Python y sirve un resultado chico: JSON, GeoJSON, CSV, PNG/COG o PMTiles, en `public/data/`, Supabase Storage o una tabla. No proceses archivos científicos dentro de una función serverless.

## Autenticación
| Servicio | Cómo | Dónde |
|---|---|---|
| api.nasa.gov | `api_key=` (DEMO_KEY: 30 req/h y 50/día por IP; key gratis: 1000 req/h) | https://api.nasa.gov/ |
| Earthdata (CMR, DAACs, ASF, LP DAAC, GES DISC, PO.DAAC) | Earthdata Login → token Bearer o `.netrc` | https://urs.earthdata.nasa.gov/ |
| FIRMS | `MAP_KEY` gratis por email | https://firms.modaps.eosdis.nasa.gov/api/map_key/ |
| GIBS, EONET, EPIC, POWER, Horizons, SBDB, Exoplanet Archive, Image Library | sin auth | — |

## Endpoints probados por categoría

### Imágenes y contexto general
- **APOD:** `GET https://api.nasa.gov/planetary/apod?api_key=KEY&date=2026-11-14`
- **NASA Image and Video Library:** `GET https://images-api.nasa.gov/search?q=apollo%2011&media_type=image` (sin key; ideal para el desafío 1).
- **EPIC (Tierra completa desde DSCOVR):** `GET https://epic.gsfc.nasa.gov/api/natural` → imágenes en `https://epic.gsfc.nasa.gov/archive/natural/YYYY/MM/DD/png/{image}.png`

### Tierra
- **GIBS (teselas listas para mapas, sin auth):**
  `https://gibs.earthdata.nasa.gov/wmts/epsg3857/best/{Layer}/default/{YYYY-MM-DD}/GoogleMapsCompatible_Level9/{z}/{y}/{x}.jpg`
  Capas típicas: `MODIS_Terra_CorrectedReflectance_TrueColor`, `VIIRS_SNPP_CorrectedReflectance_TrueColor`. El nivel máximo (Level9, Level8…) depende de la capa: consulta el GetCapabilities en `https://gibs.earthdata.nasa.gov/wmts/epsg3857/best/wmts.cgi?SERVICE=WMTS&REQUEST=GetCapabilities`. Para elegir capas, explóralas en Worldview: https://worldview.earthdata.nasa.gov/
- **CMR (buscar cualquier colección o archivo de Earthdata):**
  - Colecciones: `GET https://cmr.earthdata.nasa.gov/search/collections.json?keyword=NISAR&page_size=20`
  - Archivos (granules): `GET https://cmr.earthdata.nasa.gov/search/granules.json?short_name=MOD14A1&temporal=2025-01-01T00:00:00Z,2025-01-31T23:59:59Z&bounding_box=-62,-28,-54,-19`
  - STAC: `https://cmr.earthdata.nasa.gov/stac`
- **FIRMS (focos de calor MODIS/VIIRS, desafío 8):**
  `GET https://firms.modaps.eosdis.nasa.gov/api/area/csv/{MAP_KEY}/{SOURCE}/{west,south,east,north}/{DAYS 1-10}/{YYYY-MM-DD}`
  SOURCE: `MODIS_NRT`, `MODIS_SP`, `VIIRS_SNPP_NRT`, `VIIRS_SNPP_SP`, `VIIRS_NOAA20_NRT`, `VIIRS_NOAA21_NRT`. `_SP` es el archivo histórico "standard". Para series largas descarga el archivo histórico por país desde https://firms.modaps.eosdis.nasa.gov/download/ en lugar de iterar la API.
- **NASA POWER (meteorología y solar por punto, ideal para agro):**
  `GET https://power.larc.nasa.gov/api/temporal/daily/point?parameters=T2M,PRECTOTCORR,ALLSKY_SFC_SW_DWN&community=AG&longitude=-57.6&latitude=-25.3&start=20250101&end=20251231&format=JSON`
- **EONET (eventos naturales: incendios, tormentas, volcanes):** `GET https://eonet.gsfc.nasa.gov/api/v3/events?status=open&category=wildfires`
- **AppEEARS** (recortes de MODIS/VIIRS/SMAP/Landsat por punto o área, sin lidiar con HDF): https://appeears.earthdatacloud.nasa.gov/ (requiere Earthdata Login; los pedidos tardan de minutos a horas, así que lánzalos temprano).

### Clima espacial y objetos cercanos
- **DONKI:** `GET https://api.nasa.gov/DONKI/FLR?startDate=2026-01-01&api_key=KEY` (también `CME`, `GST`, `SEP`).
- **NeoWs:** `GET https://api.nasa.gov/neo/rest/v1/feed?start_date=2026-11-14&api_key=KEY`
- **JPL SBDB / Close Approach:** `https://ssd-api.jpl.nasa.gov/sbdb.api?sstr=433` y `https://ssd-api.jpl.nasa.gov/cad.api?dist-max=0.05`

### Luna, Marte y cielo
- **JPL Horizons API (posiciones de Sol/Tierra vistos desde un sitio, desafío 11):** `https://ssd.jpl.nasa.gov/api/horizons.api` (ver la skill `planetary-data`).
- **Trek (Moon/Mars/Vesta) — mosaicos y WMTS:** https://trek.nasa.gov/
- **NASA Exoplanet Archive (TAP):** `https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=select+pl_name,disc_year+from+ps+where+default_flag=1&format=json`
- **IRSA (SPHEREx, WISE):** https://irsa.ipac.caltech.edu/ — acceso desde Python con `astroquery.ipac.irsa`.

### Biología y humanos en el espacio
- **OSDR (Open Science Data Repository, ex-GeneLab):** búsqueda `https://osdr.nasa.gov/osdr/data/search?term=bone&type=cgene`. Ver detalle en `docs/challenges/03-*.md` y `04-*.md`.

## Python (pre-proceso)
```bash
pip install earthaccess xarray rioxarray netCDF4 h5py pandas geopandas requests
```
```python
import earthaccess
earthaccess.login(strategy="environment")  # usa EARTHDATA_USERNAME/PASSWORD o EARTHDATA_TOKEN
results = earthaccess.search_data(short_name="MOD14A1", temporal=("2025-01-01", "2025-01-31"),
                                  bounding_box=(-62, -28, -54, -19))
files = earthaccess.download(results[:3], "./raw")
```

## Errores comunes
- `429` con DEMO_KEY: pide una key propia (llega al instante) y cachea con `next: { revalidate }`.
- HTML de login en lugar del archivo: falta el token de Earthdata, o la app del DAAC no está autorizada en tu perfil de Earthdata ("Applications > Authorized Apps").
- CORS desde el navegador: llama a la API desde un Route Handler propio.
- Fechas: CMR usa ISO 8601 con `Z` y POWER usa `YYYYMMDD`.
- **Verifica cada endpoint con `curl` antes de construir sobre él**: las APIs cambian y algunas (como Mars Rover Photos) fueron archivadas o migradas.
