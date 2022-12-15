use hyper_rust::app::App;
use hyper_rust::graphics::mesh::Mesh;
use hyper_rust::graphics::model::Model;
use hyper_rust::graphics::texture::Texture;
use hyper_rust::node::Node;

async fn run() -> anyhow::Result<()> {
    let app = App::new().await;
    let diffuse_bytes = include_bytes!("./cube/cube-diffuse.jpg");
    let diffuse_texture = Texture::from_bytes(&app.device, &app.queue, diffuse_bytes, "label")?;

    app.start().await;
    Ok(())
}
fn main() {
    pollster::block_on(run());
}
