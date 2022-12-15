use hyper_rust::app::App;
use hyper_rust::node::Node;

async fn run() {
    let mut node = Node::new();

    node.set_local_position(1., 1., 1.);
    node.set_local_euler_angle(0.5 * std::f32::consts::PI, 0., 0.);
    node.set_local_scale(1., 2., 1.);
    // dbg!(&node.get_matrix());
    let mut child = Node::new();
    child.set_local_position(0., 2., 0.);
    node.add_child(child);
    dbg!(node.children[0].get_world_matrix());
    dbg!(node.children[0].get_position());
    let app = App::new().await;
    app.start().await;
}
fn main() {
    pollster::block_on(run());
}
