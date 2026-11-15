//! The rover: NASA/JPL-Caltech's Perseverance 3D model (the team's 25042_Perseverance.glb from NASA 3D
//! Resources, with NASA/JPL logos, flag and mission insignia removed from its textures; served as
//! public/data/jezero/rover.glb) at true scale, playing back the planned route. It rests on the rendered
//! terrain, tilted to the plane through its wheel contacts.

use bevy::prelude::*;

use crate::camera::{CameraMode, MainCamera};
use crate::data::{Site, VScale};
use crate::picking::{PickTarget, pick_json};
use crate::route::{Endpoints, Grid, Route, RouteParams};
use crate::terrain::Surface;

/// Minimum playback speed along the route, m/s (sped up for the demo).
const SIM_SPEED: f32 = 30.0;
/// Long routes play back in about this many seconds.
const PLAYBACK_S: f32 = 45.0;
/// Wheel contact points from the centre: half wheelbase and half track, m (model bbox 2.7 × 3.1 m).
const HALF_BASE_M: f32 = 1.3;
const HALF_TRACK_M: f32 = 1.15;

#[derive(Component)]
pub struct Rover;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoverSet;

#[derive(Resource)]
pub struct RoverPlayback {
    pub playing: bool,
    /// Distance travelled along the route polyline, m.
    pub s: f32,
    /// Playback speed multiplier (UI: x1, x2, x4). Survives route changes.
    pub speed: f32,
}

impl Default for RoverPlayback {
    fn default() -> Self {
        Self { playing: false, s: 0.0, speed: 1.0 }
    }
}

pub struct RoverPlugin;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoverPlayback>()
            .add_systems(Startup, spawn)
            .add_systems(
                Update,
                (reset_on_route_change, advance, map_scale, settle)
                    .chain()
                    .after(crate::route::replan)
                    .after(crate::terrain::TerrainSet)
                    .in_set(RoverSet),
            );
    }
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((Rover, Heading(Vec2::NEG_Y), Transform::default(), Visibility::default())).with_child((
        WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("rover.glb"))),
        // The model faces +Z; the sim's forward is -Z.
        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
    ));
}

/// Driving direction on the ground plane (x, z); the pose is rebuilt from it every frame.
#[derive(Component)]
struct Heading(Vec2);

/// Pose at x/z facing `heading`: height and tilt from the four wheel contacts on the rendered ground.
fn pose(surface: &Surface, vs: VScale, x: f32, z: f32, heading: Vec2, scale: f32) -> Transform {
    let fwd = heading.try_normalize().unwrap_or(Vec2::NEG_Y);
    let right = Vec2::new(-fwd.y, fwd.x);
    let at = |o: Vec2| {
        let q = Vec2::new(x, z) + o * scale;
        Vec3::new(q.x, surface.ground(q.x, q.y, vs), q.y)
    };
    let (f, b) = (at(fwd * HALF_BASE_M), at(-fwd * HALF_BASE_M));
    let (l, r) = (at(-right * HALF_TRACK_M), at(right * HALF_TRACK_M));
    let along = (f - b).normalize();
    let up = (r - l).normalize().cross(along).normalize();
    let y = (f.y + b.y + l.y + r.y) / 4.0;
    Transform::from_xyz(x, y, z).looking_to(along, up).with_scale(Vec3::splat(scale))
}

/// Point and heading at distance `s` along the polyline; `None` past the end.
fn along(line: &[Vec3], mut s: f32) -> Option<(Vec3, Vec3)> {
    for w in line.windows(2) {
        let seg = w[1] - w[0];
        let len = seg.length();
        if s <= len && len > 0.0 {
            return Some((w[0] + seg * (s / len), seg));
        }
        s -= len;
    }
    None
}

