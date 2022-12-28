use std::ptr::NonNull;

use crate::{ecs::entity::Entity, Float, Matrix4};
use wgpu::util::DeviceExt;

pub struct CameraComponent {
    pub proj_martix: Matrix4,
    pub bind_group: Option<wgpu::BindGroup>,
    pub entity: Option<NonNull<Entity>>,
}

impl CameraComponent {
    pub fn new_perspective(aspect: Float, fovy: Float, znear: Float, zfar: Float) -> Self {
        Self {
            proj_martix: Matrix4::new_perspective(aspect, fovy, znear, zfar),
            bind_group: None,
            entity: None,
        }
    }
    pub fn new_orthographic(
        left: Float,
        right: Float,
        bottom: Float,
        top: Float,
        znear: Float,
        zfar: Float,
    ) -> Self {
        Self {
            proj_martix: Matrix4::new_orthographic(left, right, bottom, top, znear, zfar),
            bind_group: None,
            entity: None,
        }
    }
    pub fn build_view_projection_matrix(&self) -> Matrix4 {
        unsafe {
            return self.proj_martix * self.entity.unwrap().as_mut().get_world_matrix();
        }
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
            layout: &CameraComponent::bind_group_layout(device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });
        self.bind_group = Some(camera_bind_group);
    }
}

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
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &CameraComponent) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
