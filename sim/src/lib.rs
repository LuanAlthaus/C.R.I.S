//! C.R.I.S — Bevy sim (3D terrain, overlays, EVA route planner, rover, cameras).
//! Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.

pub mod bridge;
pub mod camera;
pub mod data;
#[cfg(not(target_arch = "wasm32"))]
pub mod input;
pub mod layers;
pub mod picking;
pub mod rover;
pub mod route;
pub mod terrain;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

pub fn build_app() -> App {
    let mut app = App::new();
    // Detail tiles stream from public/data/jezero (served by Next at /data/jezero on the web).
    let file_path = if cfg!(target_arch = "wasm32") { "data/jezero" } else { "../public/data/jezero" };
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "C.R.I.S".into(),
                    canvas: Some("#mars-canvas".into()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin { file_path: file_path.into(), meta_check: AssetMetaCheck::Never, ..default() }),
    )
    .add_plugins((
        data::DataPlugin,
        terrain::TerrainPlugin,
        layers::LayersPlugin,
        route::RoutePlugin,
        picking::PickingPlugin,
        rover::RoverPlugin,
        camera::CameraPlugin,
        bridge::BridgePlugin,
    ));
    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(input::InputPlugin);
    app
}

pub fn run() {
    build_app().run();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
fn start() {
    run()
}