fn reset_on_route_change(
    route: Res<Route>,
    ends: Res<Endpoints>,
    mut playback: ResMut<RoverPlayback>,
    mut rover: Query<(&mut Transform, &mut Heading), With<Rover>>,
) {
    if !route.is_changed() && !ends.is_changed() {
        return;
    }
    let Ok((mut tf, mut heading)) = rover.single_mut() else { return };
    *playback = RoverPlayback { speed: playback.speed, ..default() };
    match along(&route.polyline, 0.0) {
        Some((p, dir)) => {
            tf.translation = p;
            heading.0 = dir.xz();
        }
        // No route: wait at the start (after a finished leg, where the rover already stands), facing as before.
        None => tf.translation = ends.start.unwrap_or(Vec3::ZERO),
    }
}

#[allow(clippy::too_many_arguments)]
fn advance(
    time: Res<Time>,
    route: Res<Route>,
    site: Res<Site>,
    grid: Res<Grid>,
    params: Res<RouteParams>,
    mut playback: ResMut<RoverPlayback>,
    mut ends: ResMut<Endpoints>,
    mut target: ResMut<PickTarget>,
    mut rover: Query<(&mut Transform, &mut Heading), With<Rover>>,
) {
    if !playback.playing || route.polyline.len() < 2 {
        return;
    }
    let Ok((mut tf, mut heading)) = rover.single_mut() else { return };
    let length = route.result.as_ref().map_or(0.0, |r| r.length_m);
    playback.s += (length / PLAYBACK_S).max(SIM_SPEED) * playback.speed * time.delta_secs();
    match along(&route.polyline, playback.s) {
        Some((p, dir)) => {
            tf.translation = p;
            heading.0 = dir.xz();
        }
        None => {
            playback.playing = false;
            // Stop on the destination itself, not a frame short of it, and make it the start of the next
            // leg: the rover stays there and the next click picks where it goes from here.
            let Some(end) = ends.end.take() else { return };
            tf.translation = end;
            ends.start = Some(end);
            *target = PickTarget::End;
            crate::bridge::emit("mars-rover-stopped", pick_json(&site, &grid, &params, end, "start"));
        }
    }
}

/// In map view a 3 m rover is invisible from kilometres away: scale it with camera distance there.
fn map_scale(
    mode: Res<CameraMode>,
    cam: Query<&GlobalTransform, With<MainCamera>>,
    mut rover: Query<&mut Transform, (With<Rover>, Without<MainCamera>)>,
) {
    let (Ok(cam), Ok(mut tf)) = (cam.single(), rover.single_mut()) else { return };
    let s = match *mode {
        CameraMode::Map => (cam.translation().distance(tf.translation) / 250.0).clamp(1.0, 40.0),
        _ => 1.0,
    };
    tf.scale = Vec3::splat(s);
}

