use bevy::pbr::MaterialExtension;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

const SHADER_ASSET_PATH: &str = "shaders/ground_material.wgsl";

// TODO: rename to something like color golf course

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct GroundMaterial {
    #[uniform(100)]
    quantize_steps: u32,
}

impl GroundMaterial {
    pub fn new() -> Self {
        GroundMaterial {
            quantize_steps: 10,
        }
    }
}

impl MaterialExtension for GroundMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}