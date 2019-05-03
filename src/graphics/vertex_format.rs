/// 定义顶点输入
#[derive(Copy, Clone)]
enum SEMANTIC {
    POSITION,
    NORMAL,
    TANGENT,
    COLOR,
    TEXCOORD0,
    TEXCOORD1,
}

pub struct VertexType {
    semantic: SEMANTIC,
    size: u8,
    normalize: bool,
}

pub struct VertexAttribPointer {
    semantic: SEMANTIC,
    size: u8,
    normalize: bool,
    offset: u32,
    stride: u32,
    // length: u32,
}

static F32_BYTES_SIZE: u32 = 4;

pub struct VertexFormat {
    pub elements: Vec<VertexAttribPointer>,
    pub has_uv0: bool,
    pub has_uv1: bool,
    pub has_color: bool,
}

impl VertexFormat {
    #[allow(unused_mut)]
    pub fn new(vertex_types: &Vec<VertexType>) -> Self {
        let mut offset = 0_u32;
        let len = vertex_types.len();
        let mut elements = vec![];
        let mut has_uv0: bool = false;
        let mut has_uv1: bool = false;
        let mut has_color: bool = false;
        for i in 0..len {
            let item = &vertex_types[i];
            let element = VertexAttribPointer {
                semantic: item.semantic,
                offset,
                size: item.size,
                normalize: item.normalize,
                stride: 0,
            };
            elements.push(element);
            offset += item.size as u32 * F32_BYTES_SIZE;
            match item.semantic {
                TEXCOORD0 => has_uv0 = true,
                TEXCOORD1 => has_uv1 = true,
                COLOR => has_color = true,
            }
        }
        VertexFormat {
            elements,
            has_uv0,
            has_uv1,
            has_color,
        }
    }
}