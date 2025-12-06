use crate::chunk::chunk_loader::ChunkLoader;
use crate::chunk::{CHUNK_SIZE_METERS, Chunk, ToUnload};
use bevy::prelude::{Commands, Component, Entity, Query, ResMut, Resource};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use crate::animation::FadeOutAnimation;

#[derive(Resource)]
pub struct ChunkManager {
    chunks: HashMap<(i32, i32), Entity>,
}

impl ChunkManager {
    pub fn new() -> Self {
        ChunkManager {
            chunks: HashMap::new(),
        }
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
                    Chunk::generate_at([
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
            commands.entity(chunk).insert((ToUnload, FadeOutAnimation::new(0.25)));
        }
    }
}

fn distance(from: (i32, i32), to: (i32, i32)) -> f32 {
    let dx = (to.0 - from.0) * CHUNK_SIZE_METERS as i32;
    let dz = (to.1 - from.1) * CHUNK_SIZE_METERS as i32;
    ((dx * dx + dz * dz) as f32).sqrt()
}

#[derive(Component)]
pub(super) struct MeshGenerationPriority(pub(super) f32);

pub(super) fn load_chunks(
    query: Query<&ChunkLoader>,
    mut chunks: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let mut schedule_load = HashMap::new();

    // load (if not yet loaded) all chunks nearby camera
    for loader in query {
        let Some(chunk_position) = loader.chunk_position else { continue };
        let bounds_hw = (loader.loading_threshold / CHUNK_SIZE_METERS as f32).ceil() as i32;
        for dx in -bounds_hw..=bounds_hw {
            for dz in -bounds_hw..=bounds_hw {
                let chunk_pos = (chunk_position.0 + dx, chunk_position.1 + dz);
                let dis = distance(chunk_pos, chunk_position);
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
    query: Query<&ChunkLoader>,
    mut chunks: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let mut schedule_unload = HashSet::new();
    // unload all chunks too far from any camera
    'outer: for (chunk_pos, _) in &mut chunks.chunks {
        for loader in query {
            let Some(chunk_position) = loader.chunk_position else { continue };
            if distance(*chunk_pos, chunk_position) <= loader.unloading_threshold {
                continue 'outer;
            }
        }
        schedule_unload.insert(*chunk_pos);
    }
    for chunk_pos in schedule_unload {
        chunks.unload_chunk(&mut commands, chunk_pos);
    }
}
