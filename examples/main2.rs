use hypergl_core::application::Application;
use hypergl_core::graphics::shader::Shader;
// use glow::native::Context;
use glow::{Context, RenderLoop};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

fn main() {

    #[cfg(not(target_arch = "wasm32"))]
    let mut app = Application::<glow::native::Context>::new_opengl("123");

    #[cfg(all(target_arch = "wasm32", feature = "webgl1"))]
    let mut app = Application::<glow::web::Context>::new_webgl1("123");

    #[cfg(all(target_arch = "wasm32", feature = "webgl2"))]
    let mut app = Application::<glow::web::Context>::new_webgl2("123");

    let shader_version = "#version 300 es";

    let vertex_shader_source = include_str!("./main2.vert");
    let fragment_shader_source = include_str!("./main2.frag");
    #[cfg(not(feature = "webgl1"))]
    let mut shader = Shader::new(
        &app.renderer,
        format!("{}\n{}", shader_version, vertex_shader_source),
        format!("{}\n{}", shader_version, fragment_shader_source),
    );
    #[cfg(all(feature = "webgl1"))]
    let mut shader = Shader::new(
        &app.renderer,
        vertex_shader_source.to_string(),
        fragment_shader_source.to_string(),
    );

    app.renderer.set_shader_program(&mut shader);

    // app.renderer.set_view_port(0, 0, 500, 300);
    app.renderer.set_scissor(0, 0, 600, 400);
    unsafe {
        app.renderer.gl.enable(glow::SCISSOR_TEST);
    }

    let gl = app.renderer.gl;
    #[cfg(not(target_arch = "wasm32"))]
    let window = app.renderer.window.unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    let mut events_loop = app.renderer.events_loop.unwrap();

    app.renderer.render_loop.run(move |running: &mut bool| {
        // Handle events differently between targets
        #[cfg(not(target_arch = "wasm32"))]
        {
            events_loop.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => *running = false,
                    _ => (),
                },
                _ => (),
            });
            window.swap_buffers();
        }
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }

    });
}