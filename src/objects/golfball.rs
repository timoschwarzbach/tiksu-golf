use crate::chunk::chunk_loader::ChunkLoader;
use crate::{camera::ActiveCamera, state::state::AppState};
use avian3d::prelude::{
    AngularDamping, AngularInertia, CoefficientCombine, Collider, Friction, LinearDamping,
    LinearVelocity, Mass, Restitution, RigidBody,
};
use bevy::{color::palettes::css::WHITE, prelude::*};
use crate::chunk::chunk_manager::ChunkManager;
use crate::generation::grasslands::GrasslandsGenerator;
use crate::objects::flag_pole::FlagPole;

pub struct GolfballPlugin;
impl Plugin for GolfballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_golfball)
            .add_systems(
                Update,
                (input_handler, input_handler_golfball, update_rigid_mode, regenerate_after_hitting_hole),
            )
            .add_systems(OnEnter(AppState::InShot), set_ball_active)
            .add_systems(OnExit(AppState::InShot), set_ball_inactive);
    }
}

#[derive(Component)]
pub struct Golfball {
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
        Transform::from_xyz(0.0, 20.0, 0.0),
        Mesh3d(meshes.add(Sphere::new(radius).mesh().ico(5).unwrap())),
        MeshMaterial3d(materials.add(Color::from(WHITE))),
        ChunkLoader::new(32.0),
        RigidBody::Dynamic,
        Collider::sphere(radius),
        Mass(0.005),
        LinearVelocity::default(),
        AngularInertia::new(Vec3::splat(0.9)),
        AngularDamping(2.5),
        LinearDamping(0.01), // air resistance
        Friction {
            static_coefficient: 0.5,
            dynamic_coefficient: 1.0,
            combine_rule: CoefficientCombine::Average,
        }, // friction todo: dependent on ground
        Restitution::new(0.2),
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
        transform.translation = Vec3::new(0.0, 5.0, 0.0);
    }
}

fn input_handler_golfball(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut set: ParamSet<(
        Single<(&Golfball, &Transform)>,
        Single<&mut Transform, With<ActiveCamera>>,
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

fn set_ball_active(mut golfball: Single<&mut Golfball>) {
    golfball.active = true
}

fn set_ball_inactive(mut golfball: Single<&mut Golfball>) {
    golfball.active = false
}

fn regenerate_after_hitting_hole(
    mut golfball: Single<&mut Transform, (With<Golfball>, Without<FlagPole>)>,
    mut flag_pole: Single<&mut Transform, (With<FlagPole>, Without<Golfball>)>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let [hole_x, hole_z] = chunk_manager.generator.hole();
    let hole_y = chunk_manager.generator.height_at(hole_x, hole_z);

    let on_hole = (golfball.translation.x - hole_x).abs() <= 0.5
        && (golfball.translation.y - hole_y).abs() <= 0.2
        && (golfball.translation.z - hole_z).abs() <= 0.5;

    if !on_hole {
        return;
    }

    let seed = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() as u32;
    chunk_manager.replace_generator(&mut commands, Box::new(GrasslandsGenerator::new(seed)));

    let [start_x, start_z] = chunk_manager.generator.start();
    golfball.translation = Vec3::new(start_x, 10.0, start_z);

    let [hole_x, hole_z] = chunk_manager.generator.hole();
    let hole_y = chunk_manager.generator.height_at(hole_x, hole_z) + 0.5;
    flag_pole.translation = Vec3::new(hole_x, hole_y, hole_z);
}
