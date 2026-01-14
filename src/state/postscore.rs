use std::time::Duration;

use bevy::prelude::*;

use crate::{
    camera::ActiveCamera, chunk::chunk_manager::ChunkManager, objects::win_tiksu::WinTiksuPlugin,
    state::state::AppState,
};

pub struct PostScorePlugin;
impl Plugin for PostScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WinTiksuPlugin)
            .add_systems(OnEnter(AppState::PostScore), place_win_camera)
            .add_systems(
                Update,
                (slowly_move_back_system, wait_for_regeneration_system)
                    .run_if(in_state(AppState::PostScore)),
            );
    }
}

fn place_win_camera(
    chunk_manager: Res<ChunkManager>,
    mut camera_transform: Single<&mut Transform, With<ActiveCamera>>,
) {
    let [start_x, start_z] = chunk_manager.generator.start();
    let [hole_x, hole_z] = chunk_manager.generator.hole();
    let hole_y = chunk_manager.generator.height_at(hole_x, hole_z);
    let hole = vec3(hole_x, hole_y + 1.0, hole_z);
    let hole_to_start = (vec3(hole_x, 0.0, hole_z) - vec3(start_x, 0.0, start_z)).normalize();
    let sideway = Vec3::Y.cross(hole_to_start).normalize();

    let camera_position = hole + hole_to_start * 7.0 + vec3(0.0, 0.5, 0.0) + sideway;
    camera_transform.translation = camera_position;
    camera_transform.look_at(hole + sideway, Vec3::Y);
}

fn slowly_move_back_system(
    mut camera_transform: Single<&mut Transform, With<ActiveCamera>>,
    time: Res<Time>,
) {
    let back = camera_transform.back();
    camera_transform.translation += back.as_vec3() * time.delta_secs() * 0.4;
}

fn wait_for_regeneration_system(
    time: Res<Time>,
    mut duration: Local<Duration>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    *duration += time.delta();
    if *duration > Duration::from_secs(10) {
        *duration = Duration::ZERO;
        game_state.set(AppState::Regenerate);
    }
}
