//! Pointer handling shared by picking and cameras (mouse + touch), click-vs-drag detection, hover preview,
//! explicit start/end pick target and distance-scaled pins.

use bevy::input::mouse::{AccumulatedMouseScroll, MouseScrollUnit};
use bevy::input::touch::Touches;
use bevy::light::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera::MainCamera;
use crate::data::{HALF_M, N, Site, VScale};
use crate::route::{Endpoints, Grid, RouteParams};
use crate::terrain::Surface;

/// Pixels the pointer may move between press and release and still count as a click.
const CLICK_SLOP_PX: f32 = 6.0;
/// A second click within this time and distance makes a double click (map zoom) instead of two picks.
const DOUBLE_CLICK_S: f64 = 0.3;
const DOUBLE_CLICK_PX: f32 = 12.0;
const HOVER_EVERY_S: f32 = 0.08;
/// Search radius for the nearest Perseverance waypoint reported with a pick.
const WAYPOINT_RADIUS_M: f32 = 400.0;

#[derive(Resource, Default)]
pub struct Pointer {
    pub pos: Option<Vec2>,
    /// Cursor movement this frame (px).
    pub delta: Vec2,
    pub left: bool,
    pub right: bool,
    pub dragging: bool,
    /// Where the current press started.
    pub down_at: Option<Vec2>,
    /// A single click, reported once the double-click window has passed.
    pub click: Option<Vec2>,
    pub double_click: Option<Vec2>,
    /// Zoom steps this frame (wheel lines or pinch), positive = zoom in.
    pub zoom: f32,
    /// Two-finger gestures this frame: twist (rad, counter-clockwise on screen) and midpoint drag (px).
    pub twist: f32,
    pub two_finger: Vec2,
    pending_click: Option<(Vec2, f64)>,
    pinch: Option<(f32, f32, Vec2)>,
}

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum PickTarget {
    #[default]
    Start,
    End,
}

impl PickTarget {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "start" => PickTarget::Start,
            "end" => PickTarget::End,
            _ => return None,
        })
    }
}

#[derive(Component)]
struct Pin;

#[derive(Component)]
struct Ghost;

#[derive(Resource)]
struct PinAssets {
    pole: Handle<Mesh>,
    head: Handle<Mesh>,
    start: Handle<StandardMaterial>,
    end: Handle<StandardMaterial>,
    ghost_start: Handle<StandardMaterial>,
    ghost_end: Handle<StandardMaterial>,
    ghost_blocked: Handle<StandardMaterial>,
}

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Pointer>()
            .init_resource::<PickTarget>()
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, update_pointer.after(bevy::input::InputSystems))
            .add_systems(Update, (pick, hover, sync_pins, scale_pins, emit_target).chain());
    }
}

