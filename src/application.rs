use crate::graphics::renderer::RendererPlatform;

pub struct Application<T: glow::Context> {
    pub renderer: RendererPlatform<T>,
}

impl<T: glow::Context> Application<T> {
    // #[cfg(target_arch = "wasm32")]
    // fn new_webgl1(title: &str) -> Application<glow::web::Context> {
    //     Application {
    //         renderer: RendererPlatform::<glow::web::Context>::new_webgl1(title),
    //     }
    // }
    #[cfg(target_arch = "wasm32")]
    pub fn new_webgl2(title: &str) -> Application<glow::web::Context> {
        Application {
            renderer: RendererPlatform::<glow::web::Context>::new_webgl2(title),
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_opengl(title: &str) -> Application<glow::native::Context> {
        Application {
            renderer: RendererPlatform::<glow::native::Context>::new_opengl(title),
        }
    }
}

