use crate::graphics::renderer::RendererPlatform;

pub struct Application<T: glow::Context> {
    pub renderer: RendererPlatform<T>,
}

impl<T: glow::Context> Application<T> {
    #[cfg(all(target_arch = "wasm32", feature = "webgl1"))]
    pub fn new_webgl1(title: &str) -> Application<impl glow::Context> {
        Application {
            renderer: RendererPlatform::<T>::new_webgl1(title),
        }
    }
    #[cfg(all(target_arch = "wasm32", not(feature = "webgl1")))]
    pub fn new_webgl2(title: &str) -> Application<glow::web::Context> {
        Application {
            renderer: RendererPlatform::<T>::new_webgl2(title),
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_opengl(title: &str) -> Application<impl glow::Context> {
        Application {
            renderer: RendererPlatform::<T>::new_opengl(title),
        }
    }
}

