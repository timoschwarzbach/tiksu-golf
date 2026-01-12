use bevy::{
    color::{Color, palettes::css::GREEN},
    prelude::*,
};

use crate::{
    chunk::chunk_manager::ChunkManager, objects::golfball::Golfball,
    ui::ui::spawn_nested_text_bundle_with_bundle,
};

#[derive(Component)]
pub(super) struct DistancesText;

pub(super) fn spawn_distances_ui(builder: &mut ChildSpawnerCommands) {
    spawn_nested_text_bundle_with_bundle(
        builder,
        Color::Srgba(GREEN),
        UiRect::default(),
        "PAR4 363m\nREST 205m\nDOWN 1m",
        (),
        DistancesText,
    );
}

pub(super) fn update_distances_ui_system(
    chunk_manager: Option<Res<ChunkManager>>,
    golfball: Single<&Transform, With<Golfball>>,
    mut query: Query<&mut Text, With<DistancesText>>,
) {
    let Some(chunk_manager) = chunk_manager else {
        return;
    };

    for mut text in &mut query {
        let course_length = get_current_course_length(&chunk_manager);
        let (remaining_dist, remaining_height) =
            get_remaining_course_dist(&chunk_manager, &golfball);

        let (height_label, height_val) = if remaining_height > 0.0 {
            ("DOWN", remaining_height)
        } else {
            ("UP", remaining_height.abs())
        };
        **text = String::from(format!(
            "PAR4 {}m\nREST {}m\n{height_label} {}m",
            course_length.round(),
            remaining_dist.round(),
            height_val.round()
        ));
    }
}

fn get_current_course_length(chunk_manager: &Res<ChunkManager>) -> f32 {
    let [hole_x, hole_z] = chunk_manager.generator.hole();
    let [start_x, start_z] = chunk_manager.generator.start();
    vec2(start_x, start_z).distance(vec2(hole_x, hole_z))
}

fn get_remaining_course_dist(
    chunk_manager: &Res<ChunkManager>,
    golfball: &Single<&Transform, With<Golfball>>,
) -> (f32, f32) {
    let [hole_x, hole_z] = chunk_manager.generator.hole();
    let hole_y = chunk_manager.generator.height_at(hole_x, hole_z) + 0.5;
    let remaining_dist =
        vec2(golfball.translation.x, golfball.translation.z).distance(vec2(hole_x, hole_y));
    let height_diff = golfball.translation.y - hole_y;
    (remaining_dist, height_diff)
}
