use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use crate::{
    chunk::chunk_manager::ChunkManager,
    objects::{flag_pole::FlagPole, golfball::Golfball},
    state::{aim::AimState, state::AppState},
    ui::shoot_challenge::AimChallengeState,
};

const GLTF_PATH: &str = "model/tiksu.glb";

pub struct AimTiksuPlugin;
impl Plugin for AimTiksuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Aim), spawn_aim_tiksu)
            .add_systems(OnExit(AppState::Aim), remove_aim_tiksu)
            .add_systems(Update, update_aim_tiksu_position)
            .add_systems(Update, setup_scene_once_loaded)
            .add_systems(
                OnEnter(AimChallengeState::Finalized),
                let_tiksu_punch_the_ball,
            );
    }
}

#[derive(Component)]
pub struct AimTiksu;

#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph_handle: Handle<AnimationGraph>,
}

fn spawn_aim_tiksu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let (graph, indicies) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(GLTF_PATH)), // aim
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(GLTF_PATH)), // chopper
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(GLTF_PATH)), // shoot
    ]);
    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations: indicies,
        graph_handle,
    });

    let mesh_scene = SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH)));
    commands.spawn((
        AimTiksu,
        mesh_scene,
        Transform::default()
            .with_scale(Vec3::splat(0.5))
            .with_rotation(Quat::from_axis_angle(Vec3::Y, 0.5 * PI)),
    ));
}

fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph_handle.clone()))
            .insert(transitions);
    }
}

fn remove_aim_tiksu(mut commands: Commands, query: Query<Entity, With<AimTiksu>>) {
    for tiksu in query {
        commands.entity(tiksu).despawn();
    }
}

fn update_aim_tiksu_position(
    mut tiksu_transform: Single<
        &mut Transform,
        (With<AimTiksu>, Without<Golfball>, Without<FlagPole>),
    >,
    golfball_transform: Single<&Transform, (With<Golfball>, Without<AimTiksu>, Without<FlagPole>)>,
    flag_pole_transform: Single<&Transform, (With<FlagPole>, Without<AimTiksu>, Without<Golfball>)>,
    aim_state: If<Res<AimState>>,
    chunk_manager: If<Res<ChunkManager>>,
) {
    let looking_direction = (flag_pole_transform.translation - golfball_transform.translation)
        .normalize()
        .rotate_y(aim_state.rotation);
    let mut tiksu_position =
        golfball_transform.translation + looking_direction.rotate_y(0.5 * PI) * 1.0;
    tiksu_position.y = chunk_manager
        .generator
        .height_at(tiksu_position.x, tiksu_position.z)
        + 1.4;

    tiksu_transform.translation = tiksu_position;
    tiksu_transform.look_at(
        vec3(
            golfball_transform.translation.x,
            tiksu_position.y,
            golfball_transform.translation.z,
        ),
        Vec3::Y,
    );
}

fn let_tiksu_punch_the_ball(
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
) {
    for (mut player, mut transitions) in &mut animation_players {
        transitions.play(&mut player, animations.animations[2], Duration::ZERO);
        // transitions.play(&mut player, animations.animations[0], Duration::ZERO)
    }
}
