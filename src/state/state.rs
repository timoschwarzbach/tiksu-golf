use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    Debug,
    // LevelTransition, -> Tell world generator to generate a new level and animate its creation
    #[default]
    Aim, // -> aim, shoot, main in-game state
    InShot,    // -> camera following the golf ball, looking at it, etc.
    PostScore, // -> after the ball has entered the hole tbd.
    Regenerate, // -> Camera over the course
               // Paused,
}

pub fn debug_state_change_input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit0) {
        next_state.set(AppState::Debug);
    }
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        next_state.set(AppState::Regenerate);
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        next_state.set(AppState::Aim);
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        next_state.set(AppState::InShot);
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        next_state.set(AppState::PostScore);
    }
}
