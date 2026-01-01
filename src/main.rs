mod add_chunk_collider;
mod animation;
mod camera;
pub mod chunk;
pub mod generation;
mod objects;
mod state;
mod ui;
mod animation;

use crate::{
    add_chunk_collider::create_collider_from_mesh,
    camera::CameraPlugin,
    chunk::ChunkPlugin,
    objects::{flag_pole::FlagPolePlugin, golfball::GolfballPlugin},
    state::{
        aim::AimStatePlugin,
        state::{AppState, debug_state_change_input_handler},
    },
    ui::shoot_challenge::ShootChallengePlugin,
};
use avian3d::PhysicsPlugins;
use bevy::light::CascadeShadowConfigBuilder;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins((
            CameraPlugin,
            GolfballPlugin,
            ChunkPlugin,
            ui::ui::UiPlugin,
            animation::AnimationPlugin,
            AimStatePlugin,
            ShootChallengePlugin,
            FlagPolePlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, create_collider_from_mesh)
        .add_systems(Update, debug_state_change_input_handler) // change game states for debug
        .run();
}

fn setup(mut commands: Commands) {
    // Light up the scene.
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            maximum_distance: 1000.0,
            first_cascade_far_bound: 20.0,
            ..default()
        }.build(),
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.1, -0.1, 0.3), Vec3::Y),
    ));

    // Text to describe the controls.
    commands.spawn((
        Text::new("Controls:\nW/A/S/D: Move\nSpace: Move Up\nLShift: Move Down\nLCtrl: Speed Up\nArrows: Rotate"),
        Node {
            position_type: PositionType::Absolute,
            top: px(200),
            left: px(12),
            ..default()
        },
    ));
}
