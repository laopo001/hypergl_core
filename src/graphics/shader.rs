
static mut SHADER_ID: usize = 0;

pub struct Shader {
    shader_id: usize,

}
impl Shader {
    fn new() -> Self {
        unsafe {
            let s = Shader {
                shader_id: SHADER_ID,
            };
            SHADER_ID += 1;
            s
        }
    }
}

#[test]
fn test_shader_id_add() {
    let _a = Shader::new();
    let b = Shader::new();
    assert_eq!(b.shader_id, 1);
}