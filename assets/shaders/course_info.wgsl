#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}
#import bevy_render::globals::Globals
#import bevy_render::view::View


@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var material_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return material_color * textureSample(material_color_texture, material_color_sampler, mesh.uv);
}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

@group(0) @binding(0) var<uniform> view: View;
@group(0) @binding(1) var<uniform> globals: Globals;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var time = globals.time * 3;
    out.position = mesh2d_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(
            (vertex.position.x * (1+sin(time+vertex.position.y*5)*(vertex.position.x+0.5)*0.05))*1.2,
            vertex.position.y*(1+0.1*-cos(time)*(vertex.position.x+0.5)) + sin(vertex.position.x*4+time)*(vertex.position.x+0.5)*0.1,
            vertex.position.z,
            1.0
        )
         + vec4<f32>(0.0, 0.0, 0.0, 0.0),
        //  + vec4<f32>(view.viewport.z*-0.5, view.viewport.w*-0.5, 0.0, 0.0),
    );
    out.uv = vertex.uv;
    return out;
}