mod fade;
mod lift;

use bevy::app::{App, Plugin, Update};
use crate::animation::fade::{update_fade_in_animation, update_fade_out_animation};

pub use fade::*;
pub use lift::*;
use crate::animation::lift::update_lift_up_animation;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_fade_in_animation)
            .add_systems(Update, update_fade_out_animation)
            .add_systems(Update, update_lift_up_animation)
            .add_systems(Update, update_lift_down_animation);
    }
}

