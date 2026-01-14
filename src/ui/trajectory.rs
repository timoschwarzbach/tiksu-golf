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
use bevy::prelude::{
    Commands, Component, Cylinder, Entity, Mesh, MeshMaterial3d, OnEnter, OnExit, ParamSet, Query,
    ResMut, Single, Transform, With,
};

pub struct TrajectoryPlugin;

#[derive(Component)]
struct Trajectory;

impl Plugin for TrajectoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Aim), show_trajectory)
            .add_systems(OnExit(AppState::Aim), hide_trajectory);
    }
}

fn show_trajectory(
    mut set: ParamSet<(
        Single<(&Golfball, &Transform)>,
        Single<&mut Transform, With<ActiveCamera>>,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let height = 2.0;
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.01, height))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            unlit: true,
            ..Default::default()
        })),
        Transform::from_xyz(
            set.p0().1.translation.x,
            set.p0().1.translation.y + height * 0.5,
            set.p0().1.translation.z,
        ),
        NotShadowReceiver,
        NotShadowCaster,
        ColliderDisabled,
        Trajectory,
    ));
}

fn hide_trajectory(mut commands: Commands, query: Query<Entity, With<Trajectory>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}
