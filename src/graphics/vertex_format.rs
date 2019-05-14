use crate::config::SEMANTIC;
use crate::utils::{console_error, console_log};
pub struct VertexType {
    semantic: SEMANTIC,
    size: u8,
    normalize: bool,
}

impl VertexType {
    pub fn new(semantic: SEMANTIC,size: u8,normalize: bool) -> Self {
        VertexType {
            semantic,size,normalize
        }
    }
}

pub struct VertexAttribPointer {
    pub semantic: SEMANTIC,
    pub size: u8,
    pub normalize: bool,
    pub offset: u32,
    pub stride: u32,
    // length: u32,
}

static F32_BYTES_SIZE: u32 = 4;

pub struct VertexFormat {
    pub elements: Vec<VertexAttribPointer>,
    pub has_uv0: bool,
    pub has_uv1: bool,
    pub has_color: bool,
    pub size: u32,
}

impl VertexFormat {
    #[allow(unused_mut)]
    pub fn new(vertex_types: Vec<VertexType>) -> Self {
        let mut offset = 0_u32;
        let len = vertex_types.len();
        let mut elements = vec![];
        let mut has_uv0: bool = false;
        let mut has_uv1: bool = false;
        let mut has_color: bool = false;
        for item in vertex_types {
            let offset_temp = offset + item.size as u32 * F32_BYTES_SIZE;
            let element = VertexAttribPointer {
                offset,
                size: item.size,
                normalize: item.normalize,
                stride: 0,
                semantic: item.semantic,
            };
            offset = offset_temp;
            match &element.semantic {
                TEXCOORD0 => has_uv0 = true,
                TEXCOORD1 => has_uv1 = true,
                COLOR => has_color = true,
            }
            elements.push(element);
        }
        for i in 0..len {
            elements[i].stride = offset;
        }
        VertexFormat {
            elements,
            has_uv0,
            has_uv1,
            has_color,
            size: offset,
        }
    }
}