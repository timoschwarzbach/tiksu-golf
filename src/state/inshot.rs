use crate::camera::ActiveCamera;
use crate::objects::flag_pole::FlagPole;
use crate::objects::golfball::Golfball;
use crate::state::state::AppState;
use bevy::app::{App, Update};
use bevy::prelude::*;

pub struct InShotPlugin;
impl Plugin for InShotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (camera_follow_golfball).run_if(in_state(AppState::InShot)),
        );
    }
}

fn camera_follow_golfball(
    mut set: ParamSet<(
        Single<(&Golfball, &Transform)>,
        Single<&mut Transform, With<ActiveCamera>>,
    )>,
    flag_pole: Single<(&FlagPole, &Transform), Without<ActiveCamera>>,
) {
    let flag_position = flag_pole.1;
    let flag_direction_vector = (flag_position.translation - set.p0().1.translation).normalize();
    let translation = set.p0().1.translation;
    let mut camera = set.p1();
    camera.translation = translation - flag_direction_vector * vec3(1.5, 1.5, 1.5);
    camera.translation.y += 0.5;
    camera.look_at(translation, Vec3::Y);
}
