use wgpu::util::DeviceExt;

use crate::{app::App, Matrix4, Point3, Vector3, PI};

pub struct Camera {
    eye: Point3,
    target: Point3,
    up: Vector3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    pub bind_group: Option<wgpu::BindGroup>,
}

impl Camera {
    pub fn new(
        eye: Point3,
        target: Point3,
        up: Vector3,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            eye,
            target,
            up,
            aspect,
            fovy,
            znear,
            zfar,
            bind_group: None,
        }
    }
    pub fn build_view_projection_matrix(&self) -> Matrix4 {
        // 1.
        let view = Matrix4::look_at_rh(&self.eye, &self.target, &self.up);
        // 2.
        let proj = Matrix4::new_perspective(self.aspect, self.fovy, self.znear, self.zfar);
        // 3.
        return proj * view;
    }
    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX, // 1
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false, // 2
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });
        return camera_bind_group_layout;
    }
    pub fn bind_group(&mut self, device: &wgpu::Device) {
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(self);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Camera::bind_group_layout(device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });
        self.bind_group = Some(camera_bind_group);
    }
}

// #[rustfmt::skip]
// pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
//     1.0, 0.0, 0.0, 0.0,
//     0.0, 1.0, 0.0, 0.0,
//     0.0, 0.0, 0.5, 0.0,
//     0.0, 0.0, 0.5, 1.0,
// );

#[repr(C)]
// derive 属性自动导入的这些 trait，令其可被存入缓冲区
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    // cgmath 的数据类型不能直接用于 bytemuck
    // 需要先将 Matrix4 矩阵转为一个 4x4 的浮点数数组
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
