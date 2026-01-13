use crate::animation::LiftUpAnimation;
use crate::chunk::chunk_manager::MeshGenerationPriority;
use crate::chunk::{CHUNK_FIDELITY, CHUNK_SIZE_METERS, Chunk};
use crate::generation::{Prop, TerrainGenerator};
use crate::material::ground::GroundMaterial;
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::gltf::GltfAssetLabel;
use bevy::image::{
    ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor,
};
use bevy::light::NotShadowCaster;
use bevy::math::Dir3;
use bevy::mesh::{Indices, Mesh, Mesh3d, Meshable, PrimitiveTopology};
use bevy::pbr::{ExtendedMaterial, MaterialExtension, MeshMaterial3d, StandardMaterial};
use bevy::prelude::{
    AlphaMode, Asset, AssetServer, Color, Commands, Entity, Image, Plane3d, Query, Reflect, Res,
    ResMut, SceneRoot, Time, Transform, Vec3, Without, default,
};
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

const CHUNKS_MESHED_PER_TICK: usize = 24;
const WATER_HEIGHT: f32 = -4.0;

impl Chunk {
    pub fn generate_at(generator: &dyn TerrainGenerator, world_offset: [i32; 2]) -> Self {
        let mut elevation = Box::new([[0.0; CHUNK_FIDELITY + 1]; CHUNK_FIDELITY + 1]);

        for x in 0..=CHUNK_FIDELITY {
            for z in 0..=CHUNK_FIDELITY {
                let height = generator.height_at(
                    x as f32 + world_offset[0] as f32,
                    z as f32 + world_offset[1] as f32,
                );
                elevation[x][z] = height
            }
        }

        let props = generator.props_in_chunk((world_offset[0], world_offset[1]));
        let bunker = generator.nearest_bunker(world_offset);

        Chunk {
            world_offset,
            elevation,
            props,
            course: generator.course_layout(),
            bunker,
        }
    }

    fn generate_mesh(&self) -> Mesh {
        let mut result = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );

        result.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            self.elevation
                .iter()
                .enumerate()
                .flat_map(|(x, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(z, &height)| [x as f32, height, z as f32])
                })
                .collect::<Vec<_>>(),
        );

        result.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            self.elevation
                .iter()
                .enumerate()
                .flat_map(|(x, row)| {
                    row.iter().enumerate().map(move |(z, _)| {
                        [
                            x as f32 / CHUNK_SIZE_METERS as f32,
                            z as f32 / CHUNK_SIZE_METERS as f32,
                        ]
                    })
                })
                .collect::<Vec<_>>(),
        );

        const CHUNK_FIDELITY_U32: u32 = CHUNK_FIDELITY as u32;

        result.insert_indices(Indices::U32(
            (0..CHUNK_FIDELITY_U32)
                .flat_map(|x| (0..CHUNK_FIDELITY_U32).map(move |y| (x, y)))
                .flat_map(move |(x, y)| {
                    [
                        y * (CHUNK_FIDELITY_U32 + 1) + x,
                        y * (CHUNK_FIDELITY_U32 + 1) + x + 1,
                        y * (CHUNK_FIDELITY_U32 + 1) + x + CHUNK_FIDELITY_U32 + 1,
                        y * (CHUNK_FIDELITY_U32 + 1) + x + 1,
                        y * (CHUNK_FIDELITY_U32 + 1) + x + CHUNK_FIDELITY_U32 + 2,
                        y * (CHUNK_FIDELITY_U32 + 1) + x + CHUNK_FIDELITY_U32 + 1,
                    ]
                })
                .collect(),
        ));

        result.compute_normals();

        result.translated_by(Vec3::new(
            self.world_offset[0] as f32,
            0.0,
            self.world_offset[1] as f32,
        ))
    }
}

const SHADER_ASSET_PATH: &str = "shaders/water.wgsl";

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct WaterExtension {
    #[uniform(100)]
    pub time: f32,
    #[texture(101)]
    #[sampler(102)]
    normal_map: Option<Handle<Image>>,
}

