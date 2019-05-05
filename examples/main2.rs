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
    let app = Application::<glow::native::Context>::new_opengl("123");
    #[cfg(not(target_arch = "wasm32"))]
    let shader_version = "#version 410";

    #[cfg(target_arch = "wasm32")]
    let app = Application::<glow::web::Context>::new_webgl2("123");
    #[cfg(target_arch = "wasm32")]
    let shader_version = "#version 300 es";

    let vertex_shader_source = include_str!("./main2.vert");
    let fragment_shader_source = include_str!("./main2.frag");
    let mut shader = Shader::new(
        &app.renderer,
        format!("{}\n{}", shader_version, vertex_shader_source),
        format!("{}\n{}", shader_version, fragment_shader_source),
    );
    app.renderer.set_shader_program(&mut shader);
    let gl = app.renderer.gl;
    #[cfg(not(target_arch = "wasm32"))]
    let window = app.renderer.window.unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    let mut events_loop = app.renderer.events_loop.unwrap();
    // unsafe {
    //     gl.clear(glow::COLOR_BUFFER_BIT);
    //     gl.draw_arrays(glow::TRIANGLES, 0, 3);
    // }

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