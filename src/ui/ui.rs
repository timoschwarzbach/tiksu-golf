use crate::{
    state::state::AppState,
    ui::{
        course_info::{CourseFlagPlugin, spawn_course_info},
        distances::{spawn_distances_ui, update_distances_ui_system},
        flag_direction::FlagDirectionUiPlugin,
        wind_indicator::WindIndicatorPlugin,
    },
};
use bevy::{
    color::palettes::css::{BLUE, RED},
    prelude::*,
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_layout)
            .add_plugins((CourseFlagPlugin, WindIndicatorPlugin, FlagDirectionUiPlugin))
            .add_systems(OnEnter(AppState::Aim), update_distances_ui_system);
    }
}

fn spawn_layout(mut commands: Commands) {
    const MARGIN: Val = Val::Px(12.);
    commands
        .spawn(Node {
            // fill the entire window
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(MARGIN),
            row_gap: MARGIN,
            ..Default::default()
        })
        .with_children(|builder| {
            // Top Row
            builder
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    width: percent(100),
                    ..default()
                })
                .with_children(|builder| {
                    builder
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Start,
                            ..default()
                        })
                        .with_children(|builder| {
                            // course info
                            spawn_course_info(builder);

                            // hole info / distances
                            spawn_distances_ui(builder);
                        });

                    // wind display
                    // spawn_nested_text_bundle(
                    //     builder,
                    //     Color::Srgba(BLUE),
                    //     UiRect::default(),
                    //     "Wind Display",
                    // );
                });

            // bottom row
            builder
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    width: percent(100),
                    ..default()
                })
                .with_children(|builder| {
                    // selected drive
                    spawn_nested_text_bundle(
                        builder,
                        Color::Srgba(RED),
                        UiRect::default(),
                        "DriveSelection",
                    );

                    // ground info
                    spawn_nested_text_bundle(
                        builder,
                        Color::Srgba(RED),
                        UiRect::default(),
                        "Ground Info\n98-100",
                    );
                });
        });
}

fn spawn_child_node(
    builder: &mut ChildSpawnerCommands,
    align_items: AlignItems,
    justify_content: JustifyContent,
) {
    builder
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items,
                justify_content,
                width: percent(100),
                height: percent(100),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        ))
        .with_children(|builder| {
            let labels = [
                (format!("{align_items:?}"), Color::Srgba(RED), 0.),
                (format!("{justify_content:?}"), Color::Srgba(BLUE), 3.),
            ];
            for (text, color, top_margin) in labels {
                // We nest the text within a parent node because margins and padding can't be directly applied to text nodes currently.
                spawn_nested_text_bundle(builder, color, UiRect::top(px(top_margin)), &text);
            }
        });
}

pub(super) fn spawn_nested_text_bundle(
    builder: &mut ChildSpawnerCommands,
    background_color: Color,
    margin: UiRect,
    text: &str,
) {
    spawn_nested_text_bundle_with_bundle(builder, background_color, margin, text, (), ())
}

pub(super) fn spawn_nested_text_bundle_with_bundle<A: Bundle, B: Bundle>(
    builder: &mut ChildSpawnerCommands,
    background_color: Color,
    margin: UiRect,
    text: &str,
    bundle_components: A,
    child_bundle_components: B,
) {
    builder
        .spawn((
            Node {
                margin,
                padding: UiRect::axes(px(5), px(1)),
                ..default()
            },
            BackgroundColor(background_color),
            bundle_components,
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new(text),
                TextFont { ..default() },
                TextColor::BLACK,
                child_bundle_components,
            ));
        });
}
