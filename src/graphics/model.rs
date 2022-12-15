use crate::graphics::material::Material;
use crate::graphics::mesh::Mesh;

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}
impl Model {
    pub fn new(meshes: Vec<Mesh>, materials: Vec<Material>) -> Self {
        return Self { meshes, materials };
    }
}
