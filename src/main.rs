mod add_chunk_collider;
mod chunk;
mod golfball;
mod noise;

use crate::{
    add_chunk_collider::create_collider_from_mesh, chunk::Chunk, golfball::GolfballPlugin,
};
use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use std::ops::{Deref, Rem, Sub};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(GolfballPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, input_handler)
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

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_transform =
        Transform::from_xyz(128.0, 50.0, 128.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);

    let light_transform =
        Transform::from_xyz(0.0, 500.0, 2000.0).looking_at(Vec3::new(20.0, 50.0, 0.0), Vec3::Y);

    // Camera in 3D space.
    commands.spawn((Camera3d::default(), camera_transform));

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

fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut movement = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Space) {
        movement += camera.local_y().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        movement -= camera.local_y().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        movement -= camera.local_z().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        movement -= camera.local_x().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement += camera.local_z().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        movement += camera.local_x().as_vec3();
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        let dir = camera.local_x();
        camera.rotate_axis(dir, time.delta_secs());
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        camera.rotate_y(time.delta_secs());
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        let dir = -camera.local_x();
        camera.rotate_axis(dir, time.delta_secs());
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        camera.rotate_y(-time.delta_secs());
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        movement *= 5.0;
    }

    camera.translation += movement * time.delta_secs() * 100.0;
}
