#import bevy_pbr::forward_io::{VertexOutput}
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}
#import bevy_pbr::mesh_view_bindings::globals
#import bevy_pbr::pbr_functions::apply_pbr_lighting
#import bevy_pbr::pbr_fragment::pbr_input_from_standard_material

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) is_front: bool) -> @location(0) vec4<f32> {
    // var pbr_input = pbr_input_from_standard_material(input, is_front);
    let time = globals.time * 1.0;
    if (input.position.z*200+time)%0.8 <= 0.2 {
        // return input.color;
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    // return input.color;
    return vec4<f32>(1.0, 0.0, 0.0, 0.7);

}

