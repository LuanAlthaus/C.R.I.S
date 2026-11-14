//! Site grids embedded at compile time from sim/assets/jezero (written by notebooks/jezero_pipeline.py).
//! World frame: x = east, z = south, y = up, metres, origin at the region centre.

use bevy::prelude::*;
use serde::Deserialize;

/// Planning grid side in cells (10 m cells over the 10 km region).
pub const N: usize = 1000;
pub const CELL_M: f32 = 10.0;
pub const HALF_M: f32 = 5000.0;
pub const SIZE_M: f32 = 2.0 * HALF_M;

#[derive(Deserialize, Clone, Debug)]
pub struct BBox {
    pub lat_min: f64,
    pub lat_max: f64,
    pub lon_min: f64,
    pub lon_max: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Meta {
    pub bbox: BBox,
    pub h_min: f32,
    pub h_max: f32,
    pub slope_scale_deg: f32,
    pub rough_scale_m: f32,
    /// Present once the radiation stage ran (sky-view factor range).
    #[serde(default)]
    pub radiation: Option<serde_json::Value>,
}

impl Meta {
    pub fn radiation_range(&self) -> Option<Range> {
        self.radiation.as_ref().and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Waypoint {
    pub sol: u32,
    pub x: f32,
    pub z: f32,
}

#[derive(Deserialize)]
struct Traverse {
    points: Vec<Waypoint>,
}

#[derive(Resource)]
pub struct Site {
    pub meta: Meta,
    /// Metres relative to `h_mid()`, row-major N×N, row 0 = north.
    pub height: Vec<f32>,
    pub slope_deg: Vec<f32>,
    pub rough_m: Vec<f32>,
    /// HiRISE optical local contrast, relative 0..255.
    pub texture: Vec<u8>,
    pub hazard: Vec<u8>,
    pub thermal: Vec<u8>,
    pub radiation: Vec<u8>,
    pub geology: Vec<u8>,
    /// Region-wide ortho (2048², ~5 m/px) for far terrain.
    pub ortho: image::GrayImage,
    /// Perseverance waypoints inside the region, in drive order.
    pub traverse: Vec<Waypoint>,
}

fn luma8(bytes: &[u8]) -> Vec<u8> {
    let img = image::load_from_memory(bytes).expect("embedded grid decodes").into_luma8();
    assert_eq!((img.width(), img.height()), (N as u32, N as u32), "grid must be {N}x{N}");
    img.into_raw()
}

impl Site {
    pub fn load() -> Self {
        let meta: Meta = serde_json::from_str(include_str!("../assets/jezero/meta.json")).expect("meta.json parses");
        let h16 = image::load_from_memory(include_bytes!("../assets/jezero/height.png"))
            .expect("height.png decodes")
            .into_luma16();
        let span = meta.h_max - meta.h_min;
        let height = h16.into_raw().iter().map(|&v| (v as f32 / 65535.0 - 0.5) * span).collect();
        let scaled = |bytes: &[u8], k: f32| luma8(bytes).into_iter().map(|v| v as f32 * k).collect();
        let traverse: Traverse =
            serde_json::from_str(include_str!("../assets/jezero/traverse.json")).expect("traverse.json parses");
        Site {
            height,
            slope_deg: scaled(include_bytes!("../assets/jezero/slope.png"), meta.slope_scale_deg),
            rough_m: scaled(include_bytes!("../assets/jezero/rough.png"), meta.rough_scale_m),
            texture: luma8(include_bytes!("../assets/jezero/texture.png")),
            hazard: luma8(include_bytes!("../assets/jezero/hazard.png")),
            thermal: luma8(include_bytes!("../assets/jezero/thermal.png")),
            radiation: luma8(include_bytes!("../assets/jezero/radiation.png")),
            geology: luma8(include_bytes!("../assets/jezero/geology.png")),
            ortho: image::load_from_memory(include_bytes!("../assets/jezero/ortho.jpg"))
                .expect("ortho.jpg decodes")
                .into_luma8(),
            traverse: traverse.points,
            meta,
        }
    }

    pub fn h_mid(&self) -> f32 {
        (self.meta.h_min + self.meta.h_max) / 2.0
    }

    /// Bilinear height (relative metres, unexaggerated) at world x/z; clamps outside the grid.
    pub fn height_at(&self, x: f32, z: f32) -> f32 {
        let fc = ((x + HALF_M) / CELL_M - 0.5).clamp(0.0, (N - 1) as f32);
        let fr = ((z + HALF_M) / CELL_M - 0.5).clamp(0.0, (N - 1) as f32);
        let (c0, r0) = (fc.floor() as usize, fr.floor() as usize);
        let (c1, r1) = ((c0 + 1).min(N - 1), (r0 + 1).min(N - 1));
        let (tx, tz) = (fc - c0 as f32, fr - r0 as f32);
        let h = |r: usize, c: usize| self.height[r * N + c];
        let top = h(r0, c0) * (1.0 - tx) + h(r0, c1) * tx;
        let bot = h(r1, c0) * (1.0 - tx) + h(r1, c1) * tx;
        top * (1.0 - tz) + bot * tz
    }

    pub fn cell_of(&self, x: f32, z: f32) -> Option<(usize, usize)> {
        let (c, r) = ((x + HALF_M) / CELL_M, (z + HALF_M) / CELL_M);
        (c >= 0.0 && r >= 0.0 && c < N as f32 && r < N as f32).then_some((r as usize, c as usize))
    }

    pub fn latlon_of(&self, x: f32, z: f32) -> (f64, f64) {
        let b = &self.meta.bbox;
        let lon = b.lon_min + (x as f64 + HALF_M as f64) / SIZE_M as f64 * (b.lon_max - b.lon_min);
        let lat = b.lat_max - (z as f64 + HALF_M as f64) / SIZE_M as f64 * (b.lat_max - b.lat_min);
        (lat, lon)
    }

    /// Nearest Perseverance waypoint within `max_m` of x/z: (sol, distance m).
    pub fn nearest_waypoint(&self, x: f32, z: f32, max_m: f32) -> Option<(u32, f32)> {
        self.traverse
            .iter()
            .map(|w| (w.sol, Vec2::new(w.x - x, w.z - z).length()))
            .filter(|&(_, d)| d <= max_m)
            .min_by(|a, b| a.1.total_cmp(&b.1))
    }
}

/// Vertical exaggeration applied to everything drawn on the terrain (1 = true relief).
#[derive(Resource, Clone, Copy, PartialEq, Debug)]
pub struct VScale(pub f32);

impl Default for VScale {
    fn default() -> Self {
        VScale(2.0)
    }
}

impl Site {
    /// Rendered ground height at x/z with vertical exaggeration.
    pub fn ground(&self, x: f32, z: f32, vs: VScale) -> f32 {
        self.height_at(x, z) * vs.0
    }
}

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Site::load()).init_resource::<VScale>();
    }
}
