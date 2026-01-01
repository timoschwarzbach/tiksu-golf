#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_functions
#import bevy_pbr::view_transformations::position_world_to_clip

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var material_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var material_color_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> internal_time: f32;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var model = mesh_functions::get_world_from_local(vertex.instance_index);
    var world_pos = mesh_functions::mesh_position_local_to_world(
        model,
        vec4<f32>(vertex.position, 1.0)
    );

    let wave = sin((vertex.position.x * 5.0 + vertex.position.z * 4.0) + 2.0 * internal_time) * 0.2;

    world_pos.y = vertex.position.y + wave;

    out.world_position = world_pos;
    out.position = position_world_to_clip(world_pos.xyz);

    var scrolling: vec2<f32> = vec2(vertex.uv.x + internal_time, vertex.uv.y + internal_time);

    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    //return vec4(0.0);
    return textureSample(material_color_texture, material_color_sampler, mesh.uv);
}