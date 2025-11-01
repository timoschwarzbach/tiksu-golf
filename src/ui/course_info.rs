use bevy::prelude::*;
use bevy::ui::{Node, widget::ImageNode};

pub(super) fn spawn_course_info(
    builder: &mut ChildSpawnerCommands,
    asset_server: Res<AssetServer>,
) {
    let image = asset_server.load("image/course_flag.png");
    builder.spawn((
        ImageNode::new(image.clone()),
        Node {
            min_width: px(100),
            min_height: px(100),
            ..default()
        },
    ));
}
