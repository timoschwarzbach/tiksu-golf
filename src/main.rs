mod add_chunk_collider;
mod camera;
pub mod chunk;
mod golfball;
mod noise;
mod ui;

use crate::{camera::CameraPlugin, chunk::ChunkPlugin, golfball::GolfballPlugin};
use avian3d::PhysicsPlugins;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins((CameraPlugin, GolfballPlugin, ChunkPlugin, ui::ui::UiPlugin))
        .add_systems(Startup, setup)
        // .add_systems(Update, create_collider_from_mesh)
        .run();
}

fn setup(mut commands: Commands) {
    let light_transform =
        Transform::from_xyz(0.0, 500.0, 2000.0).looking_at(Vec3::new(20.0, 50.0, 0.0), Vec3::Y);

    // Light up the scene.
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        light_transform,
    ));

    // Text to describe the controls.
    commands.spawn((
        Text::new("Controls:\nW/A/S/D: Move\nSpace: Move Up\nLShift: Move Down\nLCtrl: Speed Up\nArrows: Rotate"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}
