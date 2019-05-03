use crate::graphics::renderer::RendererPlatform;

pub struct Application<T: glow::Context> {
    renderer: RendererPlatform<T>,
}

impl<T: glow::Context> Application<T> {
    #[cfg(target_arch = "wasm32")]
    fn new_webgl1(title: &str) -> Application<glow::web::Context> {
        Application {
            renderer: RendererPlatform::<glow::web::Context>::new_webgl1(title),
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn new_opengl(title: &str) -> Application<glow::native::Context> {
        Application {
            renderer: RendererPlatform::<glow::native::Context>::new_opengl("123"),
        }
    }
}

