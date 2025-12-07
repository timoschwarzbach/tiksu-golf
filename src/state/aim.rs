use bevy::prelude::*;
use bevy::{
    app::{App, Update},
    state::state::OnExit,
};

use crate::camera::ActiveCamera;
use crate::golfball::Golfball;
use crate::state::state::AppState;
use crate::ui::course_info::FlagPole;
use crate::ui::shoot_challange::AimChallangeState;

pub struct AimStatePlugin;
impl Plugin for AimStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, set_aim_state.run_if(in_state(AppState::Aim)))
            .add_systems(
                Update,
                (input_handler)
                    .run_if(in_state(AppState::Aim))
                    .run_if(in_state(AimChallangeState::Idle)),
            )
            .add_systems(OnEnter(AppState::Aim), set_aim_state)
            .add_systems(OnExit(AppState::Aim), unset_aim_state);
    }
}

#[derive(Resource)]
struct AimState {
    selected_club: u8,
    zoomed: bool,
}

#[derive(Component)]
struct AimCamera;

fn set_aim_state(
    mut commands: Commands,
    camera: Single<(Entity, &ActiveCamera, &mut Transform)>,
    golfball: Single<(&Golfball, &Transform), Without<ActiveCamera>>,
    flag_pole: Single<(&FlagPole, &Transform), Without<ActiveCamera>>,
) {
    // get golfball position
    let golfball_position = golfball.1;
    // get flag pole position
    let flag_position = flag_pole.1;
    let flag_direction_vector =
        (flag_position.translation - golfball_position.translation).normalize();

    // set camera 5m in front of golfball looking at flag pole
    let camera_position = (-flag_direction_vector * 5.0) + vec3(0.0, 1.5, 0.0); // what if this is in the ground?
    let aim_camera_transform = Transform::from_translation(camera_position).looking_at(
        Vec3::new(
            flag_position.translation.x,
            camera_position.y,
            flag_position.translation.z,
        ),
        Vec3::Y,
    );

    let (camera_entity, _, mut camera_transform) = camera.into_inner();
    *camera_transform = aim_camera_transform;

    // update camera bundle
    commands.entity(camera_entity).insert(AimCamera);
    info!("set camera to aim position");

    // set up state resource
    commands.insert_resource(AimState {
        selected_club: 0,
        zoomed: false,
    });
}

fn unset_aim_state(mut commands: Commands, camera: Single<Entity, With<AimCamera>>) {
    // remove the AimCamera component when existing aim mode
    commands.entity(camera.entity()).remove::<AimCamera>();

    // remove aim state resource
    commands.remove_resource::<AimState>();
}

fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut aim_state: If<ResMut<AimState>>,
    mut camera: Single<&mut Transform, With<AimCamera>>,
    time: Res<Time>,
) {
    let multiplier = 0.25;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        camera.rotate_y(multiplier * time.delta_secs());
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        camera.rotate_y(-multiplier * time.delta_secs());
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        aim_state.zoomed = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        aim_state.zoomed = false;
    }
}
