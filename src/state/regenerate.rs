use bevy::{ecs::system::SystemId, prelude::*};

use crate::{
    camera::ActiveCamera,
    chunk::chunk_manager::ChunkManager,
    generation::grasslands::GrasslandsGenerator,
    objects::{flag_pole::FlagPole, golfball::Golfball},
    state::state::AppState,
};

pub struct RegenPlugin;
impl Plugin for RegenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Regenerate),
            (place_regen_camera, delay_course_regen, delay_state_change),
        )
        .add_systems(
            Update,
            wait_for_delayed_systems.run_if(in_state(AppState::Regenerate)),
        );
    }
}

#[derive(Component)]
struct Callback(SystemId);

#[derive(Component)]
struct CallbackDelay(Timer);

fn delay_course_regen(mut commands: Commands) {
    let system_id = commands.register_system(regenerate_course);
    commands.spawn((
        CallbackDelay(Timer::from_seconds(5.0, TimerMode::Once)),
        Callback(system_id),
    ));
}

fn delay_state_change(mut commands: Commands) {
    let system_id = commands.register_system(continue_game);
    commands.spawn((
        CallbackDelay(Timer::from_seconds(10.0, TimerMode::Once)),
        Callback(system_id),
    ));
}

fn wait_for_delayed_systems(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CallbackDelay, &Callback)>,
    time: Res<Time>,
) {
    for (entity, mut delay, callback) in &mut query {
        if delay.0.tick(time.delta()).just_finished() {
            commands.run_system(callback.0);
            commands.entity(entity).despawn();
        }
    }
}

fn place_regen_camera(
    chunk_manager: Res<ChunkManager>,
    mut camera_transform: Single<&mut Transform, With<ActiveCamera>>,
) {
    let [start_x, start_z] = chunk_manager.generator.start();
    let [hole_x, hole_z] = chunk_manager.generator.hole();
    let middle_xz = (vec2(hole_x, hole_z) + vec2(start_x, start_z)) * 0.5;
    let middle_y = chunk_manager.generator.height_at(middle_xz.x, middle_xz.y);
    let middle = vec3(middle_xz.x, middle_y, middle_xz.y);
    camera_transform.translation = middle + vec3(0.0, 500.0, 0.0);
    camera_transform.look_at(middle, Vec3::Z);
}

fn regenerate_course(
    mut golfball: Single<&mut Transform, (With<Golfball>, Without<FlagPole>)>,
    mut flag_pole: Single<&mut Transform, (With<FlagPole>, Without<Golfball>)>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let seed = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() as u32;
    chunk_manager.replace_generator(&mut commands, Box::new(GrasslandsGenerator::new(seed)));

    let [start_x, start_z] = chunk_manager.generator.start();
    let start_y = chunk_manager.generator.height_at(start_x, start_z) + 0.5;
    golfball.translation = Vec3::new(start_x, start_y, start_z);

    let [hole_x, hole_z] = chunk_manager.generator.hole();
    let hole_y = chunk_manager.generator.height_at(hole_x, hole_z) + 0.5;
    flag_pole.translation = Vec3::new(hole_x, hole_y, hole_z);
}

fn continue_game(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::PresentCourse);
}
