use bevy::prelude::*;

use crate::{
    chunk::chunk_manager::ChunkManager,
    generation::grasslands::GrasslandsGenerator,
    objects::{flag_pole::FlagPole, golfball::Golfball},
};

fn regenerate_course(
    mut golfball: Single<&mut Transform, (With<Golfball>, Without<FlagPole>)>,
    mut flag_pole: Single<&mut Transform, (With<FlagPole>, Without<Golfball>)>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let seed = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() as u32;
    chunk_manager.replace_generator(&mut commands, Box::new(GrasslandsGenerator::new(seed)));

    let [start_x, start_z] = chunk_manager.generator.start();
    golfball.translation = Vec3::new(start_x, 10.0, start_z);

    let [hole_x, hole_z] = chunk_manager.generator.hole();
    let hole_y = chunk_manager.generator.height_at(hole_x, hole_z) + 0.5;
    flag_pole.translation = Vec3::new(hole_x, hole_y, hole_z);
}
