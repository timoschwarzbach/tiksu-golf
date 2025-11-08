use std::ops::Rem;
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::math::Affine2;
use bevy::mesh::{Indices, Mesh, Mesh3d, PrimitiveTopology};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, Component, Entity, Query, ResMut, Vec3, Without, default, Image, Res, AssetServer, Vec2, Material, TypePath, Asset};
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use crate::chunk::{Chunk, CHUNK_FIDELITY, CHUNK_SIZE_METERS};
use crate::generation::grasslands;
use crate::generation::grasslands::GrasslandsGenerator;
use crate::generation::TerrainGenerator;

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

        Chunk {
            world_offset,
            elevation,
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
                    row.iter()
                        .enumerate()
                        .map(move |(z, _)| [x as f32 / CHUNK_SIZE_METERS as f32, z as f32  / CHUNK_SIZE_METERS as f32])
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

pub fn insert_chunk_mesh(
    query: Query<(Entity, &Chunk), Without<Mesh3d>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for (entity, chunk) in query {
        let material = materials.add(StandardMaterial {
            metallic_roughness_texture: Some(asset_server.load("textures/rocky_terrain/rocky_terrain_02_arm_4k.png")),
            base_color_texture: Some(asset_server.load("textures/rocky_terrain/rocky_terrain_02_diff_4k.png")),
            normal_map_texture: Some(asset_server.load("textures/rocky_terrain/rocky_terrain_02_nor_gl_4k.png")),
            ..default()
        });
        let cube_mesh_handle: Handle<Mesh> = meshes.add(chunk.generate_mesh());
        commands.entity(entity).insert((
            Mesh3d(cube_mesh_handle),
            MeshMaterial3d(material),
        ));
    }
}