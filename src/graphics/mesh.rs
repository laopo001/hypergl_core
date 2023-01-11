use crate::node::Node;
use crate::Float;
use std::collections::BTreeMap;
use wgpu::util::DeviceExt;

use super::vertex::{MeshAttributeData, MeshVertexAttribute};

#[derive(Debug)]
pub struct Mesh {
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub material_index: Option<usize>,
    pub indices: Option<Vec<u32>>,
    pub attribute_map: BTreeMap<u32, MeshAttributeData>,
    pub attrs_desc: Vec<wgpu::VertexAttribute>,
    pub stride: wgpu::BufferAddress,
}

impl Mesh {
    pub fn new() -> Self {
        return Self {
            vertex_buffer: None,
            index_buffer: None,
            material_index: None,
            indices: None,
            attribute_map: BTreeMap::new(),
            attrs_desc: vec![],
            stride: 0 as wgpu::BufferAddress,
        };
    }
    pub const ATTRIBUTE_POSITION: MeshVertexAttribute =
        MeshVertexAttribute::new("Vertex_Position", 0, wgpu::VertexFormat::Float32x3, 3);

    pub const ATTRIBUTE_NORMAL: MeshVertexAttribute =
        MeshVertexAttribute::new("Vertex_Normal", 1, wgpu::VertexFormat::Float32x3, 3);

    pub const ATTRIBUTE_UV_0: MeshVertexAttribute =
        MeshVertexAttribute::new("Vertex_Uv", 2, wgpu::VertexFormat::Float32x2, 2);

    pub const ATTRIBUTE_TANGENT: MeshVertexAttribute =
        MeshVertexAttribute::new("Vertex_Tangent", 3, wgpu::VertexFormat::Float32x4, 4);

    pub const ATTRIBUTE_COLOR: MeshVertexAttribute =
        MeshVertexAttribute::new("Vertex_Color", 4, wgpu::VertexFormat::Float32x4, 4);

    pub const ATTRIBUTE_JOINT_WEIGHT: MeshVertexAttribute =
        MeshVertexAttribute::new("Vertex_JointWeight", 5, wgpu::VertexFormat::Float32x4, 4);
    pub const ATTRIBUTE_JOINT_INDEX: MeshVertexAttribute =
        MeshVertexAttribute::new("Vertex_JointIndex", 6, wgpu::VertexFormat::Uint16x4, 4);

