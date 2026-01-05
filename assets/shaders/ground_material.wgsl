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

struct Polynomial {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

fn f(polynomial: Polynomial, x: f32) -> f32 {
    return polynomial.a * x * x * x + polynomial.b * x * x + polynomial.c * x + polynomial.d;
}

fn f_prime(polynomial: Polynomial, x: f32) -> f32 {
    return 3.0 * polynomial.a * x * x + 2.0 * polynomial.b * x + polynomial.c;
}

fn approx_distance_to_curve(polynomial: Polynomial, p: vec2<f32>) -> f32 {
    let p_y = f(polynomial, p.x);
    let p_d = f_prime(polynomial, p.x);
    let h = sqrt(1 + p_d * p_d);
    return abs((p_y - p.y) / h);
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> ground_material: GroundMaterial;

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

    let distance = approx_distance_to_curve(Polynomial(
        0.00003,
        -0.013,
        1.47,
        0.0,
    ), in.world_position.xz);

    if distance < 10.0 {
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