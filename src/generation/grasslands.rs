use crate::generation::TerrainGenerator;
use noise::NoiseFn;
use noise::Perlin;

/* Pipeline (one time):
 * 1. generate course / routes (single fixed line for now)
 *   - start and end location
 * Pipeline (per pixel):
 * 1. generate local noise map
 * 2. generate global noise map for faraway mountains
 * 3. create water
 * 4. generate sand bunkers
 *   - thresholded noise map
 *   - can only appear on course
 *   - sandy pit with line of darker grass around it
 * 5. set remaining material
 *   - end rod at end rod location
 *   - smooth grass near end location
 *   - checkerboard grass everywhere else in course area
 *   - high grass at course area edge
 *   - high grass plus trees outside course area
 *   - stone/snow material outside course area at high heights
 */
pub struct GrasslandsGenerator {
    seed: u32,
    perlin: Perlin,
}

impl GrasslandsGenerator {
    pub fn new(seed: u32) -> Self {
        GrasslandsGenerator {
            seed,
            perlin: Perlin::new(seed),
        }
    }

    fn local_height_at(&self, x: f64, y: f64) -> f64 {
        self.perlin.get([x / 12.0, y / 12.0]) * 0.15
            + self.perlin.get([x / 30.0, y / 30.0])
            + self.perlin.get([x / 120.0, y / 120.0]) * 6.0
    }
}

impl TerrainGenerator for GrasslandsGenerator {
    fn height_at(&self, x: f32, y: f32) -> f32 {
        self.local_height_at(x as f64, y as f64) as f32
    }
}
