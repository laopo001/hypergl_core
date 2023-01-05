// 顶点着色器

struct VertexUniformInput {
    camera_view_proj: mat4x4<f32>,
    model_matrix: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> v_u: VertexUniformInput;


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
    var posW = v_u.model_matrix * vec4<f32>(model.position, 1.0);

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = v_u.camera_view_proj * posW;
    return out;
}

// 片元着色器

struct FragmentUniformInput {
    color: vec4<f32>,
}


@group(1) @binding(0)
var<uniform> f_u: FragmentUniformInput;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var base_color = vec3<f32>(f_u.color.x, f_u.color.y, f_u.color.z);
    var opacity = f_u.color.w;
    return vec4<f32>(base_color, opacity);
}
