
use crate::graphics::shader::Shader;
use crate::graphics::vertex_buffer::VertexBuffer;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
pub struct RendererPlatform<T: glow::Context> {
    pub gl: T,
    #[cfg(not(target_arch = "wasm32"))]
    pub window: Option<glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::Window>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub events_loop: Option<glutin::EventsLoop>,
    #[cfg(not(target_arch = "wasm32"))]
    pub render_loop: glow::native::RenderLoop,

    #[cfg(target_arch = "wasm32")]
    pub window: Option<()>,
    #[cfg(target_arch = "wasm32")]
    pub events_loop: Option<()>,

    #[cfg(target_arch = "wasm32")]
    pub render_loop: glow::web::RenderLoop,

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
        let render_loop = glow::web::RenderLoop::from_request_animation_frame();
        let gl = glow::web::Context::from_webgl2_context(webgl2_context);

        RendererPlatform {
            gl,
            window: None,
            events_loop: None,
            render_loop,
        }
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

        RendererPlatform {
            gl: context,
            window: Some(windowed_context),
            events_loop: Some(events_loop),
            render_loop,
        }
    }
    pub fn set_shader_program(&self, shader: &mut Shader<T>) {
        shader.link();
        unsafe {
            self.gl.use_program(shader.program);
        }
    }
    // setShaderProgram(shader: Shader) {
    //     this.currShader = shader;
    //     if (shader.ready === false) {
    //         shader.link();
    //     }
    //     this.gl.useProgram(shader.program as WebGLProgram);
    // }
    pub fn set_vertex_buffer(&self, vertex_buffer: &mut VertexBuffer<T>) {
        vertex_buffer.bind(self);
    }
}

