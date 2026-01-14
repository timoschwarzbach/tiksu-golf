use crate::camera::ActiveCamera;
use crate::objects::golfball::Golfball;
use crate::state::aim::AimState;
use crate::state::state::AppState;
use avian3d::prelude::ColliderDisabled;
use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::color::palettes::css::RED;
use bevy::light::{NotShadowCaster, NotShadowReceiver};
use bevy::mesh::Mesh3d;
use bevy::pbr::{ExtendedMaterial, MaterialExtension, StandardMaterial};
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use std::f32::consts::PI;

pub struct TrajectoryPlugin;

#[derive(Component)]
struct Trajectory;

impl Plugin for TrajectoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Aim), show_trajectory)
            .add_systems(Update, update_trajectory.run_if(in_state(AppState::Aim)))
            .add_systems(OnExit(AppState::Aim), hide_trajectory)
            .add_plugins(MaterialPlugin::<
                ExtendedMaterial<StandardMaterial, TrajectoryExtension>,
            >::default());
    }
}

fn show_trajectory(
    ball: Single<(&Golfball, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, TrajectoryExtension>>>,
    mut commands: Commands,
) {
    let height = 2.0;
    commands
        .spawn((
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
            Mesh3d(meshes.add(Cylinder::new(0.02, height))),
            MeshMaterial3d(materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: RED.into(),
                    unlit: true,
                    alpha_mode: AlphaMode::Blend,
                    ..Default::default()
                },
                extension: TrajectoryExtension {},
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
    direction.rotate_local_x((aim_state.height - 90.0) / 180.0 * PI);
    transform.rotation = direction.rotation;
}

fn hide_trajectory(mut commands: Commands, query: Query<Entity, With<Trajectory>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
struct TrajectoryExtension {}

const SHADER_ASSET_PATH: &str = "shaders/aim_ray.wgsl";

impl MaterialExtension for TrajectoryExtension {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
