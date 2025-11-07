use crate::chunk::chunk_loader::ChunkLoader;
use bevy::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, input_handler);
    }
}

fn spawn_camera(mut commands: Commands) {
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_transform =
        Transform::from_xyz(128.0, 50.0, 128.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);

    // Camera in 3D space.
    commands.spawn((
        Camera3d::default(),
        camera_transform,
        ChunkLoader::new(512.0),
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
