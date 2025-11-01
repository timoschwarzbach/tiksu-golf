mod add_chunk_collider;
mod camera;
mod chunk;
mod golfball;
mod noise;

use crate::{
    add_chunk_collider::create_collider_from_mesh, camera::CameraPlugin, chunk::Chunk,
    golfball::GolfballPlugin,
};
use avian3d::PhysicsPlugins;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins((CameraPlugin, GolfballPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, chunk::insert_chunk_mesh)
        .add_systems(Update, create_collider_from_mesh)
        .run();
}

fn setup(mut commands: Commands) {
    for x in -5..=5 {
        for z in -5..=5 {
            commands.spawn(Chunk::generate_at([x * 256, z * 256]));
        }
    }

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
