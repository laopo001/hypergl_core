
use crate::config::ACTIVE_INFO_TYPE;
use crate::graphics::vertex_format::VertexAttribPointer;
pub struct ShaderVariable<T: glow::Context> {
    pub enable: bool,
    // pub element: VertexAttribPointer,
    pub name: String,
    pub gl_type: ACTIVE_INFO_TYPE,
    pub location_id: GL_Location<T::UniformLocation>,
}

pub enum GL_Location<T> {
    UniformLocation(T),
    AttribLocation(u32),
}

impl<T: glow::Context> ShaderVariable<T> {
    pub fn new(
        name: &str,
        gl_type: ACTIVE_INFO_TYPE,
        location_id: GL_Location<T::UniformLocation>,
    ) -> Self {
        ShaderVariable {
            enable: false,
            name: name.to_string(),
            gl_type,
            location_id,
        }
    }
}