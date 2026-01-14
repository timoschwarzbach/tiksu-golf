use std::time::Duration;

use bevy::{
    math::ops::{cos, sin},
    prelude::*,
};

use crate::{camera::ActiveCamera, chunk::chunk_manager::ChunkManager, state::state::AppState};

pub struct PresentCoursePlugin;
impl Plugin for PresentCoursePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            place_overview_camera.run_if(in_state(AppState::PresentCourse)),
        );
    }
}
fn place_overview_camera(
    chunk_manager: Res<ChunkManager>,
    mut camera_transform: Single<&mut Transform, With<ActiveCamera>>,
    time: Res<Time>,
    mut elapsed: Local<Duration>,
    mut state: ResMut<NextState<AppState>>,
) {
    let [start_x, start_z] = chunk_manager.generator.start();
    let [hole_x, hole_z] = chunk_manager.generator.hole();
    let middle_xz = (vec2(hole_x, hole_z) + vec2(start_x, start_z)) * 0.5;
    let middle_y = chunk_manager.generator.height_at(middle_xz.x, middle_xz.y);
    let middle = vec3(middle_xz.x, middle_y, middle_xz.y);

    let diameter = vec2(hole_x - start_x, hole_z - start_z).length() * 0.7;
    // circle around the middle
    camera_transform.translation = middle
        + vec3(
            diameter * sin(elapsed.as_secs_f32()),
            100.0,
            diameter * cos(elapsed.as_secs_f32()),
        );
    camera_transform.look_at(middle, Vec3::Y);

    *elapsed += time.delta();
    if *elapsed > Duration::from_secs(10) {
        *elapsed = Duration::ZERO;
        state.set(AppState::InShot); // this is deliberately wrong to show the ball :)
    }
}
