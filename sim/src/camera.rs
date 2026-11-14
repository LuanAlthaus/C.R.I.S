//! One camera, three modes:
//! - Map: works like Google Maps in 3D. Drag pans (the grabbed ground stays under the cursor and glides on
//!   release), wheel/pinch zooms toward the cursor, double-click zooms in on that point, Shift-, Ctrl- or
//!   right-drag rotates (left/right) and tilts (up/down), two-finger twist rotates and two-finger vertical
//!   drag tilts. Keys: arrows/WASD pan, +/- zoom, Shift+arrows or Q/E rotate and tilt. The page adds
//!   compass, zoom and 2D/3D buttons (`map_*` in the bridge).
//! - Drone: free flight from the rover's viewpoint. Drag looks around, WASD flies, Q/E or ←/→ turn, ↑/↓ look
//!   up/down, R/F climb/descend, Shift fast, wheel changes speed; never below the ground.
//! - Rover: chase camera (drag orbits around the rover, wheel zooms), kept above the rendered ground.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::gizmos::config::{DefaultGizmoConfigGroup, GizmoConfigStore};
use bevy::prelude::*;

use crate::data::{HALF_M, VScale};
use crate::picking::{Pointer, hit_terrain};
use crate::rover::Rover;
use crate::route::Endpoints;
use crate::terrain::Surface;

/// Map pitch above the horizon: lowest tilt, straight down (2D) and the default 3D view.
const PITCH_MIN: f32 = 0.15;
const PITCH_TOP: f32 = FRAC_PI_2;
const PITCH_3D: f32 = 0.95;
const DIST_MIN: f32 = 40.0;
const DIST_MAX: f32 = 16_000.0;
/// ln(zoom factor) per wheel notch or zoom button press.
const ZOOM_STEP: f32 = 0.35;

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CameraMode {
    #[default]
    Map,
    Drone,
    Rover,
}

impl CameraMode {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "map" => CameraMode::Map,
            "drone" => CameraMode::Drone,
            "rover" => CameraMode::Rover,
            _ => return None,
        })
    }
}

#[derive(Component)]
pub struct MainCamera;

/// Animated map move (double-click, buttons, reset); any drag or wheel cancels it.
#[derive(Clone, Copy)]
struct Fly {
    focus: Vec2,
    dist: f32,
    yaw: f32,
    pitch: f32,
}

/// Controller state for all modes.
#[derive(Resource)]
pub struct CamCtl {
    focus: Vec2,
    yaw: f32,
    pitch: f32,
    dist: f32,
    /// Ground point held under the cursor while panning (world, exaggerated).
    grab: Option<Vec3>,
    /// Pan velocity kept after the drag ends (m/s), decays to zero.
    glide: Vec2,
    /// Zoom still to apply (ln factor) and the ground point it zooms toward.
    zoom_left: f32,
    zoom_pivot: Option<Vec2>,
    fly: Option<Fly>,
    drone_pos: Vec3,
    drone_yaw: f32,
    drone_pitch: f32,
    drone_speed: f32,
    chase_yaw: f32,
    chase_pitch: f32,
    chase_dist: f32,
}

impl Default for CamCtl {
    fn default() -> Self {
        CamCtl {
            focus: Vec2::ZERO,
            yaw: 0.0,
            pitch: PITCH_3D,
            dist: 9500.0,
            grab: None,
            glide: Vec2::ZERO,
            zoom_left: 0.0,
            zoom_pivot: None,
            fly: None,
            drone_pos: Vec3::new(0.0, 50.0, 0.0),
            drone_yaw: 0.0,
            drone_pitch: -0.15,
            drone_speed: 15.0,
            chase_yaw: 0.0,
            chase_pitch: 0.3,
            chase_dist: 11.0,
        }
    }
}

impl CamCtl {
    fn fly_to(&mut self, f: impl FnOnce(&mut Fly)) {
        let mut fly = self.fly.unwrap_or(Fly { focus: self.focus, dist: self.dist, yaw: self.yaw, pitch: self.pitch });
        f(&mut fly);
        self.fly = Some(fly);
    }

    /// Zoom button / keyboard: `steps` > 0 zooms in about the view centre.
    pub fn zoom_steps(&mut self, steps: f32) {
        self.zoom_left -= steps * ZOOM_STEP;
        self.zoom_pivot = None;
    }

    /// Compass: turn north up.
    pub fn north_up(&mut self) {
        self.fly_to(|f| f.yaw = 0.0);
    }

    /// 2D (straight down) or 3D (tilted) view.
    pub fn top_down(&mut self, top: bool) {
        self.fly_to(|f| f.pitch = if top { PITCH_TOP } else { PITCH_3D });
    }
}

