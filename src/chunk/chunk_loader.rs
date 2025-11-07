use crate::chunk::CHUNK_SIZE_METERS;
use bevy::prelude::{Changed, Component, Query, Transform};

const CHUNK_SIZE_METERS_F32: f32 = CHUNK_SIZE_METERS as f32;

#[derive(Component)]
pub struct ChunkLoader {
    pub loading_threshold: f32,
    pub unloading_threshold: f32,
    pub chunk_position: (i32, i32),
}

impl ChunkLoader {
    pub fn new(render_distance: f32) -> ChunkLoader {
        ChunkLoader {
            loading_threshold: render_distance,
            unloading_threshold: render_distance + CHUNK_SIZE_METERS_F32,
            chunk_position: (0, 0),
        }
    }
}

// TODO: super
pub fn update_chunk_loader_position(
    query: Query<(&mut ChunkLoader, &Transform), Changed<Transform>>,
) {
    for (mut loader, transform) in query {
        loader.chunk_position = (
            (transform.translation.x / CHUNK_SIZE_METERS_F32).floor() as i32,
            (transform.translation.z / CHUNK_SIZE_METERS_F32).floor() as i32,
        );
    }
}
