use bevy::asset::RenderAssetUsages;
use bevy::camera::RenderTarget;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureDimension, TextureFormat, TextureUsages};
use bevy::scene::SceneInstanceReady;
use bevy::ui::Node;

use crate::camera::ActiveCamera;

const GLTF_PATH: &str = "model/wind_indicator.glb";
const WIND_INDICATOR_LAYER: RenderLayers = RenderLayers::layer(1);

#[derive(Component)]
struct WindIndicator;

pub(super) struct WindIndicatorPlugin;
impl Plugin for WindIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_wind_indicator_position);
    }
}
fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let mut image = Image::new_uninit(
        default(),
        TextureDimension::D2,
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::all(),
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    let image_handle = images.add(image);

    let camera = commands
        .spawn((
            Camera3d::default(),
            Camera {
                order: -2, // hidden under everything else
                clear_color: ClearColorConfig::Custom(Color::NONE),
                target: RenderTarget::Image(image_handle.clone().into()),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -5.0).looking_at(Vec3::splat(0.0), Vec3::Y),
            WIND_INDICATOR_LAYER,
        ))
        .id();

    // spawn tiksu
    let mesh_scene = SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH)));
    commands
        .spawn((
            WindIndicator,
            mesh_scene,
            Transform::from_xyz(0.0, -1.0, 0.0).with_scale(Vec3::splat(1.0)),
            WIND_INDICATOR_LAYER,
        ))
        .observe(apply_render_layers_to_children);

    // spawn ui element
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(0),
            right: px(0),
            width: px(200),
            height: px(200),
            ..default()
        },
        ViewportNode::new(camera),
    ));
}

// https://github.com/bevyengine/bevy/issues/12461
fn apply_render_layers_to_children(
    trigger: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    transforms: Query<&Transform, Without<RenderLayers>>,
    query: Query<(Entity, &RenderLayers)>,
) {
    let Ok((parent, render_layers)) = query.get(trigger.entity) else {
        return;
    };
    children.iter_descendants(parent).for_each(|entity| {
        if transforms.contains(entity) {
            commands.entity(entity).insert(render_layers.clone());
        }
    });
    commands.entity(trigger.observer()).despawn();
}

fn update_wind_indicator_position(
    mut set: ParamSet<(
        Single<&mut Transform, (With<Camera3d>, With<ActiveCamera>)>,
        Single<&mut Transform, With<WindIndicator>>,
    )>,
) {
    let active_camera_rotation = set.p0().rotation;
    let wind_indicator = set.p1();
    let mut transform = wind_indicator.into_inner();
    transform.rotation = active_camera_rotation;
}
