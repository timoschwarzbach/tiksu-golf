use bevy::{
    color::{Color, palettes::css::RED},
    prelude::*,
};

use crate::{
    chunk::chunk_manager::ChunkManager, generation::ZoneType, objects::golfball::Golfball,
    ui::ui::spawn_nested_text_bundle_with_bundle,
};

#[derive(Component)]
pub(super) struct GroundInfoContainer;
#[derive(Component)]
pub(super) struct GroundInfoText;

pub(super) fn spawn_ground_info_ui(builder: &mut ChildSpawnerCommands) {
    spawn_nested_text_bundle_with_bundle(
        builder,
        Color::Srgba(RED),
        UiRect::default(),
        "Ground Info\n0-100",
        GroundInfoContainer,
        GroundInfoText,
    );
}

pub(super) fn show_ground_info_ui_system(mut query: Query<&mut Node, With<GroundInfoContainer>>) {
    for mut node in &mut query {
        node.display = Display::Flex;
    }
}

pub(super) fn hide_ground_info_ui_system(mut query: Query<&mut Node, With<GroundInfoContainer>>) {
    for mut node in &mut query {
        node.display = Display::None;
    }
}

pub(super) fn update_ground_info_ui_system(
    chunk_manager: Option<Res<ChunkManager>>,
    golfball: Single<&Transform, With<Golfball>>,
    mut query: Query<&mut Text, With<GroundInfoText>>,
) {
    let Some(chunk_manager) = chunk_manager else {
        return;
    };

    for mut text in &mut query {
        let ground_info = get_ground_info(&chunk_manager, &golfball);

        **text = String::from(format!("Ground Info\n{}", ground_info,));
    }
}

fn get_ground_info(
    chunk_manager: &Res<ChunkManager>,
    golfball: &Single<&Transform, With<Golfball>>,
) -> &'static str {
    let zone_type = chunk_manager
        .generator
        .zone_type_at(golfball.translation.x, golfball.translation.z);
    match zone_type {
        ZoneType::Clean => "98-100",
        ZoneType::Offtrack => "60-80",
        ZoneType::Bunker => "20-40",
        ZoneType::DeadZone => "0",
    }
}
