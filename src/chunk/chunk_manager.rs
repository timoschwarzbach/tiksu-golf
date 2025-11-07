use crate::chunk::chunk_loader::ChunkLoader;
use crate::chunk::{CHUNK_SIZE_METERS, Chunk};
use bevy::prelude::{Commands, Entity, Query, ResMut, Resource};
use std::collections::{HashMap, HashSet};

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

    pub fn height_at(&self, x: f32, z: f32) -> f32 {
        todo!("get height")
    }

    fn load_chunk(&mut self, commands: &mut Commands, chunk_pos: (i32, i32)) {
        // TODO: import chunk sizes
        self.chunks.entry(chunk_pos).or_insert_with(|| {
            commands
                .spawn(Chunk::generate_at([chunk_pos.0 * 64, chunk_pos.1 * 64]))
                .id()
        });
    }

    fn unload_chunk(&mut self, commands: &mut Commands, chunk_pos: (i32, i32)) {
        if let Some(chunk) = self.chunks.remove(&chunk_pos) {
            commands.entity(chunk).despawn();
        }
    }
}

fn distance(from: (i32, i32), to: (i32, i32)) -> f32 {
    let dx = (to.0 - from.0) * CHUNK_SIZE_METERS as i32;
    let dz = (to.1 - from.1) * CHUNK_SIZE_METERS as i32;
    ((dx * dx + dz * dz) as f32).sqrt()
}

// TODO: super
pub fn load_chunks(
    query: Query<&ChunkLoader>,
    mut chunks: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    // load (if not yet loaded) all chunks nearby camera
    for loader in query {
        let bounds_hw = (loader.loading_threshold / CHUNK_SIZE_METERS as f32).ceil() as i32;
        for dx in -bounds_hw..=bounds_hw {
            for dz in -bounds_hw..=bounds_hw {
                let chunk_pos = (loader.chunk_position.0 + dx, loader.chunk_position.1 + dz);
                if distance(chunk_pos, loader.chunk_position) <= loader.loading_threshold {
                    chunks.load_chunk(&mut commands, chunk_pos);
                }
            }
        }
    }
}

// TODO: super
pub fn unload_chunks(
    query: Query<&ChunkLoader>,
    mut chunks: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let mut schedule_unload = HashSet::new();
    // unload all chunks too far from any camera
    'outer: for (chunk_pos, _) in &mut chunks.chunks {
        for loader in query {
            if distance(loader.chunk_position, *chunk_pos) <= loader.unloading_threshold {
                continue 'outer;
            }
        }
        schedule_unload.insert(*chunk_pos);
    }
    for chunk_pos in schedule_unload {
        chunks.unload_chunk(&mut commands, chunk_pos);
    }
}
