#import bevy_sprite::mesh2d_view_bindings

struct CreatureMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> mat: CreatureMaterial;
@group(1) @binding(1)
var base: texture_2d<f32>;
@group(1) @binding(2)
var base_sampler: sampler;

@fragment
fn fragment(
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    var t_base = textureSample(base, base_sampler, uv);

    return t_base;
}
