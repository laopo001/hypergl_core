use crate::graphics::renderer::RendererPlatform;
use glow::RenderLoop;

static FPS: u32 = 60;

pub struct Application<T: glow::Context> {
	pub renderer: RendererPlatform<T>,
	#[cfg(not(target_arch = "wasm32"))]
	pub render_loop: glow::native::RenderLoop,
	#[cfg(target_arch = "wasm32")]
	pub render_loop: glow::web::RenderLoop,
}

impl<T: glow::Context + 'static> Application<T> {
	#[cfg(all(target_arch = "wasm32"))]
	pub fn new_webgl(title: &str) -> Application<glow::web::Context> {
		let render_loop = glow::web::RenderLoop::from_request_animation_frame();
		Application {
			render_loop,
			renderer: RendererPlatform::<T>::new_webgl(title),
		}
	}
	#[cfg(not(target_arch = "wasm32"))]
	pub fn new_opengl(title: &str) -> Application<glow::native::Context> {
		let render_loop = glow::native::RenderLoop::from_window();
		Application {
			render_loop,
			renderer: RendererPlatform::<T>::new_opengl(title),
		}
	}
	pub fn start(self) {
		let gl = self.renderer.gl;
		#[cfg(not(target_arch = "wasm32"))]
			let window = self.renderer.window.unwrap();
		#[cfg(not(target_arch = "wasm32"))]
			let mut events_loop = self.renderer.events_loop.unwrap();
		self.render_loop.run(move |running: &mut bool| unsafe {
			#[cfg(not(target_arch = "wasm32"))] {
				events_loop.poll_events(|event| match event {
					glutin::Event::WindowEvent { event, .. } => match event {
						glutin::WindowEvent::CloseRequested => *running = false,
//							glutin::WindowEvent::Resized(w, h) => window.resize(w, h),
						glutin::WindowEvent::KeyboardInput { input, .. } => match input {
							glutin::KeyboardInput { virtual_keycode, .. } => match virtual_keycode {
								Some(x) => println!("{:?}", x),
								_ => ()
							},
						},
						_ => (),
					},
					_ => (),
				});
				window.swap_buffers();
			}
			gl.clear(glow::COLOR_BUFFER_BIT);
			gl.draw_arrays(glow::TRIANGLES, 0, 3);
		});
	}
	fn tick() {}
}

