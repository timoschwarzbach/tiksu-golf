use bevy::asset::RenderAssetUsages;
use bevy::camera::RenderTarget;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, TextureDimension, TextureFormat, TextureUsages};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use bevy::ui::Node;

#[derive(Component)]
struct FlagPole;

pub(super) fn spawn_course_info(builder: &mut ChildSpawnerCommands) {
    builder.spawn((Node {
        min_width: px(100),
        min_height: px(100),
        ..default()
    },));
}

pub(super) struct CourseFlagPlugin;
impl Plugin for CourseFlagPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FlagMaterial>::default())
            .add_systems(Startup, setup);
    }
}
fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FlagMaterial>>,
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
            Camera2d::default(),
            Camera {
                order: -1, // hidden under everything else
                clear_color: ClearColorConfig::Custom(Color::NONE),
                target: RenderTarget::Image(image_handle.clone().into()),
                ..default()
            },
        ))
        .id();

    // spawn 2d flag
    commands.spawn((
        FlagPole,
        Mesh2d(meshes.add(generate_mesh())),
        MeshMaterial2d(materials.add(FlagMaterial {
            color: LinearRgba::WHITE,
            color_texture: Some(asset_server.load("image/course_flag.png")),
        })),
        Transform::default().with_scale(Vec3::splat(128.0)),
    ));

    // spawn ui element
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(0),
            left: px(12),
            width: px(100),
            height: px(100),
            ..default()
        },
        ViewportNode::new(camera),
    ));
}

const SHADER_ASSET_PATH: &str = "shaders/course_info.wgsl";

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct FlagMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material2d for FlagMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

fn generate_mesh() -> Mesh {
    let mut result = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    result.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        (0..10)
            .into_iter()
            .flat_map(|x| {
                (0..10)
                    .into_iter()
                    .map(move |y| [x as f32 * 0.1 - 0.5, y as f32 * 0.1 - 0.5, 0.0])
            })
            .collect::<Vec<_>>(),
    );

    result.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        (0..10)
            .into_iter()
            .flat_map(|x| {
                (0..10)
                    .into_iter()
                    .map(move |y| [x as f32 * 0.1, 1.0 - y as f32 * 0.1])
            })
            .collect::<Vec<_>>(),
    );

    result.insert_indices(Indices::U32(
        (0..9)
            .flat_map(|x| (0..9).map(move |y| (x, y)))
            .flat_map(move |(x, y)| {
                [
                    y * (9 + 1) + x,
                    y * (9 + 1) + x + 1,
                    y * (9 + 1) + x + 9 + 1,
                    y * (9 + 1) + x + 1,
                    y * (9 + 1) + x + 9 + 2,
                    y * (9 + 1) + x + 9 + 1,
                ]
            })
            .collect(),
    ));

    result.with_computed_normals()
}
