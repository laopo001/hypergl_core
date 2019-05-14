use hypergl_core::application::Application;
use hypergl_core::config;
use hypergl_core::graphics::vertex_buffer::VertexBuffer;
use hypergl_core::graphics::shader::Shader;
use hypergl_core::graphics::vertex_format::{VertexFormat, VertexType};
use hypergl_core::graphics::shader_variable::GL_Location;
use hypergl_core::utils::{console_error, console_log};
// use glow::native::Context;
use glow::{Context, RenderLoop};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

fn main() {
    unsafe {
        #[cfg(not(target_arch = "wasm32"))]
        type Context = glow::native::Context;
        #[cfg(not(target_arch = "wasm32"))]
        let mut app = Application::<glow::native::Context>::new_opengl("123");

        #[cfg(target_arch = "wasm32")]
        type Context = glow::web::Context;
        #[cfg(target_arch = "wasm32")]
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

        let vertexs = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        app.renderer.set_shader_program(&mut shader);

        
            let vertex_array = app.renderer.gl.create_vertex_array().expect("Cannot create vertex array");
            app.renderer.gl.bind_vertex_array(Some(vertex_array));

            let arr: Vec<VertexType> = vec![VertexType::new(
                config::SEMANTIC::POSITION("position".to_string()),
                3,
                false,
            )];
            let format = VertexFormat::new(arr);
            let mut vertexbuffer = VertexBuffer::<Context>::new(
                format,
                3,
                glow::STATIC_DRAW,
                Box::new(vertexs),
            );
            vertexbuffer.bind(&app.renderer);
            for attrbute in shader.attributes.iter() {
                let element = vertexbuffer
                    .format
                    .elements
                    .iter()
                    .find(|&x| {
                        // console_log(attrbute.name.to_string());
                        return x.semantic.to_string() == attrbute.name;
                    })
                    .expect("不为None");
                match attrbute.location_id {
                    GL_Location::AttribLocation(u) => {
                        app.renderer.gl.vertex_attrib_pointer_f32(
                            u,
                            element.size as i32,
                            glow::FLOAT,
                            element.normalize,
                            element.stride as i32,
                            element.offset as i32,
                        );
                        app.renderer.gl.enable_vertex_attrib_array(u);
                    }
                    _ => {
                        panic!("error");
                    }
                }
            }
            let location = app
                .renderer
                .gl
                .get_uniform_location(shader.program.unwrap(), "matrix");

            app.renderer.gl.uniform_matrix_4_f32_slice(
                location,
                false,
                &[
                    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                ],
            );
        


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

                gl.clear(glow::COLOR_BUFFER_BIT);
                gl.draw_arrays(glow::TRIANGLES, 0, 3);
                // gl.delete_vertex_array(vertex_array);
            
        });
    }
}