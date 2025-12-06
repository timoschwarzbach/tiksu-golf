use bevy::app::{App, Plugin, Update};
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{Alpha, AlphaMode, Assets, Commands, Component, Entity, Query, Res, ResMut, StandardMaterial, Time};


pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_fade_in_animation)
            .add_systems(Update, update_fade_out_animation);
    }
}

#[derive(Component)]
pub struct FadeInAnimation {
    duration_seconds: f32,
    seconds_passed: f32,
}

impl FadeInAnimation {
    pub fn new(duration_seconds: f32) -> Self {
        FadeInAnimation {
            duration_seconds,
            seconds_passed: 0.0,
        }
    }
}

fn update_fade_in_animation(
    time: Res<Time>,
    query: Query<(Entity, &MeshMaterial3d<StandardMaterial>, &mut FadeInAnimation)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for (entity, material, mut animation) in query {
        animation.seconds_passed += time.delta_secs();
        let alpha = 1.0f32.min(animation.seconds_passed / animation.duration_seconds);
        if let Some(material) = materials.get_mut(material.id()) {
            material.base_color.set_alpha(alpha);
        }
        if animation.seconds_passed >= animation.duration_seconds {
            commands
                .entity(entity)
                .remove::<FadeInAnimation>();

            if let Some(material) = materials.get_mut(material.id()) {
                material.alpha_mode = AlphaMode::Opaque;
            }
        }
    }
}


#[derive(Component)]
pub struct FadeOutAnimation {
    duration_seconds: f32,
    seconds_passed: f32,
}

impl FadeOutAnimation {
    pub fn new(duration_seconds: f32) -> Self {
        FadeOutAnimation {
            duration_seconds,
            seconds_passed: 0.0,
        }
    }
}

fn update_fade_out_animation(
    time: Res<Time>,
    query: Query<(Entity, &MeshMaterial3d<StandardMaterial>, &mut FadeOutAnimation)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for (entity, material, mut animation) in query {
        animation.seconds_passed += time.delta_secs();
        let alpha = 0.0f32.max(1.0 - animation.seconds_passed / animation.duration_seconds);
        if let Some(material) = materials.get_mut(material.id()) {
            material.alpha_mode = AlphaMode::Blend;
            material.base_color.set_alpha(alpha);
        }
        if animation.seconds_passed >= animation.duration_seconds {
            commands
                .entity(entity)
                .remove::<FadeOutAnimation>();
        }
    }
}