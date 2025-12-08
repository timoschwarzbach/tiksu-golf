use bevy::prelude::{Commands, Component, Entity, Query, Res, Time, Transform};

#[derive(Component)]
pub struct LiftUpAnimation {
    target_y: f32,
    time_left: f32,
}

impl LiftUpAnimation {
    pub fn new(target_y: f32, duration_seconds: f32) -> Self {
        LiftUpAnimation {
            target_y,
            time_left: duration_seconds,
        }
    }
}

pub(super) fn update_lift_up_animation(
    time: Res<Time>,
    query: Query<(Entity, &mut Transform, &mut LiftUpAnimation)>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut animation) in query {
        let animation = animation.into_inner();
        animation.time_left -= time.delta_secs();
        if animation.time_left <= 0.0 {
            transform.translation.y = animation.target_y;
            commands.entity(entity).remove::<LiftUpAnimation>();
        } else {
            let y = animation.target_y - animation.time_left * animation.time_left * 300.0;
            transform.translation.y = y;
        }
    }
}


#[derive(Component)]
pub struct LiftDownAnimation {
    origin_y: f32,
    duration_seconds: f32,
    seconds_passed: f32,
}

impl LiftDownAnimation {
    pub fn new(origin_y: f32, duration_seconds: f32) -> Self {
        LiftDownAnimation {
            origin_y,
            duration_seconds,
            seconds_passed: 0.0,
        }
    }
}

pub(super) fn update_lift_down_animation(
    time: Res<Time>,
    query: Query<(Entity, &mut Transform, &mut LiftDownAnimation)>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut animation) in query {
        let animation = animation.into_inner();
        animation.seconds_passed += time.delta_secs();

        let passed = animation.seconds_passed.min(animation.duration_seconds);

        let y = animation.origin_y - passed * passed * 300.0;
        transform.translation.y = y;

        if animation.seconds_passed >= animation.duration_seconds {
            commands.entity(entity).remove::<LiftDownAnimation>();
        }
    }
}
