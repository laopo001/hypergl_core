use hypergl_core::application::Application;
use hypergl_core::graphics::shader::Shader;

fn main() {
    let app = Application::<glow::native::Context>::new_opengl("123");

    let vertex_shader_source = include_str!("./main2.vert");
    let fragment_shader_source = include_str!("./main2.frag");
    let shader_version = "#version 410";
    let mut shader = Shader::new(
        &app.renderer, 
        format!("{}\n{}", shader_version, vertex_shader_source),
        format!("{}\n{}", shader_version, fragment_shader_source)
    );
    shader.compile();
    shader.link();
}