use std::ptr::NonNull;

use crate::{ecs::entity::Entity, Float, Mat4};
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct CameraComponent {
    pub proj_martix: Mat4,
    pub bind_group: Option<wgpu::BindGroup>,
    pub entity: Option<NonNull<Entity>>,
}

impl CameraComponent {
    pub fn new_perspective(aspect: Float, fovy: Float, znear: Float, zfar: Float) -> Self {
        Self {
            proj_martix: Mat4::perspective_rh(aspect, fovy, znear, zfar),
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
            proj_martix: Mat4::orthographic_lh(left, right, bottom, top, znear, zfar),
            bind_group: None,
            entity: None,
        }
    }
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        unsafe {
            return self.proj_martix
                * self
                    .entity
                    .unwrap()
                    .as_mut()
                    .get_world_matrix()
                    .clone()
                    .inverse();
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
        let matrix = self.build_view_projection_matrix();
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[matrix]),
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
