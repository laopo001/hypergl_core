// 顶点着色器

struct UniformInput {
    camera_view_proj: mat4x4<f32>,
    model_matrix: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> u: UniformInput;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var posW = u.model_matrix * vec4<f32>(model.position, 1.0);

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = u.camera_view_proj * posW;
    return out;
}

// 片元着色器

@group(1) @binding(0)
var<uniform> color: vec3<f32>;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0);
}
