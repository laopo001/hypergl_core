use crate::Float;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct MeshVertexAttribute {
    pub name: &'static str,
    pub id: u32,
    pub format: wgpu::VertexFormat,
    pub len: usize,
}
impl MeshVertexAttribute {
    pub const fn new(name: &'static str, id: u32, format: wgpu::VertexFormat, len: usize) -> Self {
        return Self {
            name,
            id,
            format,
            len,
        };
    }
}
#[derive(Debug)]
pub struct MeshAttributeData {
    pub attribute: MeshVertexAttribute,
    pub values: Vec<Float>,
}

// #[repr(C)]
// #[derive(Debug)]
// pub struct Vertex {
//     pub data: BTreeMap<u32, MeshVertexAttribute>,
// }

// impl Vertex {
//     pub fn new() -> Self {
//         return Self {
//             data: BTreeMap::new(),
//         };
//     }
//     pub fn desc<'a>(&self) -> wgpu::VertexBufferLayout<'a> {
//         let attrs: Vec<wgpu::VertexAttribute> = vec![];
//         let curr = 0;
//         for (index, attr) in self.data.iter() {
//             attrs.push(wgpu::VertexAttribute {
//                 offset: curr,
//                 shader_location: attr.id,
//                 format: attr.format,
//             });
//             curr += attr.format.size();
//         }

//         wgpu::VertexBufferLayout {
//             array_stride: curr as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &attrs.as_slice(),
//         }
//     }
// }
