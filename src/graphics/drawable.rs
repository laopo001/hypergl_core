use crate::config;
use crate::graphics::vertex_buffer::VertexBuffer;
use crate::graphics::index_buffer::IndexBuffer;

static mut DRAW_ID: usize = 0;

pub struct Drawable<T: glow::Context> {
	pub	mode: config::DrawMode,
	pub	draw_id: usize,
    pub	vertex_buffer: VertexBuffer<T>,
	pub	index_buffer: Option<IndexBuffer<T>>,
}

impl<T: glow::Context> Drawable<T> {
	pub fn new(vertex_buffer: VertexBuffer<T>, index_buffer: Option<IndexBuffer<T>>) -> Drawable<T> {
		unsafe {
			let mut d = Drawable {
				mode: config::DrawMode::TRIANGLES,
				draw_id: DRAW_ID,
				vertex_buffer,
				index_buffer
			};
			DRAW_ID += 1;
			d
		}
	}
}