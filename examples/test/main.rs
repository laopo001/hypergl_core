use hyper_rust::app::App;

use hyper_rust::ecs::components::camera::CameraComponent;
use hyper_rust::ecs::components::model::ModelComponent;
use hyper_rust::ecs::entity::Entity;
use hyper_rust::graphics::base_material::material::Material;
use hyper_rust::graphics::mesh::Mesh;
use hyper_rust::graphics::model::Model;
use hyper_rust::graphics::texture::Texture;
use hyper_rust::node::{Node, NodeTrait};
use hyper_rust::Vec3;
use winit::event_loop::EventLoop;
async fn run() -> anyhow::Result<()> {
    let mut app = App::new(450, 400).await;
    app.init();

    // let diffuse_bytes = include_bytes!("./cube/cube-diffuse.jpg");
    // let diffuse_texture = Texture::from_bytes(&app, diffuse_bytes, "label")?;
    // let mat = Material::new(&app, "t".to_string(), diffuse_texture);
    let mut material = Material::new();
    material.set_color(Vec3::new(1., 0., 0.));
    let model = Model::create_triangle(material);
    let mut plane = Entity::new("plane");
    plane.add_model(ModelComponent::new(model));
    plane.set_local_position(0., 0., 0.0);

    let mut camera = Entity::new("camera");
    camera.add_camera(CameraComponent::new_perspective(
        1.0,
        0.25 * std::f32::consts::PI,
        0.1,
        100.0,
    ));
    camera.set_local_position(2.0, 2.0, 2.0);
    camera.look_at(Vec3::new(0., 0., 0.), Vec3::new(0., 1.0, 0.));
    app.root.add_child(camera);
    app.root.add_child(plane);

    app.start().await;
    Ok(())
}
fn main() {
    pollster::block_on(run());
}
