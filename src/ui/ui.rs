use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
};

use crate::ui::course_info::{CourseFlagPlugin, spawn_course_info};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_layout)
            .add_plugins(CourseFlagPlugin);
    }
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                            spawn_course_info(builder, asset_server);

                            // hole info
                            spawn_nested_text_bundle(
                                builder,
                                Color::Srgba(GREEN),
                                UiRect::default(),
                                "PAR4 363m\nREST 205m\nDOWN 1m",
                            );
                        });

                    // wind display
                    spawn_nested_text_bundle(
                        builder,
                        Color::Srgba(BLUE),
                        UiRect::default(),
                        "Wind Display",
                    );
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

fn spawn_nested_text_bundle(
    builder: &mut ChildSpawnerCommands,
    background_color: Color,
    margin: UiRect,
    text: &str,
) {
    builder
        .spawn((
            Node {
                margin,
                padding: UiRect::axes(px(5), px(1)),
                ..default()
            },
            BackgroundColor(background_color),
        ))
        .with_children(|builder| {
            builder.spawn((Text::new(text), TextFont { ..default() }, TextColor::BLACK));
        });
}
