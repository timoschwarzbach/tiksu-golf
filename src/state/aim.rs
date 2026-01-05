use avian3d::prelude::LinearVelocity;
use bevy::color::palettes::css::ORANGE_RED;
use bevy::color::palettes::tailwind::{BLUE_100, BLUE_300, BLUE_500, BLUE_700};
use bevy::prelude::*;
use bevy::{
    app::{App, Update},
    state::state::OnExit,
};

use crate::camera::ActiveCamera;
use crate::objects::flag_pole::FlagPole;
use crate::objects::golfball::Golfball;
use crate::state::state::AppState;
use crate::ui::shoot_challenge::{AimChallengeResource, AimChallengeState};

pub struct AimStatePlugin;
impl Plugin for AimStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, set_aim_state.run_if(in_state(AppState::Aim)))
            .add_systems(
                Update,
                (input_handler)
                    .run_if(in_state(AppState::Aim))
                    .run_if(in_state(AimChallengeState::Idle)),
            )
            .add_systems(OnEnter(AppState::Aim), set_aim_state)
            .add_systems(OnExit(AppState::Aim), unset_aim_state)
            .add_systems(
                OnEnter(AimChallengeState::Finalized),
                execute_golfball_punch,
            );
    }
}

#[derive(Resource)]
struct AimState {
    selected_club: u8,
    zoomed: bool,
}

#[derive(Component)]
pub struct AimCamera;

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
    let camera_position =
        golfball_position.translation + flag_direction_vector * -5.0 + vec3(0.0, 1.5, 0.0); // what if this is in the ground?
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

fn execute_golfball_punch(
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,

    aim_challenge_resource: Res<AimChallengeResource>,
    transform: Single<&Transform, With<AimCamera>>,
    mut next_aim_challenge_state: ResMut<NextState<AimChallengeState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut golfball_velocity: Single<&mut LinearVelocity, With<Golfball>>,
) {
    let mut missed = false;
    let power = aim_challenge_resource.power_marker.unwrap_or_default(); // 0 none ; 1 max
    let direction: Vec3 = transform.forward().as_vec3();
    let club_direction = Vec3::Y;
    let mut inaccuracies = Quat::IDENTITY;
    let mut deviation = Quat::IDENTITY;

    if let Some(precision) = aim_challenge_resource.precision_marker {
        if precision != 0.0 {
            let inaccuracy_yaw = rand::random_range(
                precision.abs() * -std::f32::consts::FRAC_PI_2
                    ..precision.abs() * std::f32::consts::FRAC_PI_2,
            );
            inaccuracies = Quat::from_rotation_y(inaccuracy_yaw);
            // 0 +- 0.1 is precise
            if precision.abs() > 0.1 {
                missed = true;
            }
        }
    }

    if missed {
        let random_yaw =
            rand::random_range(-std::f32::consts::FRAC_PI_2..std::f32::consts::FRAC_PI_2);
        deviation = Quat::from_rotation_y(random_yaw);
    }

    let combined_rotation = deviation * inaccuracies;
    let final_direction = combined_rotation * (club_direction + direction).normalize();

    let mut gizmo = GizmoAsset::default();
    gizmo.arrow(
        transform.translation,
        transform.translation + final_direction * Vec3::splat(power * 2.0),
        ORANGE_RED,
    );
    gizmo.arrow(
        transform.translation,
        transform.translation + direction,
        BLUE_100,
    );
    gizmo.arrow(
        transform.translation,
        transform.translation + (club_direction + direction).normalize(),
        BLUE_300,
    );
    gizmo.arrow(
        transform.translation,
        transform.translation + inaccuracies * (club_direction + direction).normalize(),
        BLUE_500,
    );
    gizmo.arrow(
        transform.translation,
        transform.translation + deviation * inaccuracies * (club_direction + direction).normalize(),
        BLUE_700,
    );
    commands.spawn(Gizmo {
        handle: gizmo_assets.add(gizmo),
        line_config: GizmoLineConfig {
            width: 4.,
            ..default()
        },
        ..default()
    });

    golfball_velocity.0 += final_direction * Vec3::splat(power * 100.0);
    // golfball_velocity.0 = vec3(final_angle.x, 0.0, final_angle.y);

    next_app_state.set(AppState::InShot);
    next_aim_challenge_state.set(AimChallengeState::Idle);
}
