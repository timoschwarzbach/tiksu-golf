use std::f32::consts::PI;

use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::{chunk::chunk_manager::ChunkManager, state::state::AppState};

const GLTF_PATH: &str = "model/tiksu.glb";

pub struct WinTiksuPlugin;
impl Plugin for WinTiksuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::PostScore), spawn_win_tiksu)
            .add_systems(OnExit(AppState::PostScore), remove_win_tiksu);
    }
}

#[derive(Component)]
struct WinTiksu;

#[derive(Component)]
struct AnimationToPlay {
    graph_handle: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}

fn spawn_win_tiksu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    chunk_manager: Res<ChunkManager>,
) {
    let (graph, index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(GLTF_PATH)), // chopper
    );
    let graph_handle = graphs.add(graph);
    let animation_to_play = AnimationToPlay {
        graph_handle,
        index,
    };

    let mesh_scene = SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH)));

    let mut tiksu_transform = Transform::default()
        .with_scale(Vec3::splat(0.5))
        .with_rotation(Quat::from_axis_angle(Vec3::Y, 0.5 * PI));

    let [start_x, start_z] = chunk_manager.generator.start();
    let [hole_x, hole_z] = chunk_manager.generator.hole();
    let hole_y = chunk_manager.generator.height_at(hole_x, hole_z);
    let hole = vec3(hole_x, hole_y + 1.0, hole_z);
    let hole_to_start = (vec3(hole_x, 0.0, hole_z) - vec3(start_x, 0.0, start_z)).normalize();
    let sideway = Vec3::Y.cross(hole_to_start).normalize();

    let tiksu_position_xz = hole + sideway * 3.0;
    let tiksu_position_y = chunk_manager
        .generator
        .height_at(tiksu_position_xz.x, tiksu_position_xz.z)
        + 1.4;
    let tiksu_position = vec3(tiksu_position_xz.x, tiksu_position_y, tiksu_position_xz.z);
    tiksu_transform.translation = tiksu_position;
    tiksu_transform.look_at(tiksu_position + hole_to_start, Vec3::Y);

    commands
        .spawn((WinTiksu, mesh_scene, tiksu_transform, animation_to_play))
        .observe(play_animation_when_ready);
}

fn play_animation_when_ready(
    scene_ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations_to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    if let Ok(animation_to_play) = animations_to_play.get(scene_ready.entity) {
        for child in children.iter_descendants(scene_ready.entity) {
            if let Ok(mut player) = players.get_mut(child) {
                player.play(animation_to_play.index);
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
    }
}

fn remove_win_tiksu(mut commands: Commands, query: Query<Entity, With<WinTiksu>>) {
    for tiksu in query {
        commands.entity(tiksu).despawn();
    }
}
