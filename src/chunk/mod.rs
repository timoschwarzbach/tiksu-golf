pub mod generation;
pub mod chunk_loader;
pub mod chunk_manager;

use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{Commands, Component};
use crate::chunk::chunk_manager::ChunkManager;

pub(self) const CHUNK_SIZE_METERS: usize = 32;
pub(self) const CHUNK_FIDELITY: usize = CHUNK_SIZE_METERS * 1;

#[derive(Component)]
pub(self) struct Chunk {
    world_offset: [i32; 2],
    elevation: Box<[[f32; CHUNK_FIDELITY + 1]; CHUNK_FIDELITY + 1]>,
}

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, |mut commands: Commands| {
                commands.insert_resource(ChunkManager::new());
            })
            .add_systems(Update, generation::insert_chunk_mesh)
            .add_systems(Update, chunk_manager::load_chunks)
            .add_systems(Update, chunk_manager::unload_chunks)
            .add_systems(Update, chunk_loader::update_chunk_loader_position);
    }
}