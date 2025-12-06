use crate::{chunk::chunk_loader::ChunkLoader, state::state::AppState};
use bevy::{
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            (
                input_handler.run_if(in_state(AppState::Debug)),
                asset_loaded,
            ),
        );
    }
}

#[derive(Component)]
pub struct ActiveCamera;

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_transform =
        Transform::from_xyz(128.0, 50.0, 128.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);

    // load skybox
    let skybox_handle = asset_server.load("textures/sky_36_2k.png");

    // Camera in 3D space.
    commands.spawn((
        Camera3d::default(),
        camera_transform,
        ActiveCamera,
        ChunkLoader::new(512.0),
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
            ..default()
        },
    ));

    // cubemap - we cannot use the skybox texture as is
    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
    });
}

fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, (With<Camera3d>, With<ActiveCamera>)>,
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
        movement *= 10.0;
    }

    camera.translation += movement * time.delta_secs() * 10.0;
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

// fix png before loading as skybox
// https://github.com/bevyengine/bevy/blob/main/examples/3d/skybox.rs
fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}
