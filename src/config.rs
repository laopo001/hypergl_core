type IntoString = Into<String>;

/// 定义顶点输入
#[derive(Clone)]
pub enum SEMANTIC {
    POSITION(String),
    NORMAL(String),
    TANGENT(String),
    COLOR(String),
    TEXCOORD0(String),
    TEXCOORD1(String),
}
impl SEMANTIC {
    pub fn to_string(&self) -> String {
        match self {
            SEMANTIC::POSITION(s) => s.to_string(),
            SEMANTIC::NORMAL(s) => s.to_string(),
            SEMANTIC::TANGENT(s) => s.to_string(),
            SEMANTIC::COLOR(s) => s.to_string(),
            SEMANTIC::TEXCOORD0(s) => s.to_string(),
            SEMANTIC::TEXCOORD1(s) => s.to_string(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum ACTIVE_INFO_TYPE {
    BOOL,
    INT,
    FLOAT,
    FLOAT_VEC2,
    FLOAT_VEC3,
    FLOAT_VEC4,
    INT_VEC2,
    INT_VEC3,
    INT_VEC4,
    BOOL_VEC2,
    BOOL_VEC3,
    BOOL_VEC4,
    FLOAT_MAT2,
    FLOAT_MAT3,
    FLOAT_MAT4,
    SAMPLER_2D,
    SAMPLER_CUBE,
    SAMPLER_2D_SHADOW,
    SAMPLER_CUBE_SHADOW,
    SAMPLER_3D,
    // FLOATARRAY,
}
