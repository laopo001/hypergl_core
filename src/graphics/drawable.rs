use crate::config;
use crate::graphics::vertex_buffer::VertexBuffer;

static mut DRAW_ID: usize = 0;

pub struct Drawable<T: glow::Context> {
	mode: config::DrawMode,
	draw_id: usize,
	vertex_buffer: VertexBuffer<T>,
}

impl<T: glow::Context> Drawable<T> {
	pub fn new(vertex_buffer: VertexBuffer<T>) -> Drawable<T> {
		unsafe {
			let mut d = Drawable {
				mode: config::DrawMode::TRIANGLES,
				draw_id: DRAW_ID,
				vertex_buffer,
			};
			DRAW_ID += 1;
			d
		}
	}
}