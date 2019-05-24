use crate::graphics::shader::Shader;
use crate::graphics::vertex_buffer::VertexBuffer;
use crate::graphics::index_buffer::IndexBuffer;
use crate::utils::{console_log, console_error};
use crate::config::ACTIVE_INFO_TYPE;
use crate::graphics::drawable::Drawable;
use crate::graphics::shader_variable::GL_Location;
use std::collections::HashMap;
use crate::config;


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct RendererPlatform<T: glow::Context> {
	pub gl: T,
	#[cfg(not(target_arch = "wasm32"))]
	pub window: Option<glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::Window>>,
	#[cfg(not(target_arch = "wasm32"))]
	pub events_loop: Option<glutin::EventsLoop>,

	pub clear_color: [f32; 4],
	pub last_scissor: [i32; 4],
	pub last_viewport: [i32; 4],
	pub gl_to_rs_map: HashMap<u32, ACTIVE_INFO_TYPE>,
}

impl<T: glow::Context> RendererPlatform<T> {

	#[cfg(all(target_arch = "wasm32"))]
	pub fn new_webgl(title: &str) -> RendererPlatform<glow::web::Context> {
		use wasm_bindgen::JsCast;
		let canvas = web_sys::window()
			.unwrap()
			.document()
			.unwrap()
			.get_element_by_id("canvas")
			.unwrap()
			.dyn_into::<web_sys::HtmlCanvasElement>()
			.unwrap();
		#[cfg(all(target_arch = "wasm32", feature = "webgl1"))]
		let gl = {
			let webgl_context = canvas
				.get_context("webgl")
				.unwrap()
				.unwrap()
				.dyn_into::<web_sys::WebGlRenderingContext>()
				.unwrap();
			glow::web::Context::from_webgl1_context(webgl_context);
		};
		#[cfg(all(target_arch = "wasm32", not(feature = "webgl1")))]
		let gl = {
			let webgl2_context = canvas
				.get_context("webgl2")
				.unwrap()
				.unwrap()
				.dyn_into::<web_sys::WebGl2RenderingContext>()
				.unwrap();
			glow::web::Context::from_webgl2_context(webgl2_context)
		};

		let mut r = RendererPlatform {
			gl,
			clear_color: [0.0, 0.0, 0.0, 1.0],
			last_scissor: [0; 4],
			last_viewport: [0; 4],
			gl_to_rs_map: HashMap::new(),
		};
		r.initialize();
		r
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

		let mut r = RendererPlatform {
			gl: context,
			window: Some(windowed_context),
			events_loop: Some(events_loop),
			clear_color: [0.0, 0.0, 0.0, 1.0],
			last_scissor: [0; 4],
			last_viewport: [0; 4],
			gl_to_rs_map: HashMap::new(),
		};
		r.initialize();
		r
	}
	pub fn set_shader_program(&self, shader: &mut Shader<T>) {
		if !shader.ready {
			shader.link();
		}
		unsafe {
			self.gl.use_program(shader.program);
		}
	}
	pub fn set_vertex_buffer(&self, vertex_buffer: &mut VertexBuffer<T>) {
		vertex_buffer.bind(self);
	}
	pub fn set_index_buffer(&self, index_buffer: &mut IndexBuffer<T>) {
		index_buffer.bind(self);
	}
	/// 参数 0~1 数值
	pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
		self.clear_color[0] = r;
		self.clear_color[1] = g;
		self.clear_color[2] = b;
		self.clear_color[3] = a;
		unsafe { self.gl.clear_color(r, g, b, a) }
	}
	// 左下角。最初（0,0）
	pub fn set_view_port(&mut self, x: i32, y: i32, w: i32, h: i32) {
		self.last_viewport[0] = x;
		self.last_viewport[1] = y;
		self.last_viewport[2] = w;
		self.last_viewport[3] = h;
		unsafe {
			self.gl.viewport(x, y, w, h);
		}
	}
	/// 左下角。最初（0,0） 裁剪
	pub fn set_scissor(&mut self, x: i32, y: i32, w: i32, h: i32) {
		self.last_scissor[0] = x;
		self.last_scissor[1] = y;
		self.last_scissor[2] = w;
		self.last_scissor[3] = h;
		unsafe {
			self.gl.scissor(x, y, w, h);
		}
	}
	fn initialize(&mut self) {
		let [r, g, b, a] = self.clear_color;
		unsafe {
			self.gl.clear_color(r, g, b, a);
		}
		self.gl_to_rs_map.insert(glow::BOOL, ACTIVE_INFO_TYPE::BOOL);
		self.gl_to_rs_map.insert(glow::INT, ACTIVE_INFO_TYPE::INT);
		self.gl_to_rs_map.insert(glow::FLOAT, ACTIVE_INFO_TYPE::FLOAT);
		self.gl_to_rs_map.insert(glow::FLOAT_VEC2, ACTIVE_INFO_TYPE::FLOAT_VEC2);
		self.gl_to_rs_map.insert(glow::FLOAT_VEC3, ACTIVE_INFO_TYPE::FLOAT_VEC3);
		self.gl_to_rs_map.insert(glow::FLOAT_VEC4, ACTIVE_INFO_TYPE::FLOAT_VEC4);
		self.gl_to_rs_map.insert(glow::INT_VEC2, ACTIVE_INFO_TYPE::INT_VEC2);
		self.gl_to_rs_map.insert(glow::INT_VEC3, ACTIVE_INFO_TYPE::INT_VEC3);
		self.gl_to_rs_map.insert(glow::INT_VEC4, ACTIVE_INFO_TYPE::INT_VEC4);
		self.gl_to_rs_map.insert(glow::FLOAT_MAT2, ACTIVE_INFO_TYPE::FLOAT_MAT2);
		self.gl_to_rs_map.insert(glow::FLOAT_MAT3, ACTIVE_INFO_TYPE::FLOAT_MAT3);
		self.gl_to_rs_map.insert(glow::FLOAT_MAT4, ACTIVE_INFO_TYPE::FLOAT_MAT4);
		self.gl_to_rs_map.insert(glow::SAMPLER_2D, ACTIVE_INFO_TYPE::SAMPLER_2D);
		self.gl_to_rs_map.insert(glow::SAMPLER_CUBE, ACTIVE_INFO_TYPE::SAMPLER_CUBE);

		#[cfg(not(feature = "webgl1"))]
			{
				self.gl_to_rs_map.insert(glow::SAMPLER_2D_SHADOW, ACTIVE_INFO_TYPE::SAMPLER_2D_SHADOW);
				self.gl_to_rs_map.insert(glow::SAMPLER_CUBE_SHADOW, ACTIVE_INFO_TYPE::SAMPLER_CUBE_SHADOW);
				self.gl_to_rs_map.insert(glow::SAMPLER_3D, ACTIVE_INFO_TYPE::SAMPLER_3D);
			}
	}
	pub fn draw(&self, drawable: &mut Drawable<T>, shader: &mut Shader<T>) {
		unsafe {
			self.set_shader_program(shader);
			self.set_vertex_buffer(&mut drawable.vertex_buffer);
			match &mut drawable.index_buffer {
				Some(index_buffer) => {
					self.set_index_buffer(index_buffer);
				}
				None => {}
			}
			for attrbute in shader.attributes.iter() {
				let element = drawable.vertex_buffer
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
						self.gl.vertex_attrib_pointer_f32(
							u,
							element.size as i32,
							glow::FLOAT,
							element.normalize,
							element.stride as i32,
							element.offset as i32,
						);
						self.gl.enable_vertex_attrib_array(u);
					}
					_ => {
						panic!("error");
					}
				}
			}

			for uniform in shader.uniforms.iter() {
				match uniform.location_id {
					GL_Location::UniformLocation(u) => {
						match shader.get_uniform_value(&uniform.name) {
							config::UniformValueType::FLOAT_MAT3(f) => {
								self.gl.uniform_matrix_3_f32_slice(
									Some(u),
									false,
									f,
								);
							}
							config::UniformValueType::FLOAT_MAT4(f) => {
								self.gl.uniform_matrix_4_f32_slice(
									Some(u),
									false,
									f,
								);
							}
							_ => { panic!("error"); }
						}
					}
					_ => {
						panic!("error");
					}
				}
			}
		}
	}
}

