#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::graphics::vertex_buffer::VertexBuffer;

pub struct RendererPlatform<T: glow::Context> {
    pub gl: T,
}

impl<T: glow::Context> RendererPlatform<T> {
    #[cfg(target_arch = "wasm32")]
    pub fn new_webgl2(title: &str) -> RendererPlatform<glow::web::Context> {
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
        let gl = glow::web::Context::from_webgl2_context(webgl2_context);
        RendererPlatform { gl }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_opengl(title: &str) -> RendererPlatform<glow::native::Context> {
        let mut events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new().with_title(title);

        let context_builder = glutin::ContextBuilder::new();

        let window = context_builder
            .build_windowed(window_builder, &events_loop)
            .unwrap();

        let windowed_context = unsafe { window.make_current().unwrap() };

        let context = glow::native::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });

        let render_loop = glow::native::RenderLoop::from_window();
        RendererPlatform { gl: context }
    }
    fn create_program(&self) {
        unsafe {
            let program = self.gl.create_program().expect("Cannot create program");
        }
    }
    pub fn set_vertex_buffer(&self, vertex_buffer: &mut VertexBuffer<T>) {
        vertex_buffer.bind(self);
    }
}
