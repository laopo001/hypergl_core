use crate::graphics::renderer::RendererPlatform;
use crate::graphics::shader_variable::{GL_Location, ShaderVariable};
use crate::utils::{console_log,console_error};
static mut SHADER_ID: usize = 0;

pub struct Shader<'a, T: glow::Context> {
    pub shader_id: usize,
    pub vshader_source: String,
    pub fshader_source: String,
    pub vshader: Option<T::Shader>,
    pub fshader: Option<T::Shader>,
    pub program: Option<T::Program>,
    pub renderer: &'a RendererPlatform<T>,
    pub attributes: Vec<ShaderVariable<T>>,
    pub uniforms: Vec<ShaderVariable<T>>,
    pub samplers: Vec<ShaderVariable<T>>,
}

impl<'a, T: glow::Context> Shader<'a, T> {
    pub fn new(renderer: &'a RendererPlatform<T>, vshader: String, fshader: String) -> Self {
        unsafe {
            let mut s = Shader {
                shader_id: SHADER_ID,
                vshader_source: vshader,
                fshader_source: fshader,
                program: None,
                vshader: None,
                fshader: None,
                renderer,
                attributes: vec![],
                uniforms: vec![],
                samplers: vec![],
            };
            SHADER_ID += 1;
            s.compile();
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
                console_error(self.renderer.gl.get_program_info_log(program));
                panic!(self.renderer.gl.get_program_info_log(program));
            }
            // let num_attributes = self.renderer.gl.get_program_paramater();
            self.renderer
                .gl
                .detach_shader(program, self.vshader.unwrap());
            self.renderer.gl.delete_shader(self.vshader.unwrap());
            self.renderer
                .gl
                .detach_shader(program, self.fshader.unwrap());
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
        gl.compile_shader(shader);
        // dbg!(gl.get_shader_compile_status(shader), source, shader);
        if !gl.get_shader_compile_status(shader) {
            console_error(gl.get_shader_info_log(shader));
            panic!(gl.get_shader_info_log(shader));
        }
        return shader;
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_shader_id_add() {
    let renderer = RendererPlatform::<glow::native::Context>::new_opengl("");
    let _a = Shader::new(&renderer, "".to_string(), "".to_string());
    let b = Shader::new(&renderer, "".to_string(), "".to_string());
    assert_eq!(b.shader_id, 1);
}