use bevy::pbr::MaterialExtension;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

const SHADER_ASSET_PATH: &str = "shaders/ground_material.wgsl";

// TODO: rename to something like color golf course

const COURSE_WIDTH: f32 = 25.0;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default, ShaderType)]
pub struct Polynomial {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl Polynomial {
    pub fn f(&self, x: f32) -> f32 {
        self.a * x * x * x + self.b * x * x + self.c * x + self.d
    }

    pub fn f_prime(&self, x: f32) -> f32 {
        3.0 * self.a * x * x + 2.0 * self.b * x + self.c
    }

    pub fn approx_distance_to_curve(&self, p: [f32; 2]) -> f32 {
        let p_y = self.f(p[0]);
        let p_d = self.f_prime(p[0]);
        let h = (1.0 + p_d * p_d).sqrt();
        ((p_y - p[1]) / h).abs()
    }

    pub fn on_clean_grass(&self, p: [f32; 2]) -> bool {
        match p[0] {
            ..0.0 => {
                let p_d = self.f_prime(0.0);
                let h = (1.0 + p_d * p_d).sqrt();
                let dy = self.f(0.0) - (p[1] - p[0] * p_d);
                (dy * dy + p[0] * p[0]).sqrt() < COURSE_WIDTH * h
            },
            300.0.. => {
                let p_d = self.f_prime(300.0);
                let h = (1.0 + p_d * p_d).sqrt();
                let dx = p[0] - 300.0;
                let dy = self.f(300.0) - (p[1] - (p[0] - 300.0) * p_d);
                (dy * dy + dx * dx).sqrt() < COURSE_WIDTH * h
            },
            _ => {
                self.approx_distance_to_curve(p) < COURSE_WIDTH
            },
        }
    }
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
    pub fn new(course: Polynomial) -> Self {
        GroundMaterial {
            course,
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