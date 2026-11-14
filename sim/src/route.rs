//! EVA route planner: A* over the 10 m site grid that respects a load-adjusted slope limit.
//! Core functions are plain Rust (no Bevy types) and unit-tested below.

use bevy::prelude::*;
use pathfinding::prelude::astar;

use crate::data::{CELL_M, N, Site, VScale};
use crate::terrain::Surface;

pub const G_MARS: f32 = 3.71;
pub const G_EARTH: f32 = 9.81;
// shortcut: planning estimate, no NASA source; replace if team finds one
pub const BASE_SPEED_MPS: f32 = 1.0;
// shortcut: planning estimate, no NASA source; replace if team finds one
const SLOPE_SLOWDOWN: f32 = 0.6;
/// Planner cell size = site grid cell.
pub const STEP_M: f32 = CELL_M;
const DIAG_M: f32 = STEP_M * std::f32::consts::SQRT_2;
const PROFILE_POINTS: usize = 100;

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, Resource)]
#[serde(default)]
pub struct RouteParams {
    pub body_kg: f32,
    pub suit_kg: f32,
    pub payload_kg: f32,
    /// 20°: NASA's suited-walking slope limit for Artemis (EVA-EXP-0042 Rev B; Artemis III site rule "< 20°").
    /// NASA has no Mars-specific value; the load factor below can only lower it.
    pub s_max_base_deg: f32,
    /// Cells whose relief residual (DTM roughness, m) exceeds this are impassable. 0.30 m: suited step height
    /// NASA found acceptable (ICES-2025-294, 30.5 cm), below Perseverance's 40 cm obstacle limit.
    pub rough_max_m: f32,
    pub w_slope: f32,
    pub w_hazard: f32,
    pub w_thermal: f32,
    pub w_rad: f32,
    /// DTM roughness (relief residual) weight.
    pub w_rough: f32,
    /// HiRISE optical texture (local contrast) weight.
    pub w_texture: f32,
    /// Mapped rock cover (talus / exposed blocky bedrock, Stack et al. 2020) weight.
    pub w_rocks: f32,
}

impl Default for RouteParams {
    fn default() -> Self {
        Self {
            body_kg: 80.0,
            suit_kg: 104.0,
            payload_kg: 20.0,
            s_max_base_deg: 20.0,
            rough_max_m: 0.30,
            w_slope: 3.0,
            w_hazard: 5.0,
            w_thermal: 0.5,
            w_rad: 0.5,
            w_rough: 2.0,
            w_texture: 1.0,
            w_rocks: 3.0,
        }
    }
}

/// Loaded weight on Mars as a fraction of the walker's body weight on Earth.
pub fn weight_factor(p: &RouteParams) -> f32 {
    (p.body_kg + p.suit_kg + p.payload_kg) * G_MARS / (p.body_kg * G_EARTH)
}

/// Allowed slope shrinks when the load on Mars exceeds the walker's Earth body weight.
pub fn s_max_eff(p: &RouteParams) -> f32 {
    p.s_max_base_deg / weight_factor(p).max(1.0)
}

pub struct CostGrid {
    pub n: usize,
    slope: Vec<f32>,
    rough: Vec<f32>,
    texture: Vec<u8>,
    rocks: Vec<u8>,
    hazard: Vec<u8>,
    thermal: Vec<u8>,
    rad: Vec<u8>,
}

impl CostGrid {
    /// Slope is already the p90 of the 2 m slopes in each 10 m cell (conservative), so no pooling here.
    pub fn from_site(site: &Site) -> Self {
        CostGrid {
            n: N,
            slope: site.slope_deg.clone(),
            rough: site.rough_m.clone(),
            texture: site.texture.clone(),
            rocks: site.rocks.clone(),
            hazard: site.hazard.clone(),
            thermal: site.thermal.clone(),
            rad: site.radiation.clone(),
        }
    }

    /// Cost multiplier for entering cell `i`; `None` = impassable.
    pub fn multiplier(&self, i: usize, p: &RouteParams) -> Option<f32> {
        let s_eff = s_max_eff(p);
        // NASA's EVA site rule is "slope less than 20°": a cell exactly at the limit is blocked.
        if self.slope[i] >= s_eff || self.rough[i] > p.rough_max_m || self.hazard[i] == 255 {
            return None;
        }
        Some(
            1.0 + p.w_slope * (self.slope[i] / s_eff).powi(2)
                + p.w_rough * self.rough[i] / p.rough_max_m.max(1e-3)
                + p.w_texture * self.texture[i] as f32 / 255.0
                + p.w_rocks * self.rocks[i] as f32 / 255.0
                + p.w_hazard * self.hazard[i] as f32 / 254.0
                + p.w_thermal * self.thermal[i] as f32 / 255.0
                + p.w_rad * self.rad[i] as f32 / 255.0,
        )
    }

