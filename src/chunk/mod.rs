pub mod chunk_loader;
pub mod chunk_manager;
pub mod generation;

use crate::animation::{FadeOutAnimation, LiftDownAnimation};
use crate::chunk::chunk_manager::ChunkManager;
use crate::chunk::generation::{WaterExtension, change_tree_material};
use crate::generation::Prop;
use crate::generation::grasslands::GrasslandsGenerator;
use crate::material::ground::Polynomial;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::Asset;
use bevy::input::ButtonInput;
use bevy::pbr::ExtendedMaterial;
use bevy::prelude::{
    Commands, Component, Entity, KeyCode, PostUpdate, Query, Reflect, Res, ResMut, With, Without,
};
use bevy::prelude::{MaterialPlugin, StandardMaterial};
use bevy::render::render_resource::{AsBindGroup, ShaderType};

pub(self) const CHUNK_SIZE_METERS: usize = 32;
pub(self) const CHUNK_FIDELITY: usize = CHUNK_SIZE_METERS * 1;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default, ShaderType)]
pub struct Bunker {
    pub x: f32,
    pub y: f32,
    pub rot: f32,
    pub size: f32,
}

impl Bunker {
    pub fn dis(&self, x: f32, y: f32) -> f32 {
        let dx = self.x - x;
        let dy = self.y - y;

        let s = self.rot.sin();
        let c = self.rot.cos();

        let rx = (dx * c + dy * s) / self.size;
        let ry = (dy * c - dx * s) / self.size * 1.6;

        (rx * rx + ry * ry).sqrt()
    }
}

#[derive(Component)]
pub struct Chunk {
    world_offset: [i32; 2],
    elevation: Box<[[f32; CHUNK_FIDELITY + 1]; CHUNK_FIDELITY + 1]>,
    props: Vec<Prop>,
    course: Polynomial,
    bunker: Bunker,
}

impl Chunk {
    pub fn height_at(&self, sub_chunk_x: f32, sub_chunk_z: f32) -> Option<f32> {
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
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, WaterExtension>,
        >::default())
            .add_systems(Startup, move |mut commands: Commands| {
                commands.insert_resource(ChunkManager::new(seed));
            })
            .add_systems(Update, generation::insert_chunk_mesh)
            .add_systems(Update, chunk_manager::load_chunks)
            .add_systems(Update, chunk_manager::unload_chunks)
            .add_systems(Update, generation::update_material_time)
            .add_systems(PostUpdate, despawn_unloaded_chunks)
            .add_systems(Update, regenerate_on_r)
            .add_observer(change_tree_material);
    }
}

#[derive(Component)]
struct ToUnload;

fn despawn_unloaded_chunks(
    query: Query<
        Entity,
        (
            With<ToUnload>,
            Without<FadeOutAnimation>,
            Without<LiftDownAnimation>,
        ),
    >,
    mut commands: Commands,
) {
    for chunk in query {
        commands.entity(chunk).despawn();
    }
}

fn regenerate_on_r(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        let seed = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() as u32;
        chunk_manager.replace_generator(&mut commands, Box::new(GrasslandsGenerator::new(seed)));
    }
}
