use bevy::app::App;
use bevy::pbr::{ExtendedMaterial, MaterialPlugin, StandardMaterial};
use bevy::prelude::Plugin;
use crate::material::ground::GroundMaterial;

pub mod ground;

pub struct CustomMaterialsPlugin;

impl Plugin for CustomMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, GroundMaterial>>::default());
    }
}