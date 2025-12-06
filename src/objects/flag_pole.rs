use avian3d::prelude::RigidBody;
use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::{BLUE, RED, WHITE},
    mesh::{Indices, PrimitiveTopology},
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

pub struct FlagPolePlugin;

impl Plugin for FlagPolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, FlagMaterialExtension>,
        >::default())
            .add_systems(Startup, spawn_flag_pole);
    }
}

#[derive(Component)]
pub struct FlagPole;

fn spawn_flag_pole(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut default_materials: ResMut<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, FlagMaterialExtension>>>,
) {
    commands
        .spawn((
            FlagPole,
            RigidBody::Static,
            Transform::from_xyz(10.0, 3.0, 0.0),
        ))
        .with_children(|builder| {
            builder.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.05, 2.0))),
                MeshMaterial3d(default_materials.add(StandardMaterial {
                    base_color: Color::from(WHITE),
                    ..Default::default()
                })),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            builder.spawn((
                Mesh3d(meshes.add(generate_mesh())),
                MeshMaterial3d(materials.add(ExtendedMaterial {
                    base: StandardMaterial {
                        base_color: RED.into(),
                        ..Default::default()
                    },
                    extension: FlagMaterialExtension {
                        color: Color::from(BLUE).to_linear(),
                    },
                })),
                Transform::from_xyz(0.5, 0.75, 0.0).with_scale(Vec3::new(1.0, 0.5, 1.0)),
            ));
        });
}

fn generate_mesh() -> Mesh {
    let mut result = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    const FLAG_SIZE: u32 = 40;
    const INVERSE_FLAG_SIZE: f32 = 1.0 / FLAG_SIZE as f32;

    result.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        (0..FLAG_SIZE)
            .into_iter()
            .flat_map(|x| {
                (0..FLAG_SIZE).into_iter().map(move |y| {
                    [
                        x as f32 * INVERSE_FLAG_SIZE - 0.5,
                        y as f32 * INVERSE_FLAG_SIZE - 0.5,
                        0.0,
                    ]
                })
            })
            .collect::<Vec<_>>(),
    );

    result.insert_indices(Indices::U32(
        (0..FLAG_SIZE - 1)
            .flat_map(|x| (0..FLAG_SIZE - 1).map(move |y| (x, y)))
            .flat_map(move |(x, y)| {
                [
                    // front
                    y * (FLAG_SIZE) + x,
                    y * (FLAG_SIZE) + x + 1,
                    y * (FLAG_SIZE) + x + FLAG_SIZE,
                    y * (FLAG_SIZE) + x + 1,
                    y * (FLAG_SIZE) + x + FLAG_SIZE + 1,
                    y * (FLAG_SIZE) + x + FLAG_SIZE,
                    // back
                    y * (FLAG_SIZE) + x,
                    y * (FLAG_SIZE) + x + FLAG_SIZE,
                    y * (FLAG_SIZE) + x + 1,
                    y * (FLAG_SIZE) + x + 1,
                    y * (FLAG_SIZE) + x + FLAG_SIZE,
                    y * (FLAG_SIZE) + x + FLAG_SIZE + 1,
                ]
            })
            .collect(),
    ));

    result
}

const SHADER_ASSET_PATH: &str = "shaders/flag_pole.wgsl";

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
struct FlagMaterialExtension {
    #[uniform(100)]
    color: LinearRgba,
}

impl MaterialExtension for FlagMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
