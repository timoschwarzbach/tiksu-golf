#import bevy_render::globals::Globals
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}
#import bevy_pbr::prepass_io

@group(0) @binding(1) var<uniform> globals: Globals;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> prepass_io::VertexOutput {
    var out: prepass_io::VertexOutput;

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
