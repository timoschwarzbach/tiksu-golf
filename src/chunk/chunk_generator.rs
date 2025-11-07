use crate::noise;
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::mesh::{Indices, Mesh, Mesh3d, PrimitiveTopology};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, Component, Entity, Query, ResMut, Vec3, Without, default};

const CHUNK_SIZE: usize = 64;

#[derive(Component)]
pub struct Chunk {
    world_offset: [i32; 2],
    elevation: Box<[[f32; CHUNK_SIZE + 1]; CHUNK_SIZE + 1]>,
}

impl Chunk {
    pub fn generate_at(world_offset: [i32; 2]) -> Self {
        let mut elevation = Box::new([[0.0; CHUNK_SIZE + 1]; CHUNK_SIZE + 1]);

        for x in 0..=CHUNK_SIZE {
            for z in 0..=CHUNK_SIZE {
                let height = noise::layered_with_mountains(
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
            Mesh::ATTRIBUTE_COLOR,
            self.elevation
                .iter()
                .enumerate()
                .flat_map(|(x, row)| {
                    row.iter().enumerate().map(move |(z, _)| {
                        if (x >> 3) & 1 != (z >> 3) & 1 {
                            [0.0, 1.0, 0.0, 0.0]
                        } else {
                            [0.0, 0.5, 0.0, 0.0]
                        }
                    })
                })
                .collect::<Vec<_>>(),
        );

        const CHUNK_SIZE_U32: u32 = CHUNK_SIZE as u32;

        result.insert_indices(Indices::U32(
            (0..CHUNK_SIZE_U32)
                .flat_map(|x| (0..CHUNK_SIZE_U32).map(move |y| (x, y)))
                .flat_map(move |(x, y)| {
                    [
                        y * (CHUNK_SIZE_U32 + 1) + x,
                        y * (CHUNK_SIZE_U32 + 1) + x + 1,
                        y * (CHUNK_SIZE_U32 + 1) + x + CHUNK_SIZE_U32 + 1,
                        y * (CHUNK_SIZE_U32 + 1) + x + 1,
                        y * (CHUNK_SIZE_U32 + 1) + x + CHUNK_SIZE_U32 + 2,
                        y * (CHUNK_SIZE_U32 + 1) + x + CHUNK_SIZE_U32 + 1,
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for (entity, chunk) in query {
        let cube_mesh_handle: Handle<Mesh> = meshes.add(chunk.generate_mesh());
        commands.entity(entity).insert((
            Mesh3d(cube_mesh_handle),
            MeshMaterial3d(materials.add(StandardMaterial {
                //base_color_texture: Some(custom_texture_handle),
                ..default()
            })),
        ));
    }
}
