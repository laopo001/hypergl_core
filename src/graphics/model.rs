use std::any::type_name;
use std::ops::Range;

use crate::app::App;

use crate::graphics::base_material::material::Material;
use crate::graphics::mesh::Mesh;

#[derive(Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}
impl Model {
    pub fn new(meshes: Vec<Mesh>, materials: Vec<Material>) -> Self {
        return Self { meshes, materials };
    }
    pub fn create_pentagon(material: Material) -> Self {
        let mut mesh = Mesh::new();
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                //
                -0.0868241,
                0.49240386,
                0.0,
                //
                -0.49513406,
                0.06958647,
                0.0,
                //
                -0.21918549,
                -0.44939706,
                0.0,
                //
                0.35966998,
                -0.3473291,
                0.0,
                //
                0.44147372,
                0.2347359,
                0.0,
            ],
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                //
                0.4131759,
                0.00759614,
                //
                0.0048659444,
                0.43041354,
                //
                0.28081453,
                0.949397,
                //
                0.85967,
                0.84732914,
                //
                0.9414737,
                0.2652641,
            ],
        );
        mesh.set_indices(vec![0, 1, 4, 1, 2, 4, 2, 3, 4]);
        let mut model = Model {
            meshes: vec![],
            materials: vec![],
        };

        model.materials.push(material);
        mesh.material_index = Some(0);
        model.meshes.push(mesh);

        return model;
    }
    pub fn create_triangle(material: Material) -> Self {
        let mut mesh = Mesh::new();
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0],
        );

        mesh.set_indices(vec![0, 1, 2]);
        let mut model = Model {
            meshes: vec![],
            materials: vec![],
        };

        model.materials.push(material);
        mesh.material_index = Some(0);
        model.meshes.push(mesh);

        return model;
    }
}

pub trait DrawModel<'a> {
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_mesh_new(&mut self, mesh: &'a Mesh, material: &'a Material);
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        camera_bind_group: &'b wgpu::BindGroup,
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
    ) {
        // self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));

        // self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        // self.set_bind_group(0, &material.bind_group, &[]);
        // self.set_bind_group(1, camera_bind_group, &[]);

        // self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
    fn draw_mesh_new(&mut self, mesh: &'b Mesh, material: &'b Material) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.as_ref().unwrap().slice(..));

        self.set_index_buffer(
            mesh.index_buffer.as_ref().unwrap().slice(..),
            wgpu::IndexFormat::Uint32,
        );

        self.set_bind_group(
            0,
            material.shader.vertex_uniform_bind_group.as_ref().unwrap(),
            &[],
        );
        self.set_bind_group(
            1,
            material
                .shader
                .fragment_uniform_bind_group
                .as_ref()
                .unwrap(),
            &[],
        );

        self.draw_indexed(0..mesh.indices.as_ref().unwrap().len() as u32, 0, 0..1);
    }
}
