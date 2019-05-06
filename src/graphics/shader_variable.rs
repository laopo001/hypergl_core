
use crate::config::ACTIVE_INFO_TYPE;
use crate::graphics::vertex_format::VertexAttribPointer;
pub struct ShaderVariable<T: glow::Context> {
    pub enable: bool,
    // pub element: VertexAttribPointer,
    pub name: String,
    pub uniform_type: ACTIVE_INFO_TYPE,
    pub location_id: T::UniformLocation,
}

impl<T: glow::Context> ShaderVariable<T> {
    pub fn new(
        name: &str,
        uniform_type: ACTIVE_INFO_TYPE,
        location_id: T::UniformLocation,
    ) -> Self {
        ShaderVariable {
            enable: false,
            name: name.to_string(),
            uniform_type,
            location_id,
        }
    }
}