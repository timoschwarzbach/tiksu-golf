#import bevy_pbr::forward_io::{VertexOutput, Vertex}
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}
#import bevy_pbr::mesh_view_bindings::globals
#import bevy_pbr::pbr_functions::apply_pbr_lighting
#import bevy_pbr::pbr_fragment::pbr_input_from_standard_material


@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> material_color: vec4<f32>;

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) is_front: bool) -> @location(0) vec4<f32> {
    var pbr_input = pbr_input_from_standard_material(input, is_front);
    pbr_input.material.base_color = material_color;
    return apply_pbr_lighting(pbr_input);
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let time = globals.time * 3.5;
    let influence = saturate(vertex.position.x+0.5);
    let main_wave = sin(time + vertex.position.x * 2.0) * 0.15;

    let flap = sin(time * 2.5 + vertex.position.x * 12.0) * 0.04;
    let ripples = sin(time * 1.8 + vertex.position.x * 25.0 + vertex.position.y * 10.0) * 0.02;

    let displacement_z = (main_wave + flap + ripples) * influence;
    let displacement_y = (sin(time * 0.5 + vertex.position.x) * 0.05) * influence;

    let displaced_position = vec4<f32>(
        vertex.position.x, 
        vertex.position.y + displacement_y, 
        vertex.position.z + displacement_z, 
        1.0
    );

    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        displaced_position
    );

    return out;
}
