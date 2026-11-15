//! Route planner: A* over the 10 m site grid that respects a load-adjusted slope limit. Two modes share it:
//! a suited walker (EVA, NASA limits) and the rover (an experimental simulation limit, see `Vehicle`).
//! Core functions are plain Rust (no Bevy types) and unit-tested below.

use bevy::prelude::*;
use pathfinding::prelude::astar;

use crate::data::{CELL_M, N, Site, VScale};
use crate::terrain::Surface;

pub const G_MARS: f32 = 3.71;
pub const G_EARTH: f32 = 9.81;
/// Suited walking on flat ground: 3.0 km/h, NASA's average EVA walking speed ("Apollo average of 2.9 km/h",
/// NASA/TM-2010-216138).
pub const BASE_SPEED_MPS: f32 = 3.0 / 3.6;
/// Perseverance's measured effective AutoNav rate, 98.0 m/h, stops to image and plan included (Verma et al. 2023,
/// IEEE Aerospace Conf., "Results from the First Year and a Half of Mars 2020 Robotic Operations", Table 1).
/// The 152 m/h top speed (NASA rover facts) is wheel speed, not a traverse rate.
pub const ROVER_SPEED_MPS: f32 = 98.0 / 3600.0;
/// Planner cell size = site grid cell.
pub const STEP_M: f32 = CELL_M;
const DIAG_M: f32 = STEP_M * std::f32::consts::SQRT_2;
const PROFILE_POINTS: usize = 100;

/// Who walks or drives the route: picks the slope limit, the load model, the speed and the hazard layer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Vehicle {
    /// Suited astronaut on foot: NASA EVA limits, hazard.png.
    #[default]
    Eva,
    /// The rover: `rover_s_max_deg`, hazard_phys.png (ground hazard without the EVA slope rule).
    Rover,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, Resource)]
