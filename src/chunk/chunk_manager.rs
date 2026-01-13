use crate::animation::LiftDownAnimation;
use crate::chunk::chunk_loader::ChunkLoader;
use crate::chunk::{CHUNK_SIZE_METERS, Chunk, ToUnload};
use bevy::prelude::{Commands, Component, Entity, Query, ResMut, Resource, Transform};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use crate::generation::grasslands::GrasslandsGenerator;
use crate::generation::TerrainGenerator;

#[derive(Resource)]
pub struct ChunkManager {
    chunks: HashMap<(i32, i32), Entity>,
    pub generator: Box<dyn TerrainGenerator + Send + Sync>,
}

impl ChunkManager {
    pub fn new(seed: u32) -> Self {
        ChunkManager {
            chunks: HashMap::new(),
            generator: Box::new(GrasslandsGenerator::new(seed)),
        }
    }

    pub fn replace_generator(&mut self, commands: &mut Commands, generator: Box<dyn TerrainGenerator + Send + Sync>) -> Box<dyn TerrainGenerator + Send + Sync> {
        let result = std::mem::replace(&mut self.generator, generator);
        for chunk_pos in self.chunks.keys().cloned().collect::<Vec<_>>() {
            self.unload_chunk(commands, chunk_pos);
        }
        result
    }

    pub fn height_at(&self, chunks: Query<(Entity, &Chunk)>, x: f32, z: f32) -> Option<f32> {
        let chunk_pos = (
            (x / CHUNK_SIZE_METERS as f32).floor() as i32,
            (z / CHUNK_SIZE_METERS as f32).floor() as i32,
        );

        let chunk_id = self.chunks.get(&chunk_pos)?;
        let chunk = chunks.get(*chunk_id).ok()?.1;

        chunk.height_at(x % CHUNK_SIZE_METERS as f32, z % CHUNK_SIZE_METERS as f32)
    }

    fn load_chunk(&mut self, commands: &mut Commands, chunk_pos: (i32, i32), priority: f32) {
        self.chunks.entry(chunk_pos).or_insert_with(|| {
            commands
                .spawn((
                    Chunk::generate_at(self.generator.as_ref(), [
                        chunk_pos.0 * CHUNK_SIZE_METERS as i32,
                        chunk_pos.1 * CHUNK_SIZE_METERS as i32,
                    ]),
                    MeshGenerationPriority(priority),
                ))
                .id()
        });
    }

    fn unload_chunk(&mut self, commands: &mut Commands, chunk_pos: (i32, i32)) {
        if let Some(chunk) = self.chunks.remove(&chunk_pos) {
            commands
                .entity(chunk)
                .insert((ToUnload, LiftDownAnimation::new(0.0, 0.25)));
        }
    }
}

fn distance(from: (f32, f32), to: (f32, f32)) -> f32 {
    let dx = to.0 - from.0;
    let dz = to.1 - from.1;
    (dx * dx + dz * dz).sqrt() * CHUNK_SIZE_METERS as f32
}

#[derive(Component)]
pub(super) struct MeshGenerationPriority(pub(super) f32);

fn get_transform_chunk_pos(transform: &Transform) -> (f32, f32) {
    (
        transform.translation.x / (CHUNK_SIZE_METERS as f32),
        transform.translation.z / (CHUNK_SIZE_METERS as f32),
    )
}

fn center_chunk_pos(pos: (i32, i32)) -> (f32, f32) {
    (pos.0 as f32 + 0.5, pos.1 as f32 + 0.5)
}

pub(super) fn load_chunks(
    query: Query<(&ChunkLoader, &Transform)>,
    mut chunks: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let mut schedule_load = HashMap::new();

    // load (if not yet loaded) all chunks nearby camera
    for (loader, loader_transform) in query {
        let loader_position = get_transform_chunk_pos(loader_transform);
        let (loader_x_i32, loader_z_i32) = (loader_position.0 as i32, loader_position.1 as i32);
        let bounds_hw = (loader.loading_threshold / CHUNK_SIZE_METERS as f32).ceil() as i32 + 1;

        for dx in -bounds_hw..=bounds_hw {
            for dz in -bounds_hw..=bounds_hw {
                let chunk_pos = (loader_x_i32 + dx, loader_z_i32 + dz);
                let dis = distance(center_chunk_pos(chunk_pos), loader_position);
                if dis <= loader.loading_threshold {
                    match schedule_load.entry(chunk_pos) {
                        Entry::Vacant(e) => {
                            e.insert(dis);
                        }
                        Entry::Occupied(mut e) => {
                            e.insert(dis.min(*e.get()));
                        }
                    }
                }
            }
        }
    }

    for (chunk_pos, priority) in schedule_load {
        chunks.load_chunk(&mut commands, chunk_pos, priority);
    }
}

pub(super) fn unload_chunks(
    query: Query<(&ChunkLoader, &Transform)>,
    mut chunks: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let mut schedule_unload = HashSet::new();
    // unload all chunks too far from any camera
    'outer: for (chunk_pos, _) in &mut chunks.chunks {
        for (loader, loader_transform) in query {
            let chunk_position = get_transform_chunk_pos(loader_transform);
            if distance(center_chunk_pos(*chunk_pos), chunk_position) <= loader.unloading_threshold
            {
                continue 'outer;
            }
        }
        schedule_unload.insert(*chunk_pos);
    }
    for chunk_pos in schedule_unload {
        chunks.unload_chunk(&mut commands, chunk_pos);
    }
}
