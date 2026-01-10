use std::f32::consts::PI;

use bevy::color::palettes::basic::RED;
use bevy::prelude::*;

use crate::{
    chunk::chunk_manager::ChunkManager,
    state::{aim::AimCamera, state::AppState},
};

pub struct FlagDirectionUiPlugin;
impl Plugin for FlagDirectionUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Aim), spawn_flag_direction_ui)
            .add_systems(OnExit(AppState::Aim), remove_flag_direction_ui)
            .add_systems(
                Update,
                update_flag_direction_ui_system.run_if(in_state(AppState::Aim)),
            );
    }
}

#[derive(Component)]
pub struct FlagDirectionUiContainer;
#[derive(Component)]
pub struct FlagDirectionUi;

pub fn spawn_flag_direction_ui(mut commands: Commands) {
    const MARGIN: Val = Val::Px(12.);
    commands
        .spawn((
            Node {
                width: percent(100),
                height: px(50),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(MARGIN),
                row_gap: MARGIN,
                position_type: PositionType::Relative,
                ..Default::default()
            },
            FlagDirectionUiContainer,
        ))
        .with_child((
            Node {
                width: px(50),
                height: px(50),
                position_type: PositionType::Absolute,
                margin: UiRect {
                    left: px(-25),
                    ..Default::default()
                },
                ..Default::default()
            },
            BackgroundColor(bevy::prelude::Color::Srgba(RED)),
            FlagDirectionUi,
        ));
}

pub fn remove_flag_direction_ui(
    mut commands: Commands,
    query: Query<Entity, With<FlagDirectionUiContainer>>,
) {
    for flag_direction_ui in query {
        commands.entity(flag_direction_ui).despawn();
    }
}

pub fn update_flag_direction_ui_system(
    chunk_manager: Option<Res<ChunkManager>>,
    camera: Single<&Transform, With<AimCamera>>,
    mut query: Query<&mut Node, With<FlagDirectionUi>>,
) {
    let Some(chunk_manager) = chunk_manager else {
        return;
    };

    let angle = get_flag_direction(&chunk_manager, &camera);
    for mut node in &mut query {
        let vw = (angle / PI) * 200.0 + 50.0;
        node.left = Val::Vw(vw);
    }
}
fn get_flag_direction(
    chunk_manager: &Res<ChunkManager>,
    camera: &Single<&Transform, With<AimCamera>>,
) -> f32 {
    let [hole_x, hole_z] = chunk_manager.generator.hole();
    let to_target = vec2(camera.translation.x, camera.translation.z) - vec2(hole_x, hole_z);
    let forward_3d = camera.back();
    let forward_2d = vec2(forward_3d.x, forward_3d.z);

    let angle = forward_2d.angle_to(to_target);
    angle
}
