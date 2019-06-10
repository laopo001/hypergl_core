use hypergl_core::application::Application;
use hypergl_core::config;
use hypergl_core::graphics::vertex_buffer::VertexBuffer;
use hypergl_core::graphics::shader::Shader;
use hypergl_core::graphics::vertex_format::{VertexFormat, VertexType};
use hypergl_core::graphics::shader_variable::GL_Location;
use hypergl_core::utils::{console};
use hypergl_core::graphics::drawable::Drawable;
use glow::{Context, RenderLoop};
use wasm_math::mat4::Mat4;
use image::GenericImageView;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
	main();
}

fn main() {
	unsafe {
		#[cfg(not(target_arch = "wasm32"))]
		type Context = glow::native::Context;
		#[cfg(not(target_arch = "wasm32"))]
			let mut app = Application::<glow::native::Context>::new_opengl("123");

		#[cfg(target_arch = "wasm32")]
		type Context = glow::web::Context;
		#[cfg(target_arch = "wasm32")]
			let mut app = Application::<glow::web::Context>::new_webgl("123");

		let shader_version = "#version 300 es";

		let vertex_shader_source = include_str!("./main2.vert");
		let fragment_shader_source = include_str!("./main2.frag");

		#[cfg(not(feature = "webgl1"))]
			let mut shader = Shader::new(
			&app.renderer,
			format!("{}\n{}", shader_version, vertex_shader_source),
			format!("{}\n{}", shader_version, fragment_shader_source),
		);
		#[cfg(all(feature = "webgl1"))]
			let mut shader = Shader::new(
			&app.renderer,
			vertex_shader_source.to_string(),
			fragment_shader_source.to_string(),
		);

		shader.set_uniform_value("matrix".to_string(), config::UniformValueType::FLOAT_MAT4([
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0,
		]));

		console::log(Mat4::default().data());
//		let vertex_array = app.renderer.gl.create_vertex_array().expect("Cannot create vertex array");
//		app.renderer.gl.bind_vertex_array(Some(vertex_array));

		let vertexs: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

		let arr: Vec<VertexType> = vec![VertexType::new(
			config::SEMANTIC::POSITION("position".to_string()),
			3,
			false,
		)];
		let format = VertexFormat::new(arr);
		let mut vertexbuffer = VertexBuffer::<Context>::new(
			format,
			3,
			glow::STATIC_DRAW,
			Box::new(vertexs),
		);

		let mut drawable = Drawable::new(vertexbuffer, None);
		app.renderer.draw(&mut drawable, &mut shader);

		let img = image::open("test.jpg").unwrap();

		println!("dimensions {:?}", img.dimensions());
		println!("{:?}", img.raw_pixels());
		img.save("test.png").unwrap();

		app.start();

	}
}