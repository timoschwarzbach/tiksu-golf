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

impl Chunk {
    pub(self) fn height_at(&self, sub_chunk_x: f32, sub_chunk_z: f32) -> Option<f32> {
        // TODO: this code is hardcoded for 1mx1m mesh fidelity - update once chunk fidelity is up for change
        let x_idx = sub_chunk_x.floor() as usize;
        let z_idx = sub_chunk_z.floor() as usize;
        let x_sub = sub_chunk_x % 1.0;
        let z_sub = sub_chunk_z % 1.0;

        let interpolated = self.elevation.get(z_idx)?.get(x_idx)? * (1.0 - z_sub) * (1.0 - x_sub)
            + self.elevation.get(z_idx + 1)?.get(x_idx)? * z_sub * (1.0 - x_sub)
            + self.elevation.get(z_idx)?.get(x_idx + 1)? * (1.0 - z_sub) * x_sub
            + self.elevation.get(z_idx + 1)?.get(x_idx + 1)? * z_sub * x_sub;

        Some(interpolated)
    }
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