    pub fn insert_attribute(&mut self, attribute: MeshVertexAttribute, values: Vec<Float>) {
        self.attribute_map
            .insert(attribute.id, MeshAttributeData { attribute, values });
        let mut attrs: Vec<wgpu::VertexAttribute> = vec![];
        let mut curr = 0;
        for (index, attr) in self.attribute_map.iter() {
            attrs.push(wgpu::VertexAttribute {
                offset: curr,
                shader_location: attr.attribute.id,
                format: attr.attribute.format,
            });
            curr += attr.attribute.format.size();
        }
        self.attrs_desc = attrs;
        self.stride = curr as wgpu::BufferAddress;
    }
    pub fn insert_attribute_iter(
        &mut self,
        attribute: MeshVertexAttribute,
        values: impl IntoIterator<Item = impl IntoIterator<Item = Float>>,
    ) {
        let mut values2: Vec<Float> = vec![];
        for x in values.into_iter() {
            for f in x.into_iter() {
                values2.push(f);
            }
        }
        self.attribute_map.insert(
            attribute.id,
            MeshAttributeData {
                attribute,
                values: values2,
            },
        );
        let mut attrs: Vec<wgpu::VertexAttribute> = vec![];
        let mut curr = 0;
        for (index, attr) in self.attribute_map.iter() {
            attrs.push(wgpu::VertexAttribute {
                offset: curr,
                shader_location: attr.attribute.id,
                format: attr.attribute.format,
            });
            curr += attr.attribute.format.size();
        }
        self.attrs_desc = attrs;
        self.stride = curr as wgpu::BufferAddress;
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = Some(indices);
    }
    pub fn desc<'a>(&'a self) -> wgpu::VertexBufferLayout<'a> {
        // self.attrs_desc = attrs;
        wgpu::VertexBufferLayout {
            array_stride: self.stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &self.attrs_desc,
        }
    }
    pub fn create_buffer(&mut self, device: &wgpu::Device) {
        let mut vertices: Vec<Float> = vec![];

        for index in 0..(self.attribute_map.get(&0).unwrap().values.len() / 3) {
            for (_, attr) in self.attribute_map.iter() {
                vertices.extend_from_slice(
                    attr.values
                        .get(index * attr.attribute.len..(index + 1) * attr.attribute.len)
                        .unwrap(),
                )
            }
        }

        // dbg!(&vertices);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(self.indices.as_ref().unwrap()),
            usage: wgpu::BufferUsages::INDEX,
        });
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
    }

    pub fn create_box(x_length: f32, y_length: f32, z_length: f32) -> Self {
        let max_x = x_length / 2.0;
        let min_x = -x_length / 2.0;
        let max_y = y_length / 2.0;
        let min_y = -y_length / 2.0;
        let max_z = z_length / 2.0;
        let min_z = -z_length / 2.0;
        let vertices = &[
            // Front
            ([min_x, min_y, max_z], [0., 0., 1.0], [0., 0.]),
            ([max_x, min_y, max_z], [0., 0., 1.0], [1.0, 0.]),
            ([max_x, max_y, max_z], [0., 0., 1.0], [1.0, 1.0]),
            ([min_x, max_y, max_z], [0., 0., 1.0], [0., 1.0]),
            // Back
            ([min_x, max_y, min_z], [0., 0., -1.0], [1.0, 0.]),
            ([max_x, max_y, min_z], [0., 0., -1.0], [0., 0.]),
            ([max_x, min_y, min_z], [0., 0., -1.0], [0., 1.0]),
            ([min_x, min_y, min_z], [0., 0., -1.0], [1.0, 1.0]),
            // Right
            ([max_x, min_y, min_z], [1.0, 0., 0.], [0., 0.]),
            ([max_x, max_y, min_z], [1.0, 0., 0.], [1.0, 0.]),
            ([max_x, max_y, max_z], [1.0, 0., 0.], [1.0, 1.0]),
            ([max_x, min_y, max_z], [1.0, 0., 0.], [0., 1.0]),
            // Left
            ([min_x, min_y, max_z], [-1.0, 0., 0.], [1.0, 0.]),
            ([min_x, max_y, max_z], [-1.0, 0., 0.], [0., 0.]),
            ([min_x, max_y, min_z], [-1.0, 0., 0.], [0., 1.0]),
            ([min_x, min_y, min_z], [-1.0, 0., 0.], [1.0, 1.0]),
            // Top
            ([max_x, max_y, min_z], [0., 1.0, 0.], [1.0, 0.]),
            ([min_x, max_y, min_z], [0., 1.0, 0.], [0., 0.]),
            ([min_x, max_y, max_z], [0., 1.0, 0.], [0., 1.0]),
            ([max_x, max_y, max_z], [0., 1.0, 0.], [1.0, 1.0]),
            // Bottom
            ([max_x, min_y, max_z], [0., -1.0, 0.], [0., 0.]),
            ([min_x, min_y, max_z], [0., -1.0, 0.], [1.0, 0.]),
            ([min_x, min_y, min_z], [0., -1.0, 0.], [1.0, 1.0]),
            ([max_x, min_y, min_z], [0., -1.0, 0.], [0., 1.0]),
        ];
        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();

        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        let indices = vec![
            0, 1, 2, 2, 3, 0, // front
            4, 5, 6, 6, 7, 4, // back
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // top
            20, 21, 22, 22, 23, 20, // bottom
        ];
        let mut mesh = Mesh::new();
        mesh.insert_attribute_iter(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute_iter(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute_iter(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(indices);
        mesh
    }
}
