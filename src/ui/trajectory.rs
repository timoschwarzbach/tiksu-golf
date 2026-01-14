use std::f32::consts::PI;
use crate::camera::ActiveCamera;
use crate::objects::golfball::Golfball;
use crate::state::state::AppState;
use avian3d::prelude::ColliderDisabled;
use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::light::{NotShadowCaster, NotShadowReceiver};
use bevy::mesh::Mesh3d;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use crate::state::aim::AimState;

pub struct TrajectoryPlugin;

#[derive(Component)]
struct Trajectory;

impl Plugin for TrajectoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Aim), show_trajectory)
            .add_systems(Update, update_trajectory.run_if(in_state(AppState::Aim)))
            .add_systems(OnExit(AppState::Aim), hide_trajectory);
    }
}

fn show_trajectory(
    ball: Single<(&Golfball, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let height = 2.0;
    commands.spawn((
        Transform::from_xyz(
            ball.1.translation.x,
            ball.1.translation.y,
            ball.1.translation.z,
        ),
        NotShadowReceiver,
        NotShadowCaster,
        ColliderDisabled,
        Trajectory,
    ))
        .with_child((
            Mesh3d(meshes.add(Cylinder::new(0.01, height))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0),
                unlit: true,
                ..Default::default()
            })),
            Transform::from_xyz(0.0, height * 0.5, 0.0),
            NotShadowReceiver,
            NotShadowCaster,
            ColliderDisabled,
            ));
}

fn update_trajectory(
    aim_state: Res<AimState>,
    mut transform: Single<&mut Transform, (With<Trajectory>, Without<ActiveCamera>)>,
    camera: Single<&mut Transform, (With<ActiveCamera>, Without<Trajectory>)>,
) {
    let mut direction = camera.clone();
    direction.rotate_local_x((aim_state.height - 90.0) / 180.0 * PI );
    transform.rotation = direction.rotation;
}

fn hide_trajectory(mut commands: Commands, query: Query<Entity, With<Trajectory>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}
