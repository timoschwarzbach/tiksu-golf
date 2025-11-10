pub mod grasslands;

pub trait TerrainGenerator {
    fn height_at(&self, x: f32, y: f32) -> f32;
}