fn update_pointer(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    scroll: Res<AccumulatedMouseScroll>,
    touches: Res<Touches>,
    time: Res<Time>,
    mut p: ResMut<Pointer>,
) {
    let now = time.elapsed_secs_f64();
    let active: Vec<Vec2> = touches.iter().map(|t| t.position()).collect();
    let (pos, left) = match active.as_slice() {
        [t] => (Some(*t), true),
        [a, b, ..] => (Some((*a + *b) / 2.0), false),
        [] => (windows.single().ok().and_then(Window::cursor_position), mouse.pressed(MouseButton::Left)),
    };
    let right = active.is_empty() && mouse.pressed(MouseButton::Right);
    let was_down = p.left || p.right;
    let down = left || right;

    p.delta = match (pos, p.pos) {
        (Some(a), Some(b)) if was_down && down => a - b,
        _ => Vec2::ZERO,
    };
    p.click = None;
    p.double_click = None;
    // Use input edges, not sampled state: a quick click can press and release within one (slow) frame.
    let pressed = mouse.any_just_pressed([MouseButton::Left, MouseButton::Right]) || touches.any_just_pressed();
    let released_left = mouse.just_released(MouseButton::Left) || (touches.any_just_released() && active.is_empty());
    let released = released_left || mouse.just_released(MouseButton::Right);
    if pressed {
        p.down_at = pos;
        p.dragging = false;
    }
    if down && let (Some(a), Some(b)) = (pos, p.down_at) {
        p.dragging |= a.distance(b) > CLICK_SLOP_PX;
    }
    if released {
        if released_left && !p.dragging && let Some(at) = pos.or(p.pos) {
            match p.pending_click.take() {
                Some((first, t)) if now - t < DOUBLE_CLICK_S && first.distance(at) < DOUBLE_CLICK_PX => {
                    p.double_click = Some(at);
                }
                _ => p.pending_click = Some((at, now)),
            }
        }
        p.dragging = false;
        p.down_at = None;
    }
    if let Some((at, t)) = p.pending_click
        && now - t >= DOUBLE_CLICK_S
    {
        p.click = Some(at);
        p.pending_click = None;
    }

    p.zoom = match scroll.unit {
        MouseScrollUnit::Line => scroll.delta.y,
        MouseScrollUnit::Pixel => scroll.delta.y / 100.0,
    };
    (p.twist, p.two_finger) = (0.0, Vec2::ZERO);
    if let [a, b, ..] = active.as_slice() {
        let (d, angle, mid) = (a.distance(*b), (*b - *a).to_angle(), (*a + *b) / 2.0);
        if let Some((prev_d, prev_angle, prev_mid)) = p.pinch {
            p.zoom += (d - prev_d) / 40.0;
            // Screen y points down, so a counter-clockwise twist lowers the angle.
            let mut da = prev_angle - angle;
            if da > std::f32::consts::PI {
                da -= std::f32::consts::TAU;
            } else if da < -std::f32::consts::PI {
                da += std::f32::consts::TAU;
            }
            p.twist = da;
            p.two_finger = mid - prev_mid;
        }
        p.pinch = Some((d, angle, mid));
        p.dragging = true; // a pinch never ends in a click
    } else {
        p.pinch = None;
    }
    // A touch release has no position on the release frame; keep the last one for the click.
    if pos.is_some() || !was_down {
        p.pos = pos;
    }
    p.left = left;
    p.right = right;
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let solid = |c: Color| StandardMaterial { base_color: c, unlit: true, ..default() };
    let ghost = |c: Color| StandardMaterial {
        base_color: c.with_alpha(0.55),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    };
    let (green, red, purple) =
        (Color::srgb_u8(0x2e, 0x9e, 0x5b), Color::srgb_u8(0xd6, 0x45, 0x2f), Color::srgb_u8(0x6b, 0x2a, 0x8c));
    let assets = PinAssets {
        pole: meshes.add(Cylinder::new(0.35, 10.0)),
        head: meshes.add(Sphere::new(2.2)),
        start: materials.add(solid(green)),
        end: materials.add(solid(red)),
        ghost_start: materials.add(ghost(green)),
        ghost_end: materials.add(ghost(red)),
        ghost_blocked: materials.add(ghost(purple)),
    };
    commands
        .spawn((Ghost, Pin, Transform::default(), Visibility::Hidden))
        .with_children(|p| pin_parts(p, &assets, assets.ghost_start.clone()));
    commands.insert_resource(assets);
}

fn pin_parts(p: &mut ChildSpawnerCommands, a: &PinAssets, mat: Handle<StandardMaterial>) {
    p.spawn((Mesh3d(a.pole.clone()), MeshMaterial3d(mat.clone()), Transform::from_xyz(0.0, 5.0, 0.0), NotShadowReceiver));
    p.spawn((Mesh3d(a.head.clone()), MeshMaterial3d(mat), Transform::from_xyz(0.0, 11.0, 0.0), NotShadowReceiver, NotShadowCaster));
}

/// First intersection of `ray` with the rendered (exaggerated) terrain; `None` if it misses the site.
/// The returned height is unexaggerated.
pub fn hit_terrain(surface: &Surface, vs: VScale, ray: Ray3d) -> Option<Vec3> {
    let inside = |p: Vec3| p.x.abs() < HALF_M && p.z.abs() < HALF_M;
    let above = |p: Vec3| p.y - surface.ground(p.x, p.z, vs);
    let mut t = 0.0;
    while t < 40_000.0 {
        let p = ray.get_point(t);
        // Step proportional to the height above ground: fast when high, fine near the surface.
        let step = (above(p) * 0.3).max(1.0);
        let q = ray.get_point(t + step);
        if inside(q) && above(q) <= 0.0 {
            let (mut lo, mut hi) = (t, t + step);
            for _ in 0..14 {
                let mid = (lo + hi) / 2.0;
                if above(ray.get_point(mid)) <= 0.0 { hi = mid } else { lo = mid }
            }
            let h = ray.get_point(hi);
            return Some(Vec3::new(h.x, surface.height(h.x, h.z), h.z));
        }
        t += step;
    }
    None
}

fn cursor_hit(pos: Vec2, cams: &Query<(&Camera, &GlobalTransform), With<MainCamera>>, surface: &Surface, vs: VScale) -> Option<Vec3> {
    let (cam, tf) = cams.single().ok()?;
    hit_terrain(surface, vs, cam.viewport_to_world(tf, pos).ok()?)
}

