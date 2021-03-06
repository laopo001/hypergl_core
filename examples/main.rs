use glow::{self, Context, RenderLoop};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    main();
}

fn main() {
    unsafe {
        // Create a context from a WebGL2 context on wasm32 targets
        #[cfg(target_arch = "wasm32")]
        let (_window, gl, _events_loop, render_loop, shader_version) = {
            use wasm_bindgen::JsCast;
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            let webgl2_context = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            (
                (),
                glow::web::Context::from_webgl2_context(webgl2_context),
                (),
                glow::web::RenderLoop::from_request_animation_frame(),
                "#version 300 es",
            )
        };

        // Create a context from a glutin window on non-wasm32 targets
        #[cfg(not(target_arch = "wasm32"))]
        let (window, gl, mut events_loop, render_loop, shader_version) = {

            let mut events_loop = glutin::EventsLoop::new();
            let window_builder = glutin::WindowBuilder::new().with_title("A fantastic window!");

            let context_builder = glutin::ContextBuilder::new();

            let window = context_builder
                .build_windowed(window_builder, &events_loop)
                .unwrap();

            // let window_builder = glutin::WindowBuilder::new()
            //     .with_title("Hello triangle!")
            //     .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));

            let windowed_context = window.make_current().unwrap();

            println!(
                "Pixel format of the window's GL context: {:?}",
                windowed_context.get_pixel_format()
            );

            let context = glow::native::Context::from_loader_function(|s| {
                windowed_context.get_proc_address(s) as *const _
            });


            let render_loop = glow::native::RenderLoop::from_window();
            (
                windowed_context,
                context,
                events_loop,
                render_loop,
                "#version 410",
            )
        };


        let vertex_shader_source = include_str!("./main1.vert");
        let fragment_shader_source = include_str!("./main1.frag");
        let shader_sources = [
            (
                glow::VERTEX_SHADER,
                format!("{}\n{}", shader_version, vertex_shader_source),
            ),
            (
                glow::FRAGMENT_SHADER,
                format!("{}\n{}", shader_version, fragment_shader_source),
            ),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());
        let program = gl.create_program().expect("Cannot create program");
        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");

            gl.shader_source(shader, shader_source);
            gl.compile_shader(shader);
            // dbg!(gl.get_shader_compile_status(shader), shader_source, shader);
            if !gl.get_shader_compile_status(shader) {
                // dbg!(gl.get_shader_info_log(shader));
                panic!(gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!(gl.get_program_info_log(program));
        }
        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        render_loop.run(move |running: &mut bool| {
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
                window.swap_buffers().unwrap();
            }

            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);

            if !*running {
                gl.delete_program(program);
                gl.delete_vertex_array(vertex_array);
            }
        });
    }
}