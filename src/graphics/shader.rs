
static mut shader_id: usize = 0;

pub struct Shader {
    shader_id: usize,

}
impl Shader {
    fn new() -> Self {
        unsafe {
            let s = Shader {
                shader_id: shader_id,
            };
            shader_id += 1;
            return s;
        }
    }
}

#[test]
fn test_shader_id_add() {
    let _a = Shader::new();
    let b = Shader::new();
    assert_eq!(b.shader_id, 1);
}