    pub fn slope(&self, i: usize) -> f32 {
        self.slope[i]
    }

    /// World x/z (metres) of planner cell centre (row, col).
    pub fn center(&self, (r, c): (usize, usize)) -> (f32, f32) {
        let half = self.n as f32 * STEP_M / 2.0;
        (c as f32 * STEP_M - half + STEP_M / 2.0, r as f32 * STEP_M - half + STEP_M / 2.0)
    }

    pub fn cell_of(&self, x: f32, z: f32) -> Option<(usize, usize)> {
        let half = self.n as f32 * STEP_M / 2.0;
        let (c, r) = ((x + half) / STEP_M, (z + half) / STEP_M);
        let n = self.n as f32;
        (c >= 0.0 && r >= 0.0 && c < n && r < n).then_some((r as usize, c as usize))
    }
}

#[derive(Clone, Debug)]
pub struct RouteResult {
    pub cells: Vec<(usize, usize)>,
    pub length_m: f32,
    pub eta_min: f32,
    pub max_slope_deg: f32,
    pub mean_hazard: f32,
}

pub fn plan(grid: &CostGrid, start: (usize, usize), goal: (usize, usize), p: &RouteParams) -> Option<RouteResult> {
    let n = grid.n;
    let idx = |(r, c): (usize, usize)| r * n + c;
    grid.multiplier(idx(start), p)?;
    grid.multiplier(idx(goal), p)?;

    let (cells, _) = astar(
        &start,
        |&(r, c)| {
            let mut out = Vec::with_capacity(8);
            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                    if (dr, dc) == (0, 0) || nr < 0 || nc < 0 || nr >= n as i32 || nc >= n as i32 {
                        continue;
                    }
                    let next = (nr as usize, nc as usize);
                    let diag = dr != 0 && dc != 0;
                    // A diagonal step may not squeeze between blocked cells at the corner.
                    let open = |cell: (usize, usize)| grid.multiplier(idx(cell), p).is_some();
                    if diag && !(open((nr as usize, c)) && open((r, nc as usize))) {
                        continue;
                    }
                    if let Some(m) = grid.multiplier(idx(next), p) {
                        let step = if diag { DIAG_M } else { STEP_M };
                        out.push((next, (step * m * 100.0).round() as u32));
                    }
                }
            }
            out
        },
        |&(r, c)| {
            // Octile distance in cm; floor keeps it admissible (every multiplier is >= 1).
            let (dr, dc) = (r.abs_diff(goal.0) as f32, c.abs_diff(goal.1) as f32);
            ((STEP_M * (dr.max(dc) - dr.min(dc)) + DIAG_M * dr.min(dc)) * 100.0).floor() as u32
        },
        |&cell| cell == goal,
    )?;

    let s_eff = s_max_eff(p);
    let speed = BASE_SPEED_MPS / weight_factor(p).max(1.0);
    let (mut length_m, mut eta_s) = (0.0, 0.0);
    for w in cells.windows(2) {
        let diag = w[0].0 != w[1].0 && w[0].1 != w[1].1;
        let step = if diag { DIAG_M } else { STEP_M };
        length_m += step;
        eta_s += step / (speed * (1.0 - SLOPE_SLOWDOWN * grid.slope(idx(w[1])) / s_eff));
    }
    let max_slope_deg = cells.iter().map(|&c| grid.slope(idx(c))).fold(0.0, f32::max);
    let mean_hazard =
        cells.iter().map(|&c| (grid.hazard[idx(c)] as f32 / 254.0).min(1.0)).sum::<f32>() / cells.len() as f32;
    Some(RouteResult { cells, length_m, eta_min: eta_s / 60.0, max_slope_deg, mean_hazard })
}

/// Total climb and descent (m) along an elevation sequence.
pub fn ascent_descent(h: &[f32]) -> (f32, f32) {
    h.windows(2).fold((0.0, 0.0), |(up, down), w| {
        let d = w[1] - w[0];
        (up + d.max(0.0), down + (-d).max(0.0))
    })
}

/// Clicked endpoints in world space (unexaggerated heights).
#[derive(Resource, Default)]
pub struct Endpoints {
    pub start: Option<Vec3>,
    pub end: Option<Vec3>,
}

/// Latest planned route; `result` is `None` when there are no endpoints or no safe path.
#[derive(Resource, Default)]
pub struct Route {
    pub result: Option<RouteResult>,
    /// World-space polyline of the route, unexaggerated terrain heights.
    pub polyline: Vec<Vec3>,
}

