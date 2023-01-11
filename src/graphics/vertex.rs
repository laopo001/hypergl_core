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
