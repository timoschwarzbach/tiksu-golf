use crate::chunk::CHUNK_SIZE_METERS;
use bevy::prelude::Component;

#[derive(Component)]
pub struct ChunkLoader {
    pub loading_threshold: f32,
    pub unloading_threshold: f32,
}

impl ChunkLoader {
    pub fn new(render_distance: f32) -> ChunkLoader {
        ChunkLoader {
            loading_threshold: render_distance,
            unloading_threshold: render_distance + CHUNK_SIZE_METERS as f32,
        }
    }
}
