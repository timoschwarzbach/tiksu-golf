#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::forward_io::FragmentOutput
#import bevy_pbr::pbr_fragment::pbr_input_from_standard_material
#import bevy_pbr::pbr_functions::apply_pbr_lighting
#import bevy_pbr::pbr_functions::main_pass_post_lighting_processing
#import bevy_pbr::pbr_functions::alpha_discard

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> time: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var normal_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(102) var normal_sampler: sampler;

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {

    var input = in;

    var resize1 = (0.2 * sin(0.05 * time - 30) + 1);
    var resize2 = (0.15 * cos(0.03 * time + 20) + 1);

    var offsetX1 = sin(0.07 * time - 50);
    var offsetX2 = cos(0.05 * time);
    var offsetY1 = cos(0.1 * time + 30);
    var offsetY2 = sin(0.03 * time - 45);


    var uv1 = resize1 * vec2(in.uv.x + offsetX1, in.uv.y + offsetX2);
    var uv2 = resize2 * vec2(in.uv.x + offsetY1, in.uv.y - offsetY2);

    var normal1 = normalize(textureSample(normal_texture, normal_sampler, uv1).xyz * 2.0 - vec3(1.0));
    var normal2 = normalize(textureSample(normal_texture, normal_sampler, uv2).xyz * 2.0 - vec3(1.0));

    var summed_normal = normalize(normal1 + normal2);
    //Normalshift back?
    input.world_normal = normalize(0.85 * input.world_normal + 0.15 * summed_normal);

    var pbr_input = pbr_input_from_standard_material(input, is_front);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    var out: FragmentOutput;

    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    return out;
}