#[serde(default)]
pub struct RouteParams {
    pub vehicle: Vehicle,
    pub body_kg: f32,
    pub suit_kg: f32,
    /// Load the walker carries (EVA mode), kg.
    pub payload_kg: f32,
    /// EVA: 20°, NASA's suited-walking slope limit for Artemis (EVA-EXP-0042 Rev B; Artemis III site rule "< 20°").
    /// NASA has no Mars-specific value; the load factor below can only lower it.
    pub s_max_base_deg: f32,
    /// Rover: 45°, an experimental simulation parameter chosen by the team. It is NOT a tested capability of
    /// Perseverance; the load factor below can only lower it.
    pub rover_s_max_deg: f32,
    /// Rover mass, kg: Perseverance 1,025 kg (NASA Mars 2020 rover facts).
    pub rover_kg: f32,
    /// Rover: extra load it carries at the full slope limit; more lowers the limit. Team assumption, no NASA source.
    pub rover_rated_payload_kg: f32,
    /// Load the rover carries (rover mode), kg.
    pub rover_payload_kg: f32,
    /// EVA: cells whose relief residual (DTM roughness, m) exceeds this are impassable. 0.30 m: suited step height
    /// NASA found acceptable (ICES-2025-294, 30.5 cm).
    pub rough_max_m: f32,
    /// Rover: same rule with its own limit. 0.40 m: Perseverance's obstacle limit (NASA Mars 2020 rover facts).
    pub rover_rough_max_m: f32,
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
            vehicle: Vehicle::Eva,
            body_kg: 80.0,
            suit_kg: 104.0,
            payload_kg: 20.0,
            s_max_base_deg: 20.0,
            rover_s_max_deg: 45.0,
            rover_kg: 1025.0,
            rover_rated_payload_kg: 100.0,
            rover_payload_kg: 20.0,
            rough_max_m: 0.30,
            rover_rough_max_m: 0.40,
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

/// Load factor; above 1 it lowers the slope limit and the speed.
/// EVA: loaded weight on Mars as a fraction of the walker's body weight on Earth.
/// Rover: loaded mass as a fraction of the rover with its rated payload (an experimental model).
pub fn weight_factor(p: &RouteParams) -> f32 {
    match p.vehicle {
        Vehicle::Eva => (p.body_kg + p.suit_kg + p.payload_kg) * G_MARS / (p.body_kg * G_EARTH),
        Vehicle::Rover => (p.rover_kg + p.rover_payload_kg) / (p.rover_kg + p.rover_rated_payload_kg).max(1.0),
    }
}

/// The vehicle's slope limit before the load factor.
pub fn s_max_base(p: &RouteParams) -> f32 {
    match p.vehicle {
        Vehicle::Eva => p.s_max_base_deg,
        Vehicle::Rover => p.rover_s_max_deg,
    }
}

/// Allowed slope: the vehicle's limit, lowered when the load factor exceeds 1.
pub fn s_max_eff(p: &RouteParams) -> f32 {
    s_max_base(p) / weight_factor(p).max(1.0)
}

/// The vehicle's roughness limit, m.
pub fn rough_max(p: &RouteParams) -> f32 {
    match p.vehicle {
        Vehicle::Eva => p.rough_max_m,
        Vehicle::Rover => p.rover_rough_max_m,
    }
}

/// Flat-ground speed with this load, m/s.
pub fn flat_speed(p: &RouteParams) -> f32 {
    let base = match p.vehicle {
        Vehicle::Eva => BASE_SPEED_MPS,
        Vehicle::Rover => ROVER_SPEED_MPS,
    };
    base / weight_factor(p).max(1.0)
}

/// Speed on ground of this slope as a fraction of the flat-ground speed.
/// EVA, Apollo walking data (Eppler et al. 2025, LPSC 56 #2023): Apollo 14 planned 1.2 m/s level and 0.6 m/s on
/// slopes (the ≈8° climb to Cone Crater): ×0.5 at 8°. Apollo 15 climbed ≈15° in deep regolith at ≈0.5 km/h:
/// ×0.11 of the 4.4 km/h level plan. Linear in between, held at ×0.11 up to the limit. These are lunar surface
/// values: the NASA Mars-suit trials found give no speed-versus-slope curve.
/// Rover: ENav drive-rate requirement, 100 m/h in benign terrain (slope < 15°) and 86 m/h in complex terrain
/// (Verma et al. 2026, IEEE Trans. Field Robotics).
// shortcut: cells store slope magnitude only, so downhill is timed like uphill (slower, conservative); use the
// signed grade between route cells if downhill times matter.
pub fn slope_speed_factor(slope_deg: f32, vehicle: Vehicle) -> f32 {
    match vehicle {
        Vehicle::Eva if slope_deg <= 8.0 => 1.0 - 0.5 * slope_deg / 8.0,
        Vehicle::Eva => (0.5 - 0.39 * (slope_deg - 8.0) / 7.0).max(0.11),
        Vehicle::Rover if slope_deg < 15.0 => 1.0,
        Vehicle::Rover => 0.86,
    }
}

pub struct CostGrid {
    pub n: usize,
    slope: Vec<f32>,
    rough: Vec<f32>,
    texture: Vec<u8>,
    rocks: Vec<u8>,
    /// EVA hazard (hazard.png): its 255 cells include the slope >= 20° rule.
    hazard: Vec<u8>,
    /// Rover hazard (hazard_phys.png): the ground itself; slope is left to the rover's own limit.
    hazard_phys: Vec<u8>,
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
            hazard_phys: site.hazard_phys.clone(),
            thermal: site.thermal.clone(),
            rad: site.radiation.clone(),
        }
    }

    /// Hazard value of cell `i` for this vehicle (0..254 = rough share, 255 = impassable).
    pub fn hazard_at(&self, i: usize, p: &RouteParams) -> u8 {
        match p.vehicle {
            Vehicle::Eva => self.hazard[i],
            Vehicle::Rover => self.hazard_phys[i],
        }
    }

    /// Which hard limits block cell `i`: (too steep, too rough, flagged impassable by the hazard model).
    pub fn limits_hit(&self, i: usize, p: &RouteParams) -> (bool, bool, bool) {
        // NASA's EVA site rule is "slope less than 20°": a cell exactly at the limit is blocked (both vehicles).
        (self.slope[i] >= s_max_eff(p), self.rough[i] > rough_max(p), self.hazard_at(i, p) == 255)
    }

    /// Cost multiplier for entering cell `i`; `None` = impassable.
    pub fn multiplier(&self, i: usize, p: &RouteParams) -> Option<f32> {
        let s_eff = s_max_eff(p);
        if self.limits_hit(i, p) != (false, false, false) {
            return None;
        }
        Some(
            1.0 + p.w_slope * (self.slope[i] / s_eff).powi(2)
                + p.w_rough * self.rough[i] / rough_max(p).max(1e-3)
                + p.w_texture * self.texture[i] as f32 / 255.0
                + p.w_rocks * self.rocks[i] as f32 / 255.0
                + p.w_hazard * self.hazard_at(i, p) as f32 / 254.0
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
    pub max_rough_m: f32,
    /// Cells with any mapped rock cover.
    pub rock_cells: u32,
    pub mean_hazard: f32,
    pub max_hazard: f32,
    /// A* cost: sum of step length × cell multiplier, in metres-equivalent.
    pub cost: f32,
}

pub fn plan(grid: &CostGrid, start: (usize, usize), goal: (usize, usize), p: &RouteParams) -> Option<RouteResult> {
    let n = grid.n;
    let idx = |(r, c): (usize, usize)| r * n + c;
    grid.multiplier(idx(start), p)?;
    grid.multiplier(idx(goal), p)?;

    let (cells, cost_cm) = astar(
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

    let speed = flat_speed(p);
    let (mut length_m, mut eta_s) = (0.0, 0.0);
    for w in cells.windows(2) {
        let diag = w[0].0 != w[1].0 && w[0].1 != w[1].1;
        let step = if diag { DIAG_M } else { STEP_M };
        length_m += step;
        eta_s += step / (speed * slope_speed_factor(grid.slope(idx(w[1])), p.vehicle));
    }
    let max_slope_deg = cells.iter().map(|&c| grid.slope(idx(c))).fold(0.0, f32::max);
    let max_rough_m = cells.iter().map(|&c| grid.rough[idx(c)]).fold(0.0, f32::max);
    let rock_cells = cells.iter().filter(|&&c| grid.rocks[idx(c)] > 0).count() as u32;
    let hazard = |c: (usize, usize)| (grid.hazard_at(idx(c), p) as f32 / 254.0).min(1.0);
    let mean_hazard = cells.iter().map(|&c| hazard(c)).sum::<f32>() / cells.len() as f32;
    let max_hazard = cells.iter().map(|&c| hazard(c)).fold(0.0, f32::max);
    Some(RouteResult {
        cells,
        length_m,
        eta_min: eta_s / 60.0,
        max_slope_deg,
        max_rough_m,
        rock_cells,
        mean_hazard,
        max_hazard,
        cost: cost_cm as f32 / 100.0,
    })
}

/// Same hard limits, no risk weights: `plan` with these finds the shortest path the vehicle is allowed
/// to take, the reference the safe route is compared against.
pub fn baseline_params(p: &RouteParams) -> RouteParams {
    RouteParams { w_slope: 0.0, w_hazard: 0.0, w_thermal: 0.0, w_rad: 0.0, w_rough: 0.0, w_texture: 0.0, w_rocks: 0.0, ..*p }
}

/// Cells on the straight line between two cells and how many of them break each hard limit (a cell
/// can break several, so the reasons may add up to more than `blocked`).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Crossing {
    pub cells: u32,
    pub blocked: u32,
    pub slope: u32,
    pub rough: u32,
    pub hazard: u32,
}

/// Walk the straight line from `a` to `b` (Bresenham, both ends included) and count impassable cells.
pub fn straight_line_blocked(grid: &CostGrid, a: (usize, usize), b: (usize, usize), p: &RouteParams) -> Crossing {
    let mut out = Crossing::default();
    let (mut x, mut y) = (a.1 as i64, a.0 as i64);
    let (x1, y1) = (b.1 as i64, b.0 as i64);
    let (dx, dy) = ((x1 - x).abs(), -(y1 - y).abs());
    let (sx, sy) = ((x1 - x).signum(), (y1 - y).signum());
    let mut err = dx + dy;
    loop {
        let (s, r, h) = grid.limits_hit(y as usize * grid.n + x as usize, p);
        out.cells += 1;
        out.blocked += (s || r || h) as u32;
        out.slope += s as u32;
        out.rough += r as u32;
        out.hazard += h as u32;
        if (x, y) == (x1, y1) {
            return out;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
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

/// World-space line of a route: through the cell centres, but starting and ending on the clicked points
/// `a` and `b` themselves, so the rover leaves from and stops exactly on the pins.
fn route_line(g: &CostGrid, cells: &[(usize, usize)], a: Vec3, b: Vec3, height: impl Fn(f32, f32) -> f32) -> Vec<Vec3> {
    let on_ground = |x: f32, z: f32| Vec3::new(x, height(x, z), z);
    let mut line: Vec<Vec3> = cells
        .iter()
        .map(|&cell| {
            let (x, z) = g.center(cell);
            on_ground(x, z)
        })
        .collect();
    if line.len() == 1 {
        line.push(line[0]); // start and end in the same cell
    }
    if let [first, .., last] = line.as_mut_slice() {
        (*first, *last) = (on_ground(a.x, a.z), on_ground(b.x, b.z));
    }
    line
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
    let (result, base, direct) = match (g.cell_of(a.x, a.z), g.cell_of(b.x, b.z)) {
        (Some(s), Some(e)) => {
            let result = plan(g, s, e, &params);
            // Same hard limits, so the baseline only exists when the safe route does.
            let base = result.as_ref().and_then(|_| plan(g, s, e, &baseline_params(&params)));
            (result, base, Some(straight_line_blocked(g, s, e, &params)))
        }
        _ => (None, None, None),
    };
    let direct = direct.map(|d| {
        serde_json::json!({"cells": d.cells, "blocked": d.blocked, "slope": d.slope, "rough": d.rough, "hazard": d.hazard})
    });
    let polyline = result.as_ref().map(|r| route_line(g, &r.cells, a, b, |x, z| site.height_at(x, z))).unwrap_or_default();
    let (sa, sb) = (site.latlon_of(a.x, a.z), site.latlon_of(b.x, b.z));
    let wf = weight_factor(&params);
    let s_eff = s_max_eff(&params);
    let vehicle = match params.vehicle {
        Vehicle::Eva => "eva",
        Vehicle::Rover => "rover",
    };
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
            let baseline = base.map(|b| {
                serde_json::json!({
                    "length_m": b.length_m, "max_slope_deg": b.max_slope_deg, "max_rough_m": b.max_rough_m,
                    "mean_hazard": b.mean_hazard, "rock_cells": b.rock_cells,
                })
            });
            serde_json::json!({
                "status": "ok", "vehicle": vehicle, "length_m": r.length_m, "eta_min": r.eta_min,
                "max_slope_deg": r.max_slope_deg,
                "mean_hazard": r.mean_hazard, "weight_factor": wf, "s_max_eff_deg": s_eff,
                "max_rough_m": r.max_rough_m, "rough_max_m": rough_max(&params), "rock_cells": r.rock_cells,
                "max_hazard": r.max_hazard, "baseline": baseline, "direct": direct,
                "ascent_m": ascent, "descent_m": descent, "profile": profile,
                "start": {"lat": sa.0, "lon": sa.1}, "end": {"lat": sb.0, "lon": sb.1},
            })
        }
        None => serde_json::json!({
            "status": "no_path", "vehicle": vehicle, "length_m": 0, "eta_min": 0, "max_slope_deg": 0, "mean_hazard": 0,
            "weight_factor": wf, "s_max_eff_deg": s_eff, "ascent_m": 0, "descent_m": 0, "profile": [],
            "max_rough_m": 0, "rough_max_m": rough_max(&params), "rock_cells": 0, "max_hazard": 0,
            "baseline": null, "direct": direct,
            "start": {"lat": sa.0, "lon": sa.1}, "end": {"lat": sb.0, "lon": sb.1},
        }),
    };
    crate::bridge::emit("mars-route", json.to_string());
    *route = Route { result, polyline };
}

/// The drawn route floats `RIBBON_LIFT_M` above the ground and is sampled every `RIBBON_STEP_M` (half
/// the relief mesh spacing), so it doesn't cut through the bumps between the planning cells.
pub(crate) const RIBBON_LIFT_M: f32 = 0.15;
pub(crate) const RIBBON_STEP_M: f32 = 1.0;

fn draw_route(route: Res<Route>, surface: Res<Surface>, vs: Res<VScale>, mut gizmos: Gizmos) {
    if route.polyline.len() > 1 {
        let line = surface.drape(route.polyline.iter().map(|p| p.xz()), *vs, RIBBON_LIFT_M, RIBBON_STEP_M);
        gizmos.linestrip(line, Color::srgb_u8(0x38, 0xe0, 0xc8));
    }
}

#[cfg(test)]
pub(crate) mod tests {
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
            hazard_phys: z.clone(),
            thermal: z.clone(),
            rad: z,
        }
    }

    fn rover() -> RouteParams {
        RouteParams { vehicle: Vehicle::Rover, ..RouteParams::default() }
    }

    /// The load that counts for this vehicle.
    fn load(p: &RouteParams) -> f32 {
        if p.vehicle == Vehicle::Rover { p.rover_payload_kg } else { p.payload_kg }
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
        assert_eq!(p.vehicle, Vehicle::Eva);
        assert!((weight_factor(&p) - 0.9644).abs() < 1e-3);
        assert_eq!(s_max_eff(&p), 20.0);
        assert!(plan(&g, (0, 0), (0, 19), &p).is_some());

        let heavy = RouteParams { payload_kg: 120.0, ..p };
        assert!((weight_factor(&heavy) - 1.437).abs() < 1e-3);
        assert!((s_max_eff(&heavy) - 13.92).abs() < 0.01);
        assert!(plan(&g, (0, 0), (0, 19), &heavy).is_none());
    }

    #[test]
    fn rover_limit_is_separate_from_eva() {
        let p = rover();
        // Same payload as the EVA default, but the rover's own limit and load model.
        assert!((weight_factor(&p) - 1045.0 / 1125.0).abs() < 1e-5);
        assert_eq!(s_max_eff(&p), 45.0);
        assert_eq!(s_max_eff(&RouteParams::default()), 20.0);
        // Changing one vehicle's limit leaves the other alone.
        assert_eq!(s_max_eff(&RouteParams { rover_s_max_deg: 30.0, ..RouteParams::default() }), 20.0);
        assert_eq!(s_max_eff(&RouteParams { s_max_base_deg: 10.0, ..p }), 45.0);
        assert!((flat_speed(&p) - ROVER_SPEED_MPS).abs() < 1e-6);
        assert!((flat_speed(&RouteParams::default()) - BASE_SPEED_MPS).abs() < 1e-6);
    }

    /// A 20-cell corridor of one slope: does each vehicle/load cross it? Expected from s_max_eff:
    /// EVA 20° (heavy 13.92°), rover 45° (heavy: 1,525 / 1,125 kg -> 33.20°).
    #[test]
    fn slope_table_by_vehicle_and_load() {
        let eva = RouteParams::default();
        let eva_heavy = RouteParams { payload_kg: 120.0, ..eva };
        let rov = rover();
        let rov_heavy = RouteParams { rover_payload_kg: 500.0, ..rov };
        assert!((s_max_eff(&rov_heavy) - 33.20).abs() < 0.01, "{}", s_max_eff(&rov_heavy));
        //            slope  eva    eva_heavy rover  rover_heavy
        let table = [
            (15.0, [true, false, true, true]),
            (25.0, [false, false, true, true]),
            (35.0, [false, false, true, false]),
            (44.0, [false, false, true, false]),
            (46.0, [false, false, false, false]),
        ];
        for (slope, expect) in table {
            let g = grid(20, slope);
            for (p, ok) in [eva, eva_heavy, rov, rov_heavy].iter().zip(expect) {
                let r = plan(&g, (0, 0), (0, 19), p);
                assert_eq!(r.is_some(), ok, "{slope}° {:?} payload {} (limit {:.2}°)", p.vehicle, load(p), s_max_eff(p));
                assert_eq!(slope < s_max_eff(p), ok);
                if let Some(r) = r {
                    assert!(r.max_slope_deg < s_max_eff(p));
                    // Steeper is slower and costlier, relative to the vehicle's own limit.
                    let t = slope / s_max_eff(p);
                    let eta = 19.0 * STEP_M / (flat_speed(p) * slope_speed_factor(slope, p.vehicle)) / 60.0;
                    assert!((r.eta_min - eta).abs() < 1e-3 * eta, "eta {} vs {}", r.eta_min, eta);
                    let m = 1.0 + p.w_slope * t * t;
                    // A* rounds each step to whole centimetres: ≤ 0.5 cm × 19 steps.
                    assert!((r.cost - 19.0 * STEP_M * m).abs() < 0.1,"cost {} vs {}", r.cost, 19.0 * STEP_M * m);
                }
            }
        }
        // Exactly at the limit is blocked: "slope less than the limit".
        assert!(plan(&grid(20, 45.0), (0, 0), (0, 19), &rov).is_none());
    }

    /// Travel times against the official figures they come from.
    #[test]
    fn travel_times_match_the_sources() {
        // Flat 1 km: suited walk at 3.0 km/h = 20 min; Perseverance at 98 m/h effective ≈ 10.2 h.
        let km = |v| 1000.0 / (flat_speed(&RouteParams { vehicle: v, payload_kg: 0.0, ..RouteParams::default() }) * 60.0);
        assert!((km(Vehicle::Eva) - 20.0).abs() < 0.01);
        assert!((km(Vehicle::Rover) - 1000.0 / 98.0 * 60.0).abs() < 0.1);
        // Apollo anchors (×0.5 at 8°, ×0.11 at 15°) and the ENav benign/complex rates (100 vs 86 m/h at 15°).
        for (deg, f) in [(0.0, 1.0), (8.0, 0.5), (15.0, 0.11), (19.0, 0.11)] {
            assert!((slope_speed_factor(deg, Vehicle::Eva) - f).abs() < 1e-6, "{deg}°");
        }
        assert_eq!((slope_speed_factor(14.9, Vehicle::Rover), slope_speed_factor(15.0, Vehicle::Rover)), (1.0, 0.86));
        // Never faster uphill.
        let mut prev = 1.0;
        for d in 0..20 {
            let f = slope_speed_factor(d as f32, Vehicle::Eva);
            assert!(f <= prev && f > 0.0);
            prev = f;
        }
    }

    #[test]
    fn each_vehicle_reads_its_own_hazard_layer() {
        // A wall the EVA model flags (slope rule) but the ground model does not: only the rover crosses.
        let mut g = grid(20, 0.0);
        for r in 0..20 {
            g.hazard[r * 20 + 10] = 255;
            g.hazard_phys[r * 20 + 10] = 60;
        }
        assert!(plan(&g, (0, 0), (0, 19), &RouteParams::default()).is_none());
        let r = plan(&g, (0, 0), (0, 19), &rover()).expect("rover path");
        assert!((r.max_hazard - 60.0 / 254.0).abs() < 1e-6);
        // A real obstacle in the ground model blocks the rover too.
        for r in 0..20 {
            g.hazard_phys[r * 20 + 10] = 255;
        }
        assert!(plan(&g, (0, 0), (0, 19), &rover()).is_none());
        assert_eq!(straight_line_blocked(&g, (5, 0), (5, 19), &rover()).hazard, 1);
    }

    #[test]
    fn each_vehicle_has_its_own_roughness_limit() {
        // A 0.35 m rough wall: over the suited step (0.30 m), under the rover's obstacle limit (0.40 m).
        let mut g = grid(20, 0.0);
        for r in 0..20 {
            g.rough[r * 20 + 10] = 0.35;
        }
        let eva = RouteParams::default();
        assert!(plan(&g, (0, 0), (0, 19), &eva).is_none());
        assert!(plan(&g, (0, 0), (0, 19), &rover()).is_some());
        // Changing one vehicle's limit leaves the other alone.
        assert!(plan(&g, (0, 0), (0, 19), &RouteParams { rover_rough_max_m: 1.0, ..eva }).is_none());
        assert!(plan(&g, (0, 0), (0, 19), &RouteParams { rough_max_m: 0.1, ..rover() }).is_some());
        assert!(plan(&g, (0, 0), (0, 19), &RouteParams { rover_rough_max_m: 0.3, ..rover() }).is_none());
    }

    #[test]
    fn params_json_from_the_web_app() {
        // What src/components/Planner.tsx sends through set_params.
        let p: RouteParams = serde_json::from_str(
            r#"{"vehicle":"rover","body_kg":80,"suit_kg":104,"payload_kg":20,"rover_payload_kg":500,"s_max_base_deg":20,
                "rover_s_max_deg":45,"rover_kg":1025,"rover_rated_payload_kg":100,"rough_max_m":0.3,
                "w_slope":3,"w_hazard":5,"w_thermal":0.5,"w_rad":0.5,"w_rough":2,"w_texture":1,"w_rocks":3}"#,
        )
        .expect("parses");
        assert_eq!(p.vehicle, Vehicle::Rover);
        assert!((s_max_eff(&p) - 33.20).abs() < 0.01);
        // Older clients without the new fields still get the EVA defaults.
        let old: RouteParams = serde_json::from_str(r#"{"payload_kg":20}"#).expect("parses");
        assert_eq!((old.vehicle, s_max_eff(&old)), (Vehicle::Eva, 20.0));
        assert!(serde_json::from_str::<RouteParams>(r#"{"vehicle":"truck"}"#).is_err());
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
    fn straight_line_counts_blocked_cells() {
        let mut g = grid(20, 0.0);
        for r in 0..20 {
            g.slope[r * 20 + 10] = 25.0;
        }
        let c = straight_line_blocked(&g, (5, 0), (5, 19), &RouteParams::default());
        assert_eq!(c, Crossing { cells: 20, blocked: 1, slope: 1, rough: 0, hazard: 0 });
        // Diagonal line: one cell per step along the longer axis.
        assert_eq!(straight_line_blocked(&g, (0, 0), (19, 19), &RouteParams::default()).cells, 20);
    }

    #[test]
    fn baseline_is_shorter_but_riskier() {
        let mut g = grid(20, 0.0);
        for r in 8..=12 {
            g.rocks[r * 20 + 10] = 255;
        }
        let p = RouteParams::default();
        let safe = plan(&g, (10, 0), (10, 19), &p).expect("safe path");
        let base = plan(&g, (10, 0), (10, 19), &baseline_params(&p)).expect("baseline path");
        assert_eq!(safe.rock_cells, 0);
        assert!(base.rock_cells > 0, "baseline {:?}", base.cells);
        assert!(base.length_m < safe.length_m, "{} vs {}", base.length_m, safe.length_m);
    }

    /// Sum of step length × entry multiplier, as A* scores a path.
    fn path_cost(g: &CostGrid, cells: &[(usize, usize)], p: &RouteParams) -> f32 {
        cells
            .windows(2)
            .map(|w| {
                let step = if w[0].0 != w[1].0 && w[0].1 != w[1].1 { DIAG_M } else { STEP_M };
                step * g.multiplier(w[1].0 * g.n + w[1].1, p).expect("open cell")
            })
            .sum()
    }

    #[test]
    fn route_line_starts_and_ends_on_the_clicked_points() {
        let g = grid(20, 0.0);
        let r = plan(&g, (3, 2), (3, 9), &RouteParams::default()).expect("path");
        let (start, end) = (Vec2::from(g.center((3, 2))), Vec2::from(g.center((3, 9))));
        // Clicks land anywhere inside the cell, not on its centre.
        let (a, b) = (Vec3::new(start.x + 3.0, 7.0, start.y - 4.0), Vec3::new(end.x - 2.5, 7.0, end.y + 4.5));
        let line = route_line(&g, &r.cells, a, b, |_, _| 1.0);
        assert_eq!(line.len(), r.cells.len());
        assert_eq!((line[0], line[line.len() - 1]), (Vec3::new(a.x, 1.0, a.z), Vec3::new(b.x, 1.0, b.z)));
        assert_eq!(line[1].xz(), Vec2::from(g.center(r.cells[1])));
        // Start and end in the same cell still give a line to walk.
        let line = route_line(&g, &[(3, 2)], a, b, |_, _| 1.0);
        assert_eq!(line, vec![Vec3::new(a.x, 1.0, a.z), Vec3::new(b.x, 1.0, b.z)]);
    }

    /// Start/end cells spread over the region, for the checks on the real site.
    pub(crate) const PAIRS: [((usize, usize), (usize, usize)); 8] = [
        ((100, 100), (300, 400)),
        ((500, 500), (650, 700)),
        ((200, 800), (450, 650)),
        ((800, 200), (600, 350)),
        ((900, 900), (700, 750)),
        ((50, 500), (250, 520)),
        ((500, 50), (520, 300)),
        ((350, 350), (500, 600)),
    ];

    /// First open cell on growing square rings around `cell`.
    pub(crate) fn open_near(g: &CostGrid, (r, c): (usize, usize), p: &RouteParams) -> (usize, usize) {
        for d in 0..50i64 {
            for dr in -d..=d {
                for dc in -d..=d {
                    let (nr, nc) = (r as i64 + dr, c as i64 + dc);
                    if dr.abs().max(dc.abs()) == d && (0..g.n as i64).contains(&nr) && (0..g.n as i64).contains(&nc) {
                        let cell = (nr as usize, nc as usize);
                        if g.multiplier(cell.0 * g.n + cell.1, p).is_some() {
                            return cell;
                        }
                    }
                }
            }
        }
        (r, c)
    }

    #[test]
    fn route_respects_hard_limits_on_real_site() {
        let site = Site::load();
        let g = CostGrid::from_site(&site);
        for p in [RouteParams::default(), rover(), RouteParams { rover_payload_kg: 500.0, ..rover() }] {
            check_real_site(&site, &g, &p);
        }
    }

    /// User report: with the rover at 45° it still could not reach 23° ground. On the real site, ground at
    /// 22–24° that the rover's own limits allow must be reachable for the rover and closed to the walker.
    #[test]
    fn rover_reaches_steep_ground_the_walker_cannot() {
        let site = Site::load();
        let g = CostGrid::from_site(&site);
        let (eva, rov) = (RouteParams::default(), rover());
        let steep: Vec<usize> = (0..g.n * g.n)
            .filter(|&i| (22.0..24.0).contains(&site.slope_deg[i]) && g.multiplier(i, &rov).is_some())
            .collect();
        assert!(steep.len() > 1000, "{} open 22–24° cells for the rover", steep.len());
        assert!(steep.iter().all(|&i| g.multiplier(i, &eva).is_none()));
        let mut reached = 0;
        for &i in steep.iter().step_by(steep.len() / 10) {
            let goal = (i / g.n, i % g.n);
            let start = open_near(&g, ((goal.0 + 30).min(g.n - 1), goal.1), &rov);
            if let Some(r) = plan(&g, start, goal, &rov) {
                reached += 1;
                assert!(r.max_slope_deg >= 22.0 && r.max_slope_deg < s_max_eff(&rov));
            }
            assert!(plan(&g, start, goal, &eva).is_none());
        }
        assert!(reached >= 8, "rover reached {reached} of 10 steep goals");
    }

    fn check_real_site(site: &Site, g: &CostGrid, p: &RouteParams) {
        let p = *p;
        let open = |c: (usize, usize)| g.multiplier(c.0 * g.n + c.1, &p).is_some();
        let mut ok = 0;
        eprintln!("{:?} payload {} kg, slope limit {:.2}°", p.vehicle, load(&p), s_max_eff(&p));
        eprintln!("pair | length safe/base m | max slope safe/base ° | max rough safe/base m | rock cells safe/base");
        for (a, b) in PAIRS {
            let (a, b) = (open_near(g, a, &p), open_near(g, b, &p));
            let Some(r) = plan(g, a, b, &p) else { continue };
            let base = plan(g, a, b, &baseline_params(&p)).expect("baseline exists when the safe route does");
            ok += 1;
            for &(row, col) in &r.cells {
                let i = row * g.n + col;
                // Never on a cell any layer marks as really dangerous for this vehicle.
                assert!(site.slope_deg[i] < s_max_eff(&p) && site.rough_m[i] <= rough_max(&p), "{:?}", (row, col));
                let hz = if p.vehicle == Vehicle::Eva { site.hazard[i] } else { site.hazard_phys[i] };
                assert_ne!(hz, 255, "route enters an impassable cell {:?}", (row, col));
            }
            assert!(r.cells.iter().all(|&c| open(c)));
            for w in r.cells.windows(2) {
                let (dr, dc) = (w[0].0.abs_diff(w[1].0), w[0].1.abs_diff(w[1].1));
                assert_eq!(dr.max(dc), 1, "non-adjacent step {:?}", w);
                if dr == 1 && dc == 1 {
                    assert!(open((w[1].0, w[0].1)) && open((w[0].0, w[1].1)), "corner cut {:?}", w);
                }
            }
            assert!(r.max_slope_deg < s_max_eff(&p) && r.max_rough_m <= rough_max(&p));
            // A* is optimal under the safety weights: the shortest allowed path never scores better.
            let base_cost = path_cost(g, &base.cells, &p);
            assert!(r.cost <= base_cost + 0.01 * base.cells.len() as f32, "{} vs {}", r.cost, base_cost);
            assert!(r.length_m + 1e-3 >= base.length_m);
            eprintln!(
                "{a:?}->{b:?} | {:.0}/{:.0} | {:.1}/{:.1} | {:.2}/{:.2} | {}/{}",
                r.length_m, base.length_m, r.max_slope_deg, base.max_slope_deg, r.max_rough_m, base.max_rough_m,
                r.rock_cells, base.rock_cells
            );
        }
        assert!(ok >= 5, "only {ok} routes");
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
