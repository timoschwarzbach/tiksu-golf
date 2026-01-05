#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct GroundMaterial {
    quantize_steps: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> ground_material: GroundMaterial;

fn distance_to_curve(origin: vec2<f32>, x_abcd: vec4<f32>, y_abcd: vec4<f32>, maximum: f32) -> f32 {
    var minimum: f32 = maximum;
    var min_idx = 0;
    for (var t_i32 = 0; t_i32 <= 30; t_i32 += 1) {
        let t: f32 = f32(t_i32) / 30.0;
        let x: f32 = x_abcd.x * t * t * t + x_abcd.y * t * t + x_abcd.z * t + x_abcd.w;
        let y: f32 = y_abcd.x * t * t * t + y_abcd.y * t * t + y_abcd.z * t + y_abcd.w;
        let sqr_d: f32 = (origin.x - x) * (origin.x - x) + (origin.y - y) * (origin.y - y);

        if sqr_d < minimum {
            minimum = sqr_d;
            min_idx = t_i32;
        }
    }

    for (var t = -15; t <= 15; t += 1) {
        let t: f32 = (f32(min_idx) + f32(t) / 15.0) / 30.0;
        if t < 0.0 || t > 1.0 {
            break;
        }

        let x: f32 = x_abcd.x * t * t * t + x_abcd.y * t * t + x_abcd.z * t + x_abcd.w;
        let y: f32 = y_abcd.x * t * t * t + y_abcd.y * t * t + y_abcd.z * t + y_abcd.w;
        let sqr_d: f32 = (origin.x - x) * (origin.x - x) + (origin.y - y) * (origin.y - y);

        if sqr_d < minimum {
            minimum = sqr_d;
        }
    }

    return minimum;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    let distance = distance_to_curve(in.world_position.xz, vec4(-1130.0, 1740.0, -360.0, 0.0), vec4(1320.0, -2130.0, 1170.0, 0.0), 200.0);

    if distance < 190.0 {
        // we can optionally modify the lit color before post-processing is applied
        out.color = vec4<f32>(vec4<u32>(out.color * f32(ground_material.quantize_steps))) / f32(ground_material.quantize_steps);
    }
    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    // we can optionally modify the final result here
    out.color = out.color * 2.0;
#endif

    return out;
}