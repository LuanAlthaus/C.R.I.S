//! Native-only shortcuts (the web build is driven by the page UI). WASD/QE/Shift are camera controls.

use bevy::prelude::*;

use crate::bridge::{SimCommand, push};
use crate::camera::CameraMode;
use crate::layers::Overlay;
use crate::rover::RoverPlayback;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, keys.after(bevy::input::InputSystems));
    }
}

fn keys(input: Res<ButtonInput<KeyCode>>, playback: Res<RoverPlayback>) {
    const DIGITS: [KeyCode; 9] = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];
    for (key, overlay) in DIGITS.iter().zip(Overlay::ALL) {
        if input.just_pressed(*key) {
            push(SimCommand::Overlay(overlay));
        }
    }
    for (key, mode) in [(KeyCode::F1, CameraMode::Map), (KeyCode::F2, CameraMode::Drone), (KeyCode::F3, CameraMode::Rover)] {
        if input.just_pressed(key) {
            push(SimCommand::Camera(mode));
        }
    }
    if input.just_pressed(KeyCode::Space) {
        push(SimCommand::Play(!playback.playing));
    }
    if input.just_pressed(KeyCode::Backspace) {
        push(SimCommand::ClearRoute);
    }
    if input.just_pressed(KeyCode::Home) {
        push(SimCommand::ResetView);
    }
}
