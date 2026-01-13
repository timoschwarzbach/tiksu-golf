use crate::generation::{Prop, PropType, TerrainGenerator, ZoneType};
use noise::NoiseFn;
use noise::Perlin;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};
use crate::chunk::Bunker;
use crate::material::ground::Polynomial;
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
    course: Polynomial,
    start: [f32; 2],
    hole: [f32; 2],
}

fn random_range(rng: &mut StdRng, min: f32, max: f32) -> f32 {
    rng.next_u32() as f32 / u32::MAX as f32 * (max - min) + min
}

impl GrasslandsGenerator {
    pub fn new(seed: u32) -> Self {
        let mut rng = StdRng::seed_from_u64(seed as u64);

        let mut polynomial = Polynomial::default();
        let hits = (
            random_range(&mut rng, -40.0, 40.0),
            random_range(&mut rng, -80.0, 80.0),
            random_range(&mut rng, -80.0, 80.0),
            random_range(&mut rng, -80.0, 80.0),
        );
        polynomial.d = hits.0;
        for _ in 0..10 {
            polynomial.c += (hits.1 - polynomial.f(100.0)) / 100.0;
            polynomial.b += (hits.2 - polynomial.f(200.0)) / 200.0 / 200.0;
            polynomial.a += (hits.3 - polynomial.f(300.0)) / 300.0 / 300.0 / 300.0;
        }

        let start = [0.0, polynomial.f(0.0)];
        let hole = [300.0, polynomial.f(300.0)];

        GrasslandsGenerator {
            seed,
            perlin: Perlin::new(seed),
            course: polynomial,
            start,
            hole,
        }
    }

    fn bunker_depth(&self, x: f32, y: f32) -> f32 {
        let chunk_x = ((x / 32.0).floor() as i32) * 32;
        let chunk_y = ((y / 32.0).floor() as i32) * 32;
        let bunker = self.nearest_bunker([chunk_x, chunk_y]);
        let result = (1.0 - bunker.dis(x, y)).max(0.0);
        result.sqrt() * 1.2
    }

    fn local_height_at(&self, x: f64, y: f64) -> f64 {
        self.perlin.get([x / 12.0, y / 12.0]) * 0.15
            + self.perlin.get([x / 30.0, y / 30.0])
            + self.perlin.get([x / 120.0, y / 120.0]) * 6.0
    }
}

fn dist(from: [f32; 2], to: [f32; 2]) -> f32 {
    let dx = from[0] - to[0];
    let dy = from[1] - to[1];
    (dx * dx + dy * dy).sqrt()
}

impl TerrainGenerator for GrasslandsGenerator {
    fn height_at(&self, x: f32, y: f32) -> f32 {
        let height = self.local_height_at(x as f64, y as f64) as f32
            - self.bunker_depth(x, y);
        let dist_to_start_or_hole = dist(self.start(), [x, y])
            .min(dist(self.hole(), [x, y]));
        // ensure start and hole are never underwater
        let min_height = -3.7 - (dist_to_start_or_hole * 0.075).powi(4);
        height.max(min_height)
    }

    fn props_in_chunk(&self, offset: (i32, i32)) -> Vec<Prop> {
        // TODO: don't hardcode chunk size and water height
        let approx_tree_count = ((self.perlin.get([offset.0 as f64 / 200.0, offset.1 as f64 / 200.0]) + 0.1) * 5.0).max(0.0) as usize;
        let seed = ((offset.0 as u64) << 16) ^ (offset.1 as u64);
        let mut random = StdRng::seed_from_u64(seed);

        let mut result = Vec::new();

        for _candidate in 0..approx_tree_count {
            let x = random.random_range(0.0..32.0);
            let z = random.random_range(0.0..32.0);
            let y = self.height_at(x + offset.0 as f32, z + offset.1 as f32);

            if self.zone_type_at(x + offset.0 as f32, z + offset.1 as f32) == ZoneType::Offtrack {
                let seed = random.next_u32();

                result.push(Prop {
                    prop_type: PropType::Tree,
                    position: (x, y, z),
                    seed,
                });
            }
        }

        result
    }

    fn course_layout(&self) -> Polynomial {
        self.course.clone()
    }

    fn start(&self) -> [f32; 2] {
        self.start
    }

    fn hole(&self) -> [f32; 2] {
        self.hole
    }

    fn zone_type_at(&self, x: f32, y: f32) -> ZoneType {
        if self.height_at(x, y) <= -3.0 {
            ZoneType::DeadZone
        } else if self.bunker_depth(x, y) != 0.0 {
            ZoneType::Bunker
        } else if self.course.on_clean_grass([x, y]) {
            ZoneType::Clean
        } else {
            ZoneType::Offtrack
        }
    }

    fn nearest_bunker(&self, world_offset: [i32; 2]) -> Bunker {
        let column = world_offset[0] | 63;
        let mut random = StdRng::seed_from_u64(column as u64 | ((self.seed as u64) << 32));

        let x = column as f32 - random_range(&mut random, 29.0, 35.0);
        let y = self.course.f(x) + random_range(&mut random, -170.0, 170.0);

        if self.course.approx_distance_to_curve([x, y]) >= 32.0 || x < 20.0 || 280.0 < x {
            return Bunker {
                x: -1_000_000.0,
                y: -1_000_000.0,
                rot: 0.0,
                size: 0.0,
            }
        }

        let rot = random_range(&mut random, 0.0, std::f32::consts::PI);

        let size = random_range(&mut random, 18.0, 28.0);

        Bunker {
            x,
            y,
            rot,
            size,
        }
    }
}