#[derive(Resource)]
pub struct Grid(pub CostGrid);

pub struct RoutePlugin;

impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        let grid = CostGrid::from_site(app.world().resource::<Site>());
        app.insert_resource(Grid(grid))
            .init_resource::<RouteParams>()
            .init_resource::<Endpoints>()
            .init_resource::<Route>()
            .add_systems(Startup, gizmo_style)
            .add_systems(Update, (replan, draw_route).chain());
    }
}

fn gizmo_style(mut store: ResMut<GizmoConfigStore>) {
    let (cfg, _) = store.config_mut::<DefaultGizmoConfigGroup>();
    cfg.line.width = 4.0;
    cfg.depth_bias = -1.0; // map view: lines on top of the terrain; camera::enter_mode changes it per view
}

pub fn replan(
    ends: Res<Endpoints>,
    params: Res<RouteParams>,
    grid: Res<Grid>,
    site: Res<Site>,
    mut route: ResMut<Route>,
) {
    if !ends.is_changed() && !params.is_changed() {
        return;
    }
    let (Some(a), Some(b)) = (ends.start, ends.end) else {
        if route.result.is_some() || !route.polyline.is_empty() {
            *route = Route::default();
        }
        return;
    };
    let g = &grid.0;
    let result = match (g.cell_of(a.x, a.z), g.cell_of(b.x, b.z)) {
        (Some(s), Some(e)) => plan(g, s, e, &params),
        _ => None,
    };
    let polyline: Vec<Vec3> = result
        .as_ref()
        .map(|r| {
            r.cells
                .iter()
                .map(|&cell| {
                    let (x, z) = g.center(cell);
                    Vec3::new(x, site.height_at(x, z), z)
                })
                .collect()
        })
        .unwrap_or_default();
    let (sa, sb) = (site.latlon_of(a.x, a.z), site.latlon_of(b.x, b.z));
    let wf = weight_factor(&params);
    let s_eff = s_max_eff(&params);
    let json = match &result {
        Some(r) => {
            let h: Vec<f32> = polyline.iter().map(|p| p.y + site.h_mid()).collect();
            let (ascent, descent) = ascent_descent(&h);
            // Elevation profile: ≤PROFILE_POINTS samples of (distance m, absolute elevation m).
            let mut d = 0.0;
            let mut dist = vec![0.0f32];
            for w in polyline.windows(2) {
                d += Vec2::new(w[1].x - w[0].x, w[1].z - w[0].z).length();
                dist.push(d);
            }
            let stride = h.len().div_ceil(PROFILE_POINTS).max(1);
            let mut profile: Vec<_> = (0..h.len())
                .step_by(stride)
                .map(|i| serde_json::json!({"d": dist[i].round(), "h": (h[i] * 10.0).round() / 10.0}))
                .collect();
            if (h.len() - 1) % stride != 0 {
                let i = h.len() - 1;
                profile.push(serde_json::json!({"d": dist[i].round(), "h": (h[i] * 10.0).round() / 10.0}));
            }
            serde_json::json!({
                "status": "ok", "length_m": r.length_m, "eta_min": r.eta_min, "max_slope_deg": r.max_slope_deg,
                "mean_hazard": r.mean_hazard, "weight_factor": wf, "s_max_eff_deg": s_eff,
                "ascent_m": ascent, "descent_m": descent, "profile": profile,
                "start": {"lat": sa.0, "lon": sa.1}, "end": {"lat": sb.0, "lon": sb.1},
            })
        }
        None => serde_json::json!({
            "status": "no_path", "length_m": 0, "eta_min": 0, "max_slope_deg": 0, "mean_hazard": 0,
            "weight_factor": wf, "s_max_eff_deg": s_eff, "ascent_m": 0, "descent_m": 0, "profile": [],
            "start": {"lat": sa.0, "lon": sa.1}, "end": {"lat": sb.0, "lon": sb.1},
        }),
    };
    crate::bridge::emit("mars-route", json.to_string());
    *route = Route { result, polyline };
}

