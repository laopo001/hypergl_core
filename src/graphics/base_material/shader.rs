use crate::{Mat4, Vec3, Vec4};
use wgpu::util::DeviceExt;
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexUniformInput {
    pub camera_view_proj: Mat4,
    pub model_matrix: Mat4,
}
impl VertexUniformInput {
    pub fn new() -> Self {
        return Self {
            camera_view_proj: Mat4::IDENTITY,
            model_matrix: Mat4::IDENTITY,
        };
    }
    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let vertex_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    // 绑定 0
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX, // 1
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false, // 2
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("vertex_bind_group_layout"),
            });
        return vertex_bind_group_layout;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FragmentUniformInput {
    pub color: Vec4,
}
impl FragmentUniformInput {
    pub fn new() -> Self {
        return Self {
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        };
    }
    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let fragment_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    // 绑定 0
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT, // 1
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false, // 2
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("fragment_bind_group_layout"),
            });
        return fragment_bind_group_layout;
    }
}

#[derive(Debug)]
pub struct BaseShader {
    pub vertex_uniform_input: VertexUniformInput,
    pub vertex_uniform_bind_group: Option<wgpu::BindGroup>,
    pub fragment_uniform_input: FragmentUniformInput,
    pub fragment_uniform_bind_group: Option<wgpu::BindGroup>,
}
impl BaseShader {
    pub fn new() -> Self {
        return Self {
            vertex_uniform_input: VertexUniformInput::new(),
            vertex_uniform_bind_group: None,
            fragment_uniform_input: FragmentUniformInput::new(),
            fragment_uniform_bind_group: None,
        };
    }
    pub fn bind_group(&mut self, device: &wgpu::Device) {
        let vertex_u_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer"),
            contents: bytemuck::cast_slice(&[self.vertex_uniform_input]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let vertex_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &VertexUniformInput::bind_group_layout(device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: vertex_u_buffer.as_entire_binding(),
            }],
            label: Some("vertex_bind_group"),
        });
        self.vertex_uniform_bind_group = Some(vertex_bind_group);

        let frag_u_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer"),
            contents: bytemuck::cast_slice(&[self.fragment_uniform_input]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let frag_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &FragmentUniformInput::bind_group_layout(device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: frag_u_buffer.as_entire_binding(),
            }],
            label: Some("fragment_bind_group"),
        });
        self.fragment_uniform_bind_group = Some(frag_bind_group);
    }
}
