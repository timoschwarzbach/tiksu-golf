#import bevy_pbr::forward_io::VertexOutput
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

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var time = globals.time * 5;
    var posX = vertex.position.x + 0.5;
    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(
            vertex.position.x,
            vertex.position.y + sin(0.5*time+posX*2)*posX*0.05,
            vertex.position.z + sin(time+posX*16)*posX*0.03 + sin(time+posX*8)*posX*posX*0.1,
            1.0
        )
    );
    return out;
}