impl MaterialExtension for WaterExtension {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

pub(super) fn insert_chunk_mesh(
    query: Query<(Entity, &Chunk, &MeshGenerationPriority), Without<Mesh3d>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut ground_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, GroundMaterial>>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut water_material: ResMut<Assets<ExtendedMaterial<StandardMaterial, WaterExtension>>>,
    mut commands: Commands,
) {
    let selection = query
        .iter()
        .sort_by::<&MeshGenerationPriority>(|a, b| a.0.total_cmp(&b.0))
        .take(CHUNKS_MESHED_PER_TICK);

    for (entity, chunk, _) in selection {
        // terrain height mesh
        let material = ground_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color_texture: Some(
                    asset_server.load("textures/grass/Grass008_2K-PNG_Color.png"),
                ),
                normal_map_texture: Some(
                    asset_server.load("textures/grass/Grass008_2K-PNG_NormalGL.png"),
                ),
                reflectance: 0.06,
                alpha_mode: AlphaMode::AlphaToCoverage,
                //base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                ..default()
            },
            extension: GroundMaterial::new(chunk.course.clone(), chunk.bunker.clone()),
        });
        let normal_handle =
            asset_server.load_with_settings("textures/water/water0342normal.png", |s: &mut _| {
                *s = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        mag_filter: ImageFilterMode::Linear,
                        ..default()
                    }),
                    ..default()
                }
            });
        let terrain_mesh_handle: Handle<Mesh> = meshes.add(chunk.generate_mesh());
        commands
            .entity(entity)
            .insert((
                Mesh3d(terrain_mesh_handle),
                MeshMaterial3d(material),
                NotShadowCaster,
                // FadeInAnimation::new(0.25),
                Transform::from_xyz(0.0, -100.0, 0.0),
                LiftUpAnimation::new(0.0, 0.25),
            ))
            .remove::<MeshGenerationPriority>();

        // water plane mesh
        if chunk
            .elevation
            .iter()
            .any(|row| row.iter().any(|height| *height < WATER_HEIGHT))
        {
            let x = chunk.world_offset[0] as f32 + CHUNK_SIZE_METERS as f32 * 0.5;
            let z = chunk.world_offset[1] as f32 + CHUNK_SIZE_METERS as f32 * 0.5;

            let child = commands
                .spawn((
                    Transform::from_xyz(x, -5.0, z),
                    Mesh3d(
                        meshes.add(
                            Plane3d::default()
                                .mesh()
                                .size(CHUNK_SIZE_METERS as f32, CHUNK_SIZE_METERS as f32)
                                .normal(Dir3::Y),
                        ),
                    ),
                    MeshMaterial3d(water_material.add(ExtendedMaterial {
                        base: StandardMaterial {
                            base_color: Color::srgba(0.059, 0.886, 0.902, 0.7),
                            reflectance: 0.15,
                            alpha_mode: AlphaMode::Blend,
                            ..default()
                        },
                        extension: WaterExtension {
                            time: 0.0,
                            normal_map: Some(normal_handle),
                            ..default()
                        },
                    })),
                    NotShadowCaster,
                ))
                .id();

            commands.entity(entity).add_child(child);
        }

        // props
        for Prop {
            position: (px, py, pz),
            seed,
            ..
        } in &chunk.props
        {
            let height = 0.035 + ((*seed) % 100) as f32 * 0.0001;
            let child = commands
                .spawn((
                    Transform::from_xyz(
                        chunk.world_offset[0] as f32 + *px,
                        *py - 0.5,
                        chunk.world_offset[1] as f32 + *pz,
                    )
                    .with_scale(Vec3::splat(height)),
                    SceneRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("model/pine_tree.glb")),
                    ),
                ))
                .id();

            commands.entity(entity).add_child(child);
        }

        // props
        for Prop {
            position: (px, py, pz),
            seed,
            ..
        } in &chunk.props
        {
            let height = 0.035 + ((*seed) % 100) as f32 * 0.0001;
            let child = commands
                .spawn((
                    Transform::from_xyz(
                        chunk.world_offset[0] as f32 + *px,
                        *py - 0.5,
                        chunk.world_offset[1] as f32 + *pz,
                    )
                    .with_scale(Vec3::splat(height)),
                    SceneRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("model/pine_tree.glb")),
                    ),
                ))
                .id();

            commands.entity(entity).add_child(child);
        }
    }
}

pub fn update_material_time(
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, WaterExtension>>>,
    time: Res<Time>,
) {
    for (_id, mat) in materials.iter_mut() {
        mat.extension.time = time.elapsed_secs();
    }
}
