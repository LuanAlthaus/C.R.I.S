//! The rover: NASA/JPL-Caltech's Perseverance 3D model (the team's 25042_Perseverance.glb from NASA 3D
//! Resources, with NASA/JPL logos, flag and mission insignia removed from its textures; served as
//! public/data/jezero/rover.glb) at true scale, playing back the planned route. It rests on the rendered
//! terrain, tilted to the plane through its wheel contacts.

use bevy::prelude::*;

use crate::camera::{CameraMode, MainCamera};
use crate::data::VScale;
use crate::route::{Endpoints, Route};
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

#[derive(Resource, Default)]
pub struct RoverPlayback {
    pub playing: bool,
    /// Distance travelled along the route polyline, m.
    pub s: f32,
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
    *playback = RoverPlayback::default();
    let (p, dir) = along(&route.polyline, 0.0).unwrap_or((ends.start.unwrap_or(Vec3::ZERO), Vec3::NEG_Z));
    tf.translation = p;
    heading.0 = dir.xz();
}

fn advance(
    time: Res<Time>,
    route: Res<Route>,
    mut playback: ResMut<RoverPlayback>,
    mut rover: Query<(&mut Transform, &mut Heading), With<Rover>>,
) {
    if !playback.playing || route.polyline.len() < 2 {
        return;
    }
    let Ok((mut tf, mut heading)) = rover.single_mut() else { return };
    let length = route.result.as_ref().map_or(0.0, |r| r.length_m);
    playback.s += (length / PLAYBACK_S).max(SIM_SPEED) * time.delta_secs();
    match along(&route.polyline, playback.s) {
        Some((p, dir)) => {
            tf.translation = p;
            heading.0 = dir.xz();
        }
        None => {
            playback.playing = false;
            crate::bridge::emit("mars-rover-stopped", "{}".into());
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

/// Restart from the beginning when play is pressed after the route finished.
pub fn rewind_if_finished(playback: &mut RoverPlayback, route: &Route) {
    if along(&route.polyline, playback.s).is_none() {
        playback.s = 0.0;
    }
}
