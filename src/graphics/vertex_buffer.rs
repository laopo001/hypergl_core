
use crate::graphics::renderer::RendererPlatform;
use crate::graphics::vertex_format::VertexFormat;
use glow::Context;
// type Buffer = impl glow::Context;

pub struct VertexBuffer<T: glow::Context> {
    pub buffer_id: Option<T::Buffer>,
    format: VertexFormat,
    num_vertices: u32,
    buffer: Box<[u8]>,
    usage: u32,
}

impl<T: glow::Context> VertexBuffer<T> {
    pub fn new(
        format: VertexFormat,
        num_vertices: u32,
        usage: u32,
        data: Box<[u8]>,
    ) -> VertexBuffer<T> {
        VertexBuffer {
            buffer_id: None,
            num_vertices,
            format,
            usage,
            buffer: data,
        }
    }
    pub fn bind(&mut self, renderer: &RendererPlatform<T>) {
        match self.buffer_id.take() {
            Some(buffer_id) => unsafe {
                renderer.gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer_id));
            },
            None => unsafe {
                let t = renderer.gl.create_buffer().unwrap();
                self.buffer_id = Some(t);
                renderer.gl.bind_buffer(glow::ARRAY_BUFFER, self.buffer_id);
                renderer.gl.buffer_data_u8_slice(
                    glow::ARRAY_BUFFER,
                    self.buffer.as_ref(),
                    self.usage,
                );
            },
        }
    }
}