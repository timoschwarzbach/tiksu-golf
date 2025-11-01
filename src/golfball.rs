use avian3d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::chunk::Chunk;

pub struct GolfballPlugin;
impl Plugin for GolfballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_golfball)
            .add_systems(Update, input_handler)
            .add_systems(FixedUpdate, advance_physics_air);
    }
}

#[derive(Component)]
struct Golfball {
    airbone: bool,
}

struct PerformanceParamaters {
    wind_direction: Vec2,
    wind_speed: f32,
}

struct FrictionParamaters {
    ground_factor: f32,
}

fn spawn_golfball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.0021335;
    commands.spawn((
        Golfball { airbone: false },
        RigidBody::Dynamic,
        Collider::sphere(radius),
        LinearVelocity::default(),
        Mesh3d(meshes.add(Sphere::new(radius).mesh().ico(5).unwrap())),
        MeshMaterial3d(materials.add(Color::from(WHITE))),
        Transform::from_xyz(0.0, 20.0, 0.0),
    ));
}

fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    golfball: Single<(&Golfball, &mut LinearVelocity, &mut Transform)>,
) {
    let (_, mut velocity, mut transform) = golfball.into_inner();
    if keyboard_input.pressed(KeyCode::KeyT) {
        velocity.0 = Vec3::new(0.0, 15.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        // reset velocity and transform for debug
        velocity.0 = Vec3::splat(0.0);
        transform.translation = Vec3::new(0.0, 20.0, 0.0);
    }
}

fn advance_physics_air(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&Golfball, &mut LinearVelocity, &mut Transform)>,
) {
    let ground_accel = Vec3::new(0.0, -9.81, 0.0);
    let wind_accel = Vec3::new(0.0, 0.0, 0.0);
    let total_accel = ground_accel + wind_accel;

    for (_, mut velocity, mut transform) in
        query.iter_mut().filter(|(golfball, _, _)| golfball.airbone)
    {
        velocity.0 += total_accel * fixed_time.delta_secs();
        transform.translation += velocity.0 * fixed_time.delta_secs();
    }
}

fn check_ground_interaction(
    golfballs: Query<&Golfball, With<Mesh3d>>, // tood: check airborne
    ground_meshes: Query<(Entity, &Chunk), With<Mesh3d>>,
) {
    for golfball in golfballs.iter().filter(|golfball| !golfball.airbone) {}
}
