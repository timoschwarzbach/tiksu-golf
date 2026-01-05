use bevy::pbr::MaterialExtension;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

const SHADER_ASSET_PATH: &str = "shaders/ground_material.wgsl";

// TODO: rename to something like color golf course

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default, ShaderType)]
pub struct Polynomial {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct GroundMaterial {
    #[uniform(100)]
    course: Polynomial,
    #[uniform(100)]
    start_x: f32,
    #[uniform(100)]
    end_x: f32,
}

impl GroundMaterial {
    pub fn new() -> Self {
        GroundMaterial {
            course: Polynomial {
                a: -0.00003,
                b: 0.013,
                c: -1.47,
                d: 0.0,
            },
            start_x: 0.0,
            end_x: 300.0,
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