fn draw_route(route: Res<Route>, surface: Res<Surface>, vs: Res<VScale>, mut gizmos: Gizmos) {
    if route.polyline.len() > 1 {
        let line = surface.drape(route.polyline.iter().map(|p| p.xz()), *vs, 0.15, 4.0);
        gizmos.linestrip(line, Color::srgb_u8(0x38, 0xe0, 0xc8));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(n: usize, slope: f32) -> CostGrid {
        let z = vec![0u8; n * n];
        CostGrid {
            n,
            slope: vec![slope; n * n],
            rough: vec![0.0; n * n],
            texture: z.clone(),
            rocks: z.clone(),
            hazard: z.clone(),
            thermal: z.clone(),
            rad: z,
        }
    }

    #[test]
    fn flat_grid_gives_straight_line() {
        let g = grid(20, 0.0);
        let r = plan(&g, (0, 0), (0, 19), &RouteParams::default()).expect("path");
        assert_eq!(r.cells, (0..20).map(|c| (0, c)).collect::<Vec<_>>());
        assert!((r.length_m - 19.0 * STEP_M).abs() < 1e-3, "length {}", r.length_m);
    }

    #[test]
    fn hazard_wall_forces_path_through_gap() {
        let mut g = grid(20, 0.0);
        for r in 0..20 {
            if r != 15 {
                g.hazard[r * 20 + 10] = 255;
            }
        }
        let r = plan(&g, (0, 0), (0, 19), &RouteParams::default()).expect("path");
        assert!(r.cells.contains(&(15, 10)), "path {:?}", r.cells);
    }

    #[test]
    fn full_wall_has_no_path() {
        let mut g = grid(20, 0.0);
        for r in 0..20 {
            g.hazard[r * 20 + 10] = 255;
        }
        assert!(plan(&g, (0, 0), (0, 19), &RouteParams::default()).is_none());
    }

    #[test]
    fn diagonal_wall_is_not_crossed_at_its_corners() {
        // Wall of blocked cells along the anti-diagonal: only diagonal moves between them would cross it.
        let mut g = grid(20, 0.0);
        for k in 0..20 {
            g.hazard[k * 20 + (19 - k)] = 255;
        }
        assert!(plan(&g, (0, 0), (19, 19), &RouteParams::default()).is_none());
    }

    #[test]
    fn payload_lowers_allowed_slope() {
        let g = grid(20, 15.0);
        let p = RouteParams::default();
        assert!((weight_factor(&p) - 0.9644).abs() < 1e-3);
        assert_eq!(s_max_eff(&p), 20.0);
        assert!(plan(&g, (0, 0), (0, 19), &p).is_some());

        let heavy = RouteParams { payload_kg: 120.0, ..p };
        assert!((weight_factor(&heavy) - 1.437).abs() < 1e-3);
        assert!((s_max_eff(&heavy) - 13.92).abs() < 0.01);
        assert!(plan(&g, (0, 0), (0, 19), &heavy).is_none());
    }

    #[test]
    fn takes_cheap_detour_around_hot_band() {
        let mut g = grid(20, 0.0);
        for r in 8..=12 {
            g.thermal[r * 20 + 10] = 255;
        }
        let p = RouteParams { w_thermal: 10.0, ..RouteParams::default() };
        let r = plan(&g, (10, 0), (10, 19), &p).expect("path");
        assert!(r.cells.iter().all(|&(row, col)| col != 10 || !(8..=12).contains(&row)), "path {:?}", r.cells);
        // Detour costs < 3 extra cells of walking; going straight through would cost 10x per hot cell.
        assert!(r.length_m < 19.0 * STEP_M + 3.0 * STEP_M, "length {}", r.length_m);
    }

    #[test]
    fn avoids_rough_band_when_weighted() {
        let mut g = grid(20, 0.0);
        for r in 8..=12 {
            g.rough[r * 20 + 10] = 0.25; // under the 0.30 m limit: passable, only costly
        }
        let p = RouteParams { w_rough: 10.0, ..RouteParams::default() };
        let r = plan(&g, (10, 0), (10, 19), &p).expect("path");
        assert!(r.cells.iter().all(|&(row, col)| col != 10 || !(8..=12).contains(&row)), "path {:?}", r.cells);
    }

    #[test]
    fn detours_around_mapped_rocks() {
        let mut g = grid(20, 0.0);
        for r in 8..=12 {
            g.rocks[r * 20 + 10] = 255;
        }
        let r = plan(&g, (10, 0), (10, 19), &RouteParams::default()).expect("path");
        assert!(r.cells.iter().all(|&(row, col)| col != 10 || !(8..=12).contains(&row)), "path {:?}", r.cells);
    }

    #[test]
    fn rough_wall_over_the_limit_blocks_until_the_limit_is_raised() {
        let mut g = grid(20, 0.0);
        for r in 0..20 {
            g.rough[r * 20 + 10] = 0.35;
        }
        let p = RouteParams::default();
        assert!(plan(&g, (0, 0), (0, 19), &p).is_none());
        assert!(plan(&g, (0, 0), (0, 19), &RouteParams { rough_max_m: 0.4, ..p }).is_some());
    }

    #[test]
    fn ascent_descent_sums_climbs_and_drops() {
        assert_eq!(ascent_descent(&[0.0, 3.0, 1.0, 4.0]), (6.0, 2.0));
    }
}
