use hyper_rust::app::App;
use hyper_rust::camera::Camera;
use hyper_rust::ecs::components::camera::CameraComponent;
use hyper_rust::ecs::entity::Entity;
use hyper_rust::graphics::material::Material;
use hyper_rust::graphics::mesh::Mesh;
use hyper_rust::graphics::model::Model;
use hyper_rust::graphics::texture::Texture;
use hyper_rust::node::{Node, NodeTrait};
use hyper_rust::{Point3, Vector3};
use winit::event_loop::EventLoop;
async fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let mut app = App::new(&event_loop, 450, 400).await;
    app.init();
    // println!("{:p}", &app);
    let diffuse_bytes = include_bytes!("./cube/cube-diffuse.jpg");
    let diffuse_texture = Texture::from_bytes(&app, diffuse_bytes, "label")?;
    let mat = Material::new(&app, "t".to_string(), diffuse_texture);
    let model = Model::create_plane(&app, mat);
    let mut e = Entity::new("test");
    e.add_camera(CameraComponent::new_perspective(
        1.0,
        0.25 * std::f32::consts::PI,
        0.1,
        100.0,
    ));
    e.set_local_position(2.0, 2.0, 2.0);
    e.lookat(Point3::new(0., 0., 0.), Vector3::new(0., 1., 0.));
    app.root.add_child(Box::new(e));

    app.root.sync();

    // app.system.test.push(1);
    // dbg!(app.system.cameras.len());
    // let camera = Camera::new(
    //     [2.0, 2.0, 2.0].into(),
    //     [0.0, 0.0, 0.0].into(),
    //     [0.0, 1.0, 0.0].into(),
    //     1.0,
    //     0.25 * std::f32::consts::PI,
    //     0.1,
    //     100.0,
    // );
    // app.camera = Some(camera);
    app.start(event_loop, model).await;
    Ok(())
}
fn main() {
    pollster::block_on(run());
}
