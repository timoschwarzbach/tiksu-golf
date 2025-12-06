use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    Debug,
    // LevelTransition, -> Tell world generator to generate a new level and animate its creation
    PresentCourse, // -> Camera flyover over the course
    #[default]
    Aim, // -> aim, shoot, main in-game state
    InShot,        // -> camera following the golf ball, looking at it, etc.
    PostScore,     // -> after the ball has entered the hole tbd.
                   // Paused,
}
