pub mod chunk_generator;
pub mod chunk_loader;
pub mod chunk_manager;
pub use chunk_generator::{Chunk, insert_chunk_mesh};

pub(self) const CHUNK_SIZE_METERS: usize = 64;

// TODO: move chunk struct here, rename modules
