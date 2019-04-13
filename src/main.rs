use glow::{self, Context, RenderLoop};
use glutin::GlContext;

fn main() {
    unsafe {
        let events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_title("Hello triangle!")
            .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        let context_builder = glutin::ContextBuilder::new().with_vsync(true);
        let window = glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
        let context =
            glow::native::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        window.make_current().unwrap();
        let render_loop = glow::native::RenderLoop::from_window();
    }
}
