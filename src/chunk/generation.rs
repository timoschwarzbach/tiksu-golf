use crate::chunk::{CHUNK_FIDELITY, CHUNK_SIZE_METERS, Chunk};
use crate::generation::{Prop, TerrainGenerator};
use crate::generation::grasslands::GrasslandsGenerator;
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::gltf::GltfAssetLabel;
use bevy::light::NotShadowCaster;
use bevy::math::Dir3;
use bevy::mesh::{Indices, Mesh, Mesh3d, Meshable, PrimitiveTopology};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{AssetServer, Commands, Entity, Query, Res, ResMut, Vec3, Without, default, AlphaMode, Color, Transform, Plane3d, Cuboid, SceneRoot};
use crate::animation::FadeInAnimation;
use crate::chunk::chunk_manager::MeshGenerationPriority;

const CHUNKS_MESHED_PER_TICK: usize = 24;
const WATER_HEIGHT: f32 = -4.0;

impl Chunk {
    pub fn generate_at(world_offset: [i32; 2]) -> Self {
        let mut elevation = Box::new([[0.0; CHUNK_FIDELITY + 1]; CHUNK_FIDELITY + 1]);

        // TODO: store somewhere (?)
        let generator = GrasslandsGenerator::new(42);

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

        Chunk {
            world_offset,
            elevation,
            props,
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

pub(super) fn insert_chunk_mesh(
    query: Query<(Entity, &Chunk, &MeshGenerationPriority), Without<Mesh3d>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let selection = query
        .iter()
        .sort_by::<&MeshGenerationPriority>(|a, b| a.0.total_cmp(&b.0))
        .take(CHUNKS_MESHED_PER_TICK);

    for (entity, chunk, _) in selection {
        // terrain height mesh
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(
                asset_server.load("textures/grass/Grass008_2K-PNG_Color.png"),
            ),
            normal_map_texture: Some(
                asset_server.load("textures/grass/Grass008_2K-PNG_NormalGL.png"),
            ),
            reflectance: 0.06,
            alpha_mode: AlphaMode::Blend,
            base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
            ..default()
        });
        let terrain_mesh_handle: Handle<Mesh> = meshes.add(chunk.generate_mesh());
        commands
            .entity(entity)
            .insert((Mesh3d(terrain_mesh_handle), MeshMaterial3d(material), NotShadowCaster, FadeInAnimation::new(0.25)))
            .remove::<MeshGenerationPriority>();

        // water plane mesh
        if chunk.elevation.iter().any(|row| row.iter().any(|height| *height < WATER_HEIGHT)) {
            let x = chunk.world_offset[0] as f32 + CHUNK_SIZE_METERS as f32 * 0.5;
            let z = chunk.world_offset[1] as f32 + CHUNK_SIZE_METERS as f32 * 0.5;

            let child = commands.spawn((
                Transform::from_xyz(x, -5.0, z),
                Mesh3d(meshes.add(Plane3d::default().mesh().size(CHUNK_SIZE_METERS as f32, CHUNK_SIZE_METERS as f32).normal(Dir3::Y))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgba(0.13, 0.59, 0.84, 0.6),
                    reflectance: 0.1,
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })),
                NotShadowCaster,
            )).id();

            commands
                .entity(entity)
                .add_child(child);
        }

        // props
        for Prop { position: (px, py, pz), .. } in &chunk.props {
            let child = commands.spawn((
                Transform::from_xyz(chunk.world_offset[0] as f32 + *px, *py - 0.5, chunk.world_offset[1] as f32 + *pz).with_scale(Vec3::splat(0.04)),
                SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("model/pine_tree.glb"))),
            )).id();

            commands
                .entity(entity)
                .add_child(child);
        }
    }
}
