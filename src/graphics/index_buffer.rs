use crate::graphics::renderer::RendererPlatform;

pub struct IndexBuffer<T: glow::Context> {
	pub buffer_id: Option<T::Buffer>,
	pub buffer: Box<[u32]>,
	pub usage: u32,
}

impl<T: glow::Context> IndexBuffer<T> {
	pub fn new(data: Box<[u32]>, data_type: String, usage: u32) -> IndexBuffer<T> {
		IndexBuffer {
			buffer_id: None,
			buffer: data,
			usage
		}
	}
	pub fn bind(&mut self, renderer: &RendererPlatform<T>){
		match self.buffer_id.take() {
			Some(buffer_id) => unsafe {
				renderer.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer_id));
			},
			None => unsafe {
				let t = renderer.gl.create_buffer().unwrap();
				self.buffer_id = Some(t);
				renderer.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.buffer_id);
				renderer.gl.buffer_data_u32_slice(
					glow::ARRAY_BUFFER,
					self.buffer.as_ref(),
					self.usage,
				);
			},
		}
	}
}