/// Set by the bridge: frame the route endpoints (or the whole region) in map mode.
#[derive(Resource, Default)]
pub struct ResetView(pub bool);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraMode>()
            .init_resource::<CamCtl>()
            .init_resource::<ResetView>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (enter_mode, reset_view, control, report_heading).chain().after(crate::rover::RoverSet),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection { near: 0.2, far: 40_000.0, ..default() }),
        Transform::from_xyz(0.0, 8000.0, 6000.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Horizontal forward (yaw 0 looks north = -z) and right vectors.
fn heading(yaw: f32) -> (Vec3, Vec3) {
    let fwd = Vec3::new(-yaw.sin(), 0.0, -yaw.cos());
    (fwd, Vec3::new(-fwd.z, 0.0, fwd.x))
}

/// Angle difference wrapped to (-π, π].
fn wrap(a: f32) -> f32 {
    (a + PI).rem_euclid(TAU) - PI
}

fn enter_mode(
    mode: Res<CameraMode>,
    mut ctl: ResMut<CamCtl>,
    rover: Query<&Transform, With<Rover>>,
    mut gizmos: ResMut<GizmoConfigStore>,
) {
    if !mode.is_changed() {
        return;
    }
    // Map: route and traverse draw on top like a map overlay. Ground views: plain depth test, so hills and the
    // rover hide the lines (they are draped 0.15 m above the ground).
    gizmos.config_mut::<DefaultGizmoConfigGroup>().0.depth_bias = if *mode == CameraMode::Map { -1.0 } else { 0.0 };
    if *mode != CameraMode::Drone {
        return;
    }
    // Start the drone where the rover's chase camera would be, looking the way the rover faces.
    let Ok(r) = rover.single() else { return };
    let f = r.forward();
    let yaw = (-f.x).atan2(-f.z);
    let (fwd, _) = heading(yaw);
    ctl.drone_pos = r.translation - fwd * 10.0 + Vec3::Y * 4.0;
    ctl.drone_yaw = yaw;
    ctl.drone_pitch = -0.15;
}

fn reset_view(mut reset: ResMut<ResetView>, mut ctl: ResMut<CamCtl>, ends: Res<Endpoints>) {
    if !reset.0 {
        return;
    }
    reset.0 = false;
    let d = CamCtl::default();
    let (focus, dist) = match (ends.start, ends.end) {
        (Some(a), Some(b)) => ((a.xz() + b.xz()) / 2.0, (a.xz().distance(b.xz()) * 1.6).clamp(300.0, d.dist)),
        (Some(a), None) | (None, Some(a)) => (a.xz(), 800.0),
        (None, None) => (d.focus, d.dist),
    };
    (ctl.zoom_left, ctl.glide) = (0.0, Vec2::ZERO);
    ctl.fly = Some(Fly { focus, dist, yaw: d.yaw, pitch: d.pitch });
}

#[allow(clippy::too_many_arguments)]
fn control(
    mode: Res<CameraMode>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    pointer: Res<Pointer>,
    surface: Res<Surface>,
    vs: Res<VScale>,
    mut ctl: ResMut<CamCtl>,
    rover: Query<&Transform, (With<Rover>, Without<MainCamera>)>,
    mut cam: Query<(&mut Transform, &Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok((mut tf, camera, gtf)) = cam.single_mut() else { return };
    let dt = time.delta_secs();
    let key = |k: KeyCode| keys.pressed(k) as i32 as f32;
    let shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let ctrl = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let wasd = Vec2::new(key(KeyCode::KeyD) - key(KeyCode::KeyA), key(KeyCode::KeyW) - key(KeyCode::KeyS));
    let arrows = Vec2::new(
        key(KeyCode::ArrowRight) - key(KeyCode::ArrowLeft),
        key(KeyCode::ArrowUp) - key(KeyCode::ArrowDown),
    );
    let d = if pointer.dragging { pointer.delta } else { Vec2::ZERO };
    let ground = |x: f32, z: f32| surface.ground(x, z, *vs);
    // World point (exaggerated) under a screen position, using last frame's camera.
    let hit = |pos: Vec2| {
        let ray = camera.viewport_to_world(gtf, pos).ok()?;
        hit_terrain(&surface, *vs, ray).map(|p| Vec3::new(p.x, p.y * vs.0, p.z))
    };

    match *mode {
        CameraMode::Map => {
            let rotate_drag = pointer.right || (pointer.left && (shift || ctrl));
            if pointer.dragging || pointer.zoom != 0.0 {
                ctl.fly = None;
            }
            // Pan: keep the grabbed ground point under the cursor.
            if pointer.dragging && pointer.left && !rotate_drag {
                if ctl.grab.is_none() {
                    ctl.grab = pointer.down_at.and_then(hit);
                    ctl.glide = Vec2::ZERO;
                }
                let held = ctl.grab.zip(pointer.pos.and_then(|p| camera.viewport_to_world(gtf, p).ok()));
                if let Some((g, ray)) = held
                    && let Some(t) = ray.intersect_plane(g, InfinitePlane3d::new(Vec3::Y))
                {
                    let shift_xz = (g - ray.get_point(t)).xz();
                    ctl.focus += shift_xz;
                    if dt > 0.0 {
                        ctl.glide = ctl.glide.lerp(shift_xz / dt, 0.5);
                    }
                }
            } else {
                ctl.grab = None;
                let glide = ctl.glide;
                ctl.focus += glide * dt;
                ctl.glide *= (-5.0 * dt).exp();
            }
            if rotate_drag && pointer.dragging {
                ctl.yaw -= d.x * 0.005;
                ctl.pitch += d.y * 0.004;
            }
            // Two fingers: twist rotates (the map follows the fingers), dragging down tilts toward the horizon.
            ctl.yaw -= pointer.twist;
            ctl.pitch -= pointer.two_finger.y * 0.004;

            // Keyboard: arrows/WASD pan, Shift+arrows rotate/tilt, Q/E rotate, +/- zoom.
            let (fwd, right) = heading(ctl.yaw);
            let pan = if shift { wasd } else { wasd + arrows };
            let step = (right * pan.x + fwd * pan.y).xz() * ctl.dist * 0.6 * dt;
            ctl.focus += step;
            if shift {
                ctl.yaw -= arrows.x * 1.2 * dt;
                ctl.pitch += arrows.y * 0.8 * dt;
            }
            ctl.yaw += (key(KeyCode::KeyQ) - key(KeyCode::KeyE)) * 1.2 * dt;
            if keys.any_just_pressed([KeyCode::Equal, KeyCode::NumpadAdd]) {
                ctl.zoom_steps(1.0);
            }
            if keys.any_just_pressed([KeyCode::Minus, KeyCode::NumpadSubtract]) {
                ctl.zoom_steps(-1.0);
            }

            // Wheel/pinch zoom toward the ground under the cursor, eased over a few frames.
            if pointer.zoom != 0.0 {
                ctl.zoom_left -= pointer.zoom * ZOOM_STEP;
                ctl.zoom_pivot = pointer.pos.and_then(hit).map(|p| p.xz());
            }
            if ctl.zoom_left.abs() > 1e-4 {
                let step = ctl.zoom_left * (1.0 - (-14.0 * dt).exp());
                ctl.zoom_left -= step;
                let dist = (ctl.dist * step.exp()).clamp(DIST_MIN, DIST_MAX);
                let k = dist / ctl.dist;
                if let Some(p) = ctl.zoom_pivot {
                    ctl.focus = p + (ctl.focus - p) * k;
                }
                ctl.dist = dist;
            } else {
                ctl.zoom_left = 0.0;
            }
            // Double-click: zoom in 2× centred on the point.
            if let Some(p) = pointer.double_click.and_then(hit) {
                let dist = (ctl.dist * 0.5).max(DIST_MIN);
                ctl.fly_to(|f| (f.focus, f.dist) = (p.xz(), dist));
            }
            if let Some(f) = ctl.fly {
                let a = 1.0 - (-6.0 * dt).exp();
                ctl.focus = ctl.focus.lerp(f.focus, a);
                ctl.dist *= (f.dist / ctl.dist).powf(a);
                ctl.yaw += wrap(f.yaw - ctl.yaw) * a;
                ctl.pitch += (f.pitch - ctl.pitch) * a;
                let done = ctl.focus.distance(f.focus) < 0.5
                    && (ctl.dist / f.dist - 1.0).abs() < 1e-3
                    && wrap(f.yaw - ctl.yaw).abs() < 1e-3
                    && (f.pitch - ctl.pitch).abs() < 1e-3;
                if done {
                    (ctl.focus, ctl.dist, ctl.yaw, ctl.pitch, ctl.fly) = (f.focus, f.dist, f.yaw, f.pitch, None);
                }
            }

            ctl.pitch = ctl.pitch.clamp(PITCH_MIN, PITCH_TOP);
            ctl.yaw = wrap(ctl.yaw);
            ctl.focus = ctl.focus.clamp(Vec2::splat(-HALF_M), Vec2::splat(HALF_M));
            let focus = Vec3::new(ctl.focus.x, ground(ctl.focus.x, ctl.focus.y), ctl.focus.y);
            let (sy, cy, sp, cp) = (ctl.yaw.sin(), ctl.yaw.cos(), ctl.pitch.sin(), ctl.pitch.cos());
            let eye = focus + Vec3::new(sy * cp, sp, cy * cp) * ctl.dist;
            let floor = ground(eye.x, eye.z) + 10.0;
            *tf = if eye.y >= floor {
                // Built from the angles, not looking_at: stays defined when looking straight down.
                Transform::from_translation(eye).with_rotation(Quat::from_euler(EulerRot::YXZ, ctl.yaw, -ctl.pitch, 0.0))
            } else {
                Transform::from_translation(eye.with_y(floor)).looking_at(focus, Vec3::Y)
            };
        }
        CameraMode::Drone => {
            ctl.drone_yaw -= d.x * 0.004;
            ctl.drone_pitch -= d.y * 0.004;
            let turn = key(KeyCode::KeyQ) - key(KeyCode::KeyE) - arrows.x;
            ctl.drone_yaw = wrap(ctl.drone_yaw + turn * 1.6 * dt);
            ctl.drone_pitch = (ctl.drone_pitch + arrows.y * 1.0 * dt).clamp(-1.45, 1.45);
            ctl.drone_speed = (ctl.drone_speed * (pointer.zoom * 0.2).exp()).clamp(2.0, 500.0);
            let (fwd, right) = heading(ctl.drone_yaw);
            let up = key(KeyCode::KeyR) + key(KeyCode::PageUp) - key(KeyCode::KeyF) - key(KeyCode::PageDown);
            let speed = ctl.drone_speed * if shift { 4.0 } else { 1.0 };
            let step = (fwd * wasd.y + right * wasd.x + Vec3::Y * up) * speed * dt;
            let mut p = ctl.drone_pos + step;
            p.x = p.x.clamp(-HALF_M, HALF_M);
            p.z = p.z.clamp(-HALF_M, HALF_M);
            p.y = p.y.clamp(ground(p.x, p.z) + 2.0, 12_000.0);
            ctl.drone_pos = p;
            let look = Quat::from_euler(EulerRot::YXZ, ctl.drone_yaw, ctl.drone_pitch, 0.0);
            *tf = Transform::from_translation(p).with_rotation(look);
        }
        CameraMode::Rover => {
            let Ok(r) = rover.single() else { return };
            ctl.chase_yaw -= d.x * 0.005;
            ctl.chase_pitch = (ctl.chase_pitch + d.y * 0.004).clamp(0.05, 1.4);
            ctl.chase_dist = (ctl.chase_dist * (-pointer.zoom * 0.15).exp()).clamp(5.0, 200.0);
            let f = r.forward();
            let yaw = (-f.x).atan2(-f.z) + ctl.chase_yaw;
            let (sy, cy, sp, cp) = (yaw.sin(), yaw.cos(), ctl.chase_pitch.sin(), ctl.chase_pitch.cos());
            let target = r.translation + r.up() * 1.2 * r.scale.y;
            let mut eye = target + Vec3::new(sy * cp, sp, cy * cp) * ctl.chase_dist * r.scale.y;
            // Rise over terrain between the rover and the camera so the view is never inside a hill.
            for t in [0.25f32, 0.5, 0.75, 1.0] {
                let q = target.lerp(eye, t);
                let deficit = ground(q.x, q.z) + 0.8 - q.y;
                if deficit > 0.0 {
                    eye.y += deficit / t;
                }
            }
            let mut p = tf.translation.lerp(eye, 1.0 - (-6.0 * dt).exp());
            p.y = p.y.max(ground(p.x, p.z) + 0.8);
            tf.translation = p;
            tf.look_at(target, Vec3::Y);
        }
    }
}

/// Tells the page the map heading (degrees clockwise from north) and tilt (degrees from straight down),
/// for the compass and the 2D/3D button.
fn report_heading(mode: Res<CameraMode>, ctl: Res<CamCtl>, mut last: Local<Option<(f32, f32)>>) {
    if *mode != CameraMode::Map {
        return;
    }
    if mode.is_changed() {
        *last = None; // the page remounts its compass when the map view comes back
    }
    let heading = (-ctl.yaw.to_degrees()).rem_euclid(360.0) + 0.0; // + 0.0 turns -0.0 into 0.0
    let tilt = 90.0 - ctl.pitch.to_degrees();
    let changed = last.is_none_or(|(h, t)| wrap((heading - h).to_radians()).abs() > 0.01 || (tilt - t).abs() > 0.5);
    if changed {
        *last = Some((heading, tilt));
        crate::bridge::emit("mars-camera", format!(r#"{{"heading_deg":{heading:.1},"tilt_deg":{tilt:.1}}}"#));
    }
}
