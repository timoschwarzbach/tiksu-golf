mod add_chunk_collider;
mod camera;
pub mod chunk;
pub mod generation;
mod golfball;
mod ui;
mod animation;

use crate::{
    add_chunk_collider::create_collider_from_mesh, camera::CameraPlugin, chunk::ChunkPlugin,
    golfball::GolfballPlugin,
};
use avian3d::PhysicsPlugins;
use bevy::light::CascadeShadowConfigBuilder;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins((CameraPlugin, GolfballPlugin, ChunkPlugin, ui::ui::UiPlugin, animation::AnimationPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, create_collider_from_mesh)
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
            maximum_distance: 1000.0, // Shadows stop at 100 units
            first_cascade_far_bound: 20.0, // First split near camera
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
