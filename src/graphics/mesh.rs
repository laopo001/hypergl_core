use wgpu::util::DeviceExt;

use crate::graphics::vertex::Vertex;
use crate::node::Node;
pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material_index: Option<usize>,
    pub node: *mut Node,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, VERTICES: &[Vertex], INDICES: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        // 新添加!
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        return Self {
            vertex_buffer,
            index_buffer,
            num_elements: INDICES.len() as u32,
            material_index: None,
            node: std::ptr::null_mut(),
        };
    }
}
