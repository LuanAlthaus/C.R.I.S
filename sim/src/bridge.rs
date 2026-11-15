//! JS <-> sim bridge: wasm exports queue commands; the sim reports back with window CustomEvents.

use std::cell::RefCell;

use bevy::diagnostic::FrameCount;
use bevy::prelude::*;

use crate::camera::{CamCtl, CameraMode, ResetView};
use crate::data::VScale;
use crate::layers::{Overlay, ShowTraverse};
use crate::picking::PickTarget;
use crate::rover::RoverPlayback;
use crate::route::{Endpoints, RouteParams};
use crate::terrain::Terrain;

pub enum SimCommand {
    Overlay(Overlay),
    Camera(CameraMode),
    Params(RouteParams),
    Play(bool),
    /// Rover playback speed multiplier (1, 2 or 4).
    RoverSpeed(f32),
    ClearRoute,
    PickTarget(PickTarget),
    VerticalScale(f32),
    ShowTraverse(bool),
    ResetView,
    /// Map camera buttons: zoom steps (+ = in), north up, 2D (true) / 3D (false).
    MapZoom(f32),
    MapNorth,
    MapTopDown(bool),
}

thread_local! {
    static QUEUE: RefCell<Vec<SimCommand>> = const { RefCell::new(Vec::new()) };
}

pub fn push(cmd: SimCommand) {
    QUEUE.with(|q| q.borrow_mut().push(cmd));
}

/// Sends `json` as the `detail` of a window CustomEvent named `name` (logged on native).
pub fn emit(name: &str, json: String) {
    #[cfg(target_arch = "wasm32")]
    {
        let Some(window) = web_sys::window() else { return };
        let detail = js_sys::JSON::parse(&json).unwrap_or(wasm_bindgen::JsValue::NULL);
        let init = web_sys::CustomEventInit::new();
        init.set_detail(&detail);
        if let Ok(ev) = web_sys::CustomEvent::new_with_event_init_dict(name, &init) {
            let _ = window.dispatch_event(&ev);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    if name != "mars-hover" && name != "mars-pins" {
        info!("{name} {json}");
    }
}

pub struct BridgePlugin;

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, drain_commands).add_systems(Update, emit_ready);
    }
}

#[derive(bevy::ecs::system::SystemParam)]
struct Targets<'w> {
    overlay: ResMut<'w, Overlay>,
    mode: ResMut<'w, CameraMode>,
    params: ResMut<'w, RouteParams>,
    playback: ResMut<'w, RoverPlayback>,
    ends: ResMut<'w, Endpoints>,
    target: ResMut<'w, PickTarget>,
    vs: ResMut<'w, VScale>,
    traverse: ResMut<'w, ShowTraverse>,
    reset: ResMut<'w, ResetView>,
    cam: ResMut<'w, CamCtl>,
}

fn drain_commands(mut t: Targets) {
    for cmd in QUEUE.with(|q| std::mem::take(&mut *q.borrow_mut())) {
        match cmd {
            SimCommand::Overlay(o) => {
                t.overlay.set_if_neq(o);
            }
            SimCommand::Camera(m) => {
                t.mode.set_if_neq(m);
            }
            SimCommand::Params(p) => {
                t.params.set_if_neq(p);
            }
            SimCommand::Play(play) => t.playback.playing = play,
            SimCommand::RoverSpeed(v) => t.playback.speed = v.clamp(0.25, 8.0),
            SimCommand::ClearRoute => {
                *t.ends = Endpoints::default();
                *t.target = PickTarget::Start;
            }
            SimCommand::PickTarget(p) => *t.target = p,
            SimCommand::VerticalScale(v) => {
                t.vs.set_if_neq(VScale(v.clamp(1.0, 5.0)));
            }
            SimCommand::ShowTraverse(s) => t.traverse.0 = s,
            SimCommand::ResetView => {
                t.reset.0 = true;
                t.mode.set_if_neq(CameraMode::Map);
            }
            SimCommand::MapZoom(s) => t.cam.zoom_steps(s),
            SimCommand::MapNorth => t.cam.north_up(),
            SimCommand::MapTopDown(top) => t.cam.top_down(top),
        }
    }
}

fn emit_ready(mut done: Local<bool>, frames: Res<FrameCount>, terrain: Query<(), With<Terrain>>) {
    if !*done && frames.0 > 2 && !terrain.is_empty() {
        *done = true;
        emit("mars-sim-ready", "{}".into());
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn set_overlay(name: &str) {
        match Overlay::parse(name) {
            Some(o) => push(SimCommand::Overlay(o)),
            None => warn!("unknown overlay {name}"),
        }
    }

    #[wasm_bindgen]
    pub fn set_camera_mode(mode: &str) {
        match CameraMode::parse(mode) {
            Some(m) => push(SimCommand::Camera(m)),
            None => warn!("unknown camera mode {mode}"),
        }
    }

    #[wasm_bindgen]
    pub fn set_params(json: &str) {
        match serde_json::from_str::<RouteParams>(json) {
            Ok(p) => push(SimCommand::Params(p)),
            Err(e) => warn!("set_params: {e}"),
        }
    }

    #[wasm_bindgen]
    pub fn play_route(play: bool) {
        push(SimCommand::Play(play));
    }

    #[wasm_bindgen]
    pub fn set_rover_speed(speed: f32) {
        push(SimCommand::RoverSpeed(speed));
    }

    #[wasm_bindgen]
    pub fn clear_route() {
        push(SimCommand::ClearRoute);
    }

    #[wasm_bindgen]
    pub fn set_pick_target(which: &str) {
        match PickTarget::parse(which) {
            Some(p) => push(SimCommand::PickTarget(p)),
            None => warn!("unknown pick target {which}"),
        }
    }

    #[wasm_bindgen]
    pub fn set_vertical_scale(v: f32) {
        push(SimCommand::VerticalScale(v));
    }

    #[wasm_bindgen]
    pub fn set_show_traverse(show: bool) {
        push(SimCommand::ShowTraverse(show));
    }

    #[wasm_bindgen]
    pub fn reset_view() {
        push(SimCommand::ResetView);
    }

    #[wasm_bindgen]
    pub fn map_zoom(steps: f32) {
        push(SimCommand::MapZoom(steps));
    }

    #[wasm_bindgen]
    pub fn map_north() {
        push(SimCommand::MapNorth);
    }

    #[wasm_bindgen]
    pub fn map_top_down(top: bool) {
        push(SimCommand::MapTopDown(top));
    }
}
