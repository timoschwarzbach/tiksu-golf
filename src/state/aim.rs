use avian3d::prelude::{Forces, RigidBodyForces};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::{
    app::{App, Update},
    state::state::OnExit,
};

use crate::camera::ActiveCamera;
use crate::chunk::chunk_manager::ChunkManager;
use crate::generation::ZoneType;
use crate::objects::aim_tiksu::AimTiksuPlugin;
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
                (input_handler, aim_camera_position_system)
                    .run_if(in_state(AppState::Aim))
                    .run_if(in_state(AimChallengeState::Idle)),
            )
            .add_systems(OnEnter(AppState::Aim), set_aim_state)
            .add_systems(OnExit(AppState::Aim), unset_aim_state)
            .add_systems(
                OnEnter(AimChallengeState::Finalized),
                delay_golfball_execute,
            )
            .add_systems(
                Update,
                wait_for_golfball_punch_delay.run_if(in_state(AimChallengeState::Finalized)),
            )
            .add_plugins(AimTiksuPlugin);
    }
}

#[derive(Resource)]
pub struct AimState {
    pub selected_club: u8,
    pub rotation: f32,
    pub zoomed: bool,
}

#[derive(Component)]
pub struct AimCamera;

fn set_aim_state(mut commands: Commands, camera: Single<Entity, With<ActiveCamera>>) {
    // update camera bundle
    commands.entity(camera.entity()).insert(AimCamera);

    // set up state resource
    commands.insert_resource(AimState {
        selected_club: 0,
        rotation: 0.0,
        zoomed: false,
    });
}

fn aim_camera_position_system(
    mut camera: Single<&mut Transform, With<ActiveCamera>>,
    golfball: Single<(&Golfball, &Transform), Without<ActiveCamera>>,
    flag_pole: Single<(&FlagPole, &Transform), Without<ActiveCamera>>,
    aim_state: If<Res<AimState>>,
) {
    // get golfball position
    let golfball_position = golfball.1;
    // get flag pole position
    let flag_position = flag_pole.1;
    let look_direction_vector = (flag_position.translation - golfball_position.translation)
        .normalize()
        .rotate_y(aim_state.rotation);

    // set camera 5m in front of golfball looking at golfball
    let camera_position =
        golfball_position.translation + look_direction_vector * -5.0 + vec3(0.0, 1.5, 0.0); // what if this is in the ground?
    let aim_camera_transform = Transform::from_translation(camera_position).looking_at(
        Vec3::new(
            golfball_position.translation.x,
            camera_position.y,
            golfball_position.translation.z,
        ),
        Vec3::Y,
    );

    **camera = aim_camera_transform;
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
    time: Res<Time>,
) {
    let multiplier = 0.25;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        aim_state.rotation += multiplier * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        aim_state.rotation -= multiplier * time.delta_secs();
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        aim_state.zoomed = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        aim_state.zoomed = false;
    }
}

#[derive(Component)]
struct GolfballPunchDelay(Timer);

fn delay_golfball_execute(mut commands: Commands) {
    let system_id = commands.register_system(execute_golfball_punch);
    commands.spawn((
        GolfballPunchDelay(Timer::from_seconds(3.0, TimerMode::Once)),
        Callback(system_id),
    ));
}

#[derive(Component)]
struct Callback(SystemId);

fn wait_for_golfball_punch_delay(
    mut commands: Commands,
    mut query: Query<(Entity, &mut GolfballPunchDelay, &Callback)>,
    time: Res<Time>,
) {
    for (entity, mut delay, callback) in &mut query {
        if delay.0.tick(time.delta()).just_finished() {
            commands.run_system(callback.0);
            commands.entity(entity).despawn();
        }
    }
}

fn execute_golfball_punch(
    aim_challenge_resource: Res<AimChallengeResource>,
    transform: Single<&Transform, With<AimCamera>>,
    mut next_aim_challenge_state: ResMut<NextState<AimChallengeState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut golfball_forces: Single<Forces, With<Golfball>>,
    chunk_manager: Res<ChunkManager>,
) {
    let mut missed = false;
    let power = aim_challenge_resource.power_marker.unwrap_or_default(); // 0 none ; 1 max
    let direction: Vec3 = transform.forward().as_vec3();
    let club_direction = Vec3::Y * power;
    let mut inaccuracies = Quat::IDENTITY;
    let mut deviation = Quat::IDENTITY;

    if let Some(precision) = aim_challenge_resource.precision_marker {
        if precision != 0.0 {
            let inaccuracy_yaw = rand::random_range(
                precision.abs() * -std::f32::consts::FRAC_PI_2 * 0.2
                    ..precision.abs() * std::f32::consts::FRAC_PI_2 * 0.2,
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

    let zone_type = chunk_manager
        .generator
        .zone_type_at(transform.translation.x, transform.translation.z);
    let power_ground_multiplier = match zone_type {
        ZoneType::DeadZone => 0.0,
        ZoneType::Clean => rand::random_range(0.98..1.0),
        ZoneType::Offtrack => rand::random_range(0.8..0.9),
        ZoneType::Bunker => rand::random_range(0.2..0.4),
    };

    let force_vector = final_direction * Vec3::splat(power * power_ground_multiplier * 10.0);

    // wait for tiksu

    golfball_forces.apply_force(force_vector);
    next_app_state.set(AppState::InShot);
    next_aim_challenge_state.set(AimChallengeState::Idle);
}
