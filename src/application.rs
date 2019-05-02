#[cfg(target_arch = "wasm32")]
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlRenderbuffer,
    WebGlRenderingContext, WebGlSampler, WebGlShader, WebGlSync, WebGlTexture,
    WebGlUniformLocation, WebGlVertexArrayObject,
};

#[derive(Debug)]
pub struct Application<T: glow::Context> {
    gl: T,
}

impl<T: glow::Context> Application<T> {
    #[cfg(target_arch = "wasm32")]
    fn new_webgl1(context: WebGlRenderingContext) -> Application<glow::web::Context> {
        let gl = glow::web::Context::from_webgl1_context(context);
        Application { gl }
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn new_opengl(title: &str) -> Application<glow::native::Context> {
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
        Application { gl: context }
    }
    fn test(&self) {
        unsafe {
            let program = self.gl.create_program().expect("Cannot create program");
        }

    }
}


#[test]
fn test() {}
