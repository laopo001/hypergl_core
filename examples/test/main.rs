use hyper_rust::app::App;

async fn run() {
    let app = App::new().await;
    app.start().await;
}
fn main() {
    pollster::block_on(run());
}