/// Every frame: the detail tile under the rover may have streamed in or the exaggeration changed.
fn settle(surface: Res<Surface>, vs: Res<VScale>, mut rover: Query<(&mut Transform, &Heading), With<Rover>>) {
    let Ok((mut tf, heading)) = rover.single_mut() else { return };
    *tf = pose(&surface, *vs, tf.translation.x, tf.translation.z, heading.0, tf.scale.x);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::route::tests::{PAIRS, open_near};
    use crate::route::{CostGrid, RIBBON_LIFT_M, RIBBON_STEP_M, RouteParams, Vehicle, plan, s_max_eff};
    use crate::terrain::{NT, TILE_M};
    use std::collections::BTreeSet;

    /// The planner works on the DTM layers, but next to the camera the rover now stands on a mesh that
    /// also carries relief read from the image. Drive the real-site routes over that ground.
    #[test]
    fn routes_hold_on_the_relief_terrain() {
        let site = Site::load();
        let g = CostGrid::from_site(&site);
        // EVA routes and rover-mode routes (45°, experimental) over the same ground.
        for p in [RouteParams::default(), RouteParams { vehicle: Vehicle::Rover, ..RouteParams::default() }] {
            drive_routes(&site, &g, p);
        }
    }

    fn drive_routes(site: &Site, g: &CostGrid, p: RouteParams) {
        let vs = VScale(1.0);
        eprintln!("{:?}: slope limit {:.1}°", p.vehicle, s_max_eff(&p));
        let mut ok = 0;
        eprintln!(
            "pair | length m | max tilt on the 4 m mesh / relief mesh ° | most the relief adds ° | metres tilted past the limit | relief under the rover rms / max m | route line clearance m"
        );
        for (a, b) in PAIRS {
            let (a, b) = (open_near(&g, a, &p), open_near(&g, b, &p));
            let Some(r) = plan(&g, a, b, &p) else { continue };
            ok += 1;
            let line: Vec<Vec3> = r
                .cells
                .iter()
                .map(|&c| {
                    let (x, z) = g.center(c);
                    Vec3::new(x, 0.0, z)
                })
                .collect();
            // Every chunk within reach of the wheel contacts.
            let tile = |v: f32| ((v + crate::data::HALF_M) / TILE_M).clamp(0.0, NT as f32 - 1e-3) as usize;
            let chunks: BTreeSet<(usize, usize)> = line
                .iter()
                .flat_map(|q| [-5.0, 5.0].map(|dx| [-5.0, 5.0].map(|dz| (tile(q.x + dx), tile(q.z + dz)))))
                .flatten()
                .collect();
            let (detail, sharp) = Surface::from_tiles(&site, chunks);

            let tilt = |s: &Surface, q: Vec3, dir: Vec3| pose(s, vs, q.x, q.z, dir.xz(), 1.0).up().angle_between(Vec3::Y).to_degrees();
            let (mut flat, mut bumpy, mut moved) = (Vec::new(), Vec::new(), Vec::new());
            let mut d = 0.0;
            while let Some((q, dir)) = along(&line, d) {
                flat.push(tilt(&detail, q, dir));
                bumpy.push(tilt(&sharp, q, dir));
                moved.push(sharp.height(q.x, q.z) - detail.height(q.x, q.z));
                d += 0.5;
            }
            let max = |v: &[f32]| v.iter().copied().fold(0.0, f32::max);
            let added = flat.iter().zip(&bumpy).map(|(f, b)| b - f).fold(0.0, f32::max);
            let over_m = bumpy.iter().filter(|t| **t >= s_max_eff(&p)).count() as f32 * 0.5;
            let rms = (moved.iter().map(|m| m * m).sum::<f32>() / moved.len() as f32).sqrt();
            let step = moved.iter().map(|m| m.abs()).fold(0.0, f32::max);

            // The drawn route must stay above the bumps between its samples.
            let ribbon = sharp.drape(line.iter().map(|q| q.xz()), vs, RIBBON_LIFT_M, RIBBON_STEP_M);
            let clearance = ribbon
                .windows(2)
                .flat_map(|w| (0..=8).map(move |k| w[0].lerp(w[1], k as f32 / 8.0)))
                .map(|q| q.y - sharp.height(q.x, q.z))
                .fold(f32::MAX, f32::min);

            eprintln!(
                "{a:?}->{b:?} | {:.0} | {:.1}/{:.1} | {added:.1} | {over_m:.1} | {rms:.2}/{step:.2} | {clearance:.3}",
                r.length_m,
                max(&flat),
                max(&bumpy)
            );
            assert!(clearance > 0.0, "route line dips {clearance} m under the ground");
            // On the DTM mesh the rover stays inside the planner's slope limit.
            assert!(max(&flat) < s_max_eff(&p), "rover tilts {}° on the DTM mesh", max(&flat));
            // The relief is an estimate: it may tip the rover a little further, not throw it around. Rover routes
            // may cross rougher ground (limit 0.40 m vs 0.30 m on foot), where the image relief is stronger.
            assert!(added < 10.0, "relief adds {added}° of tilt");
        }
        assert!(ok >= 5, "only {ok} routes");
    }
}
