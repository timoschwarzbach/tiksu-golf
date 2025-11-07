use crate::chunk::chunk_loader::ChunkLoader;
use avian3d::prelude::{Collider, Friction, LinearDamping, LinearVelocity, RigidBody};
use bevy::{color::palettes::css::WHITE, prelude::*};

pub struct GolfballPlugin;
impl Plugin for GolfballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_golfball).add_systems(
            Update,
            (input_handler, input_handler_golfball, update_rigid_mode),
        );
    }
}

#[derive(Component)]
struct Golfball {
    active: bool,
}

fn spawn_golfball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.021335;
    commands.spawn((
        Golfball { active: true },
        RigidBody::Dynamic,
        Collider::sphere(radius),
        LinearVelocity::default(),
        LinearDamping(0.8), // air resistance
        Friction::new(0.8), // friction todo: dependent on ground
        Mesh3d(meshes.add(Sphere::new(radius).mesh().ico(5).unwrap())),
        MeshMaterial3d(materials.add(Color::from(WHITE))),
        Transform::from_xyz(0.0, 20.0, 0.0),
        ChunkLoader::new(32.0),
    ));
}

fn input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    golfball: Single<(&Golfball, &mut LinearVelocity, &mut Transform)>,
) {
    let (_, mut velocity, mut transform) = golfball.into_inner();
    if keyboard_input.pressed(KeyCode::KeyT) {
        velocity.0 = Vec3::new(20.0, 15.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        // reset velocity and transform for debug
        velocity.0 = Vec3::splat(0.0);
        transform.translation = Vec3::new(0.0, 20.0, 0.0);
    }
}

fn input_handler_golfball(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut set: ParamSet<(
        Single<(&Golfball, &Transform)>,
        Single<&mut Transform, With<Camera3d>>,
    )>,
) {
    // golf ball debug
    if keyboard_input.pressed(KeyCode::KeyF) {
        let translation = set.p0().1.translation;
        let mut camera = set.p1();
        camera.translation = translation + Vec3::new(5.0, 0.1, 0.0);
        camera.look_at(translation, Vec3::Y);
    }
}

fn update_rigid_mode(mut commands: Commands, query: Query<(Entity, &Golfball, &RigidBody)>) {
    for (entity, golfball, ridgid_body) in query {
        commands
            .entity(entity)
            .insert_if(RigidBody::Dynamic, || {
                golfball.active && ridgid_body.ne(&RigidBody::Dynamic)
            })
            .insert_if(RigidBody::Static, || {
                !golfball.active && ridgid_body.ne(&RigidBody::Static)
            });
    }
}
