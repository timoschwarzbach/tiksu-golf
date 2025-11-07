mod add_chunk_collider;
mod camera;
mod golfball;
mod noise;
pub mod chunk;

use crate::{
    add_chunk_collider::create_collider_from_mesh, camera::CameraPlugin, chunk::Chunk,
    golfball::GolfballPlugin,
};
use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use crate::chunk::{chunk_loader, chunk_manager};
use crate::chunk::chunk_manager::ChunkManager;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins((CameraPlugin, GolfballPlugin))
        .add_systems(Startup, setup)

        .add_systems(Startup, |mut commands: Commands| {
            commands.insert_resource(ChunkManager::new());
        })
        .add_systems(Update, chunk::insert_chunk_mesh)
        .add_systems(Update, chunk_manager::load_chunks)
        .add_systems(Update, chunk_manager::unload_chunks)
        .add_systems(Update, chunk_loader::update_chunk_loader_position)

        .add_systems(Update, create_collider_from_mesh)
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
