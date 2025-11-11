#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}
#import bevy_pbr::mesh_view_bindings::globals


@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> material_color: vec4<f32>;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.blend_color;
}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var time = globals.time * 5;
    var posX = vertex.position.x + 0.5;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(
            vertex.position.x,
            vertex.position.y + sin(0.5*time+posX*2)*posX*0.05,
            vertex.position.z + sin(time+posX*16)*posX*0.03 + sin(time+posX*8)*posX*posX*0.1,
            1.0
        )
    );
    // out.blend_color = vertex.blend_color * material_color;
    out.blend_color = material_color;
    return out;
}