/// Point readout shared by hover and pick events.
fn point_json(site: &Site, grid: &Grid, params: &RouteParams, p: Vec3) -> serde_json::Value {
    let (lat, lon) = site.latlon_of(p.x, p.z);
    let cell = site.cell_of(p.x, p.z);
    let slope = cell.map_or(0.0, |(r, c)| site.slope_deg[r * N + c]);
    let passable = cell.is_some_and(|(r, c)| grid.0.multiplier(r * N + c, params).is_some());
    serde_json::json!({
        "lat": lat, "lon": lon, "elevation_m": p.y + site.h_mid(), "slope_deg": slope, "passable": passable,
    })
}

#[allow(clippy::too_many_arguments)]
fn pick(
    pointer: Res<Pointer>,
    cams: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    site: Res<Site>,
    surface: Res<Surface>,
    vs: Res<VScale>,
    grid: Res<Grid>,
    params: Res<RouteParams>,
    mut target: ResMut<PickTarget>,
    mut ends: ResMut<Endpoints>,
) {
    let Some(pos) = pointer.click else { return };
    let Some(p) = cursor_hit(pos, &cams, &surface, *vs) else { return };
    let which = match *target {
        PickTarget::Start => {
            ends.start = Some(p);
            if ends.end.is_none() {
                *target = PickTarget::End;
            }
            "start"
        }
        PickTarget::End => {
            ends.end = Some(p);
            "end"
        }
    };
    let mut json = point_json(&site, &grid, &params, p);
    json["which"] = which.into();
    let near = site.nearest_waypoint(p.x, p.z, WAYPOINT_RADIUS_M);
    json["nearest_sol"] = near.map(|n| n.0).into();
    json["nearest_sol_dist_m"] = near.map(|n| n.1.round()).into();
    crate::bridge::emit("mars-pick", json.to_string());
}

#[allow(clippy::too_many_arguments)]
fn hover(
    pointer: Res<Pointer>,
    time: Res<Time>,
    cams: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    site: Res<Site>,
    surface: Res<Surface>,
    vs: Res<VScale>,
    grid: Res<Grid>,
    params: Res<RouteParams>,
    target: Res<PickTarget>,
    assets: Res<PinAssets>,
    mut last: Local<(f32, Option<Vec2>)>,
    mut ghost: Query<(&mut Transform, &mut Visibility, &Children), With<Ghost>>,
    mut mats: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    let Ok((mut tf, mut vis, children)) = ghost.single_mut() else { return };
    let now = time.elapsed_secs();
    if pointer.dragging || pointer.pos.is_none() {
        *vis = Visibility::Hidden;
        return;
    }
    if pointer.pos == last.1 || now - last.0 < HOVER_EVERY_S {
        return;
    }
    *last = (now, pointer.pos);
    let Some(p) = pointer.pos.and_then(|pos| cursor_hit(pos, &cams, &surface, *vs)) else {
        *vis = Visibility::Hidden;
        return;
    };
    let json = point_json(&site, &grid, &params, p);
    let mat = match (json["passable"].as_bool(), *target) {
        (Some(false), _) => &assets.ghost_blocked,
        (_, PickTarget::Start) => &assets.ghost_start,
        (_, PickTarget::End) => &assets.ghost_end,
    };
    for c in children.iter() {
        if let Ok(mut m) = mats.get_mut(c) {
            m.0 = mat.clone();
        }
    }
    tf.translation = Vec3::new(p.x, p.y * vs.0, p.z);
    *vis = Visibility::Visible;
    crate::bridge::emit("mars-hover", json.to_string());
}

#[derive(Component)]
struct EndpointPin;

fn sync_pins(
    mut commands: Commands,
    ends: Res<Endpoints>,
    vs: Res<VScale>,
    assets: Res<PinAssets>,
    pins: Query<Entity, With<EndpointPin>>,
) {
    if !ends.is_changed() && !vs.is_changed() {
        return;
    }
    for e in &pins {
        commands.entity(e).despawn();
    }
    for (p, mat) in [(ends.start, &assets.start), (ends.end, &assets.end)] {
        if let Some(p) = p {
            commands
                .spawn((EndpointPin, Pin, Transform::from_xyz(p.x, p.y * vs.0, p.z), Visibility::default()))
                .with_children(|c| pin_parts(c, &assets, mat.clone()));
        }
    }
}

/// Keep pins a readable size on screen: scale with camera distance.
fn scale_pins(cams: Query<&GlobalTransform, With<MainCamera>>, mut pins: Query<&mut Transform, With<Pin>>) {
    let Ok(cam) = cams.single() else { return };
    for mut tf in &mut pins {
        let d = cam.translation().distance(tf.translation);
        tf.scale = Vec3::splat((d / 300.0).clamp(0.12, 30.0));
    }
}

fn emit_target(target: Res<PickTarget>) {
    if target.is_changed() {
        let t = match *target {
            PickTarget::Start => "start",
            PickTarget::End => "end",
        };
        crate::bridge::emit("mars-pick-target", format!(r#"{{"target":"{t}"}}"#));
    }
}
