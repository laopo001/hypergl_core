use crate::graphics::renderer::RendererPlatform;

static mut SHADER_ID: usize = 0;

pub struct Shader<'a, T: glow::Context> {
    pub shader_id: usize,
    pub vshader_source: String,
    pub fshader_source: String,
    pub vshader: Option<T::Shader>,
    pub fshader: Option<T::Shader>,
    pub program: Option<T::Program>,
    pub renderer: &'a RendererPlatform<T>,

}
impl<'a, T: glow::Context> Shader<'a, T> {
    pub fn new(renderer: &'a RendererPlatform<T>, vshader: String, fshader: String) -> Self {
        unsafe {
            let s = Shader {
                shader_id: SHADER_ID,
                vshader_source: vshader,
                fshader_source: fshader,
                program: None,
                vshader: None,
                fshader: None,
                renderer,
            };
            SHADER_ID += 1;
            s
        }
    }
    pub fn compile(&mut self) {
        let vshader = load_shader(
            &self.renderer.gl,
            glow::VERTEX_SHADER,
            self.vshader_source.as_str(),
        );
        let fshader = load_shader(
            &self.renderer.gl,
            glow::FRAGMENT_SHADER,
            self.fshader_source.as_str(),
        );
        self.program = Some(create_program(&self.renderer.gl, vshader, fshader));
        self.vshader = Some(vshader);
        self.fshader = Some(fshader);
    }
    pub fn link(&mut self) {
        unsafe {
            let program = self.program.expect("必须先执行compile");
            self.renderer.gl.link_program(program);
            if !self.renderer.gl.get_program_link_status(program) {
                panic!(self.renderer.gl.get_program_info_log(program));
            }
            self.renderer
                .gl
                .detach_shader(program, self.vshader.unwrap());
            self.renderer.gl.delete_shader(self.fshader.unwrap());
        }
    }
}

pub fn create_program<T>(gl: &T, vertex_shader: T::Shader, fragment_shader: T::Shader) -> T::Program
where
    T: glow::Context,
{
    unsafe {
        let program = gl.create_program().expect("cannot create shader");
        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        return program;
    }
}

pub fn load_shader<T>(gl: &T, shader_type: u32, source: &str) -> T::Shader
where
    T: glow::Context,
{
    unsafe {
        let shader = gl.create_shader(shader_type).expect("cannot create shader");
        gl.shader_source(shader, source);
        if !gl.get_shader_compile_status(shader) {
            panic!(gl.get_shader_info_log(shader));
        }
        return shader;
    }
}

#[test]
fn test_shader_id_add() {
    let renderer = RendererPlatform::<glow::native::Context>::new_opengl("");
    let _a = Shader::<glow::native::Context>::new(&renderer, "".to_string(), "".to_string());
    let b = Shader::<glow::native::Context>::new(&renderer, "".to_string(), "".to_string());
    assert_eq!(b.shader_id, 1);
}