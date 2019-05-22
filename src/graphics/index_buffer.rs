pub struct IndexBuffer<T: glow::Context, K> {
	pub buffer_id: Option<T::Buffer>,
	pub buffer: Box<[K]>,
}

impl<T: glow::Context, K> IndexBuffer<T, K> {
	pub fn new(data: Box<[K]>, data_type: String, usage: u32) -> IndexBuffer<T, K> {
		IndexBuffer {
			buffer_id: None,
			buffer: data,
		}
	}
}