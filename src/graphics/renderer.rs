use crate::graphics::shader::Shader;
use crate::graphics::vertex_buffer::VertexBuffer;
use crate::utils::console_log;
use crate::config::ACTIVE_INFO_TYPE;
use std::collections::HashMap;


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct RendererPlatform<T: glow::Context> {
    pub gl: T,
    #[cfg(not(target_arch = "wasm32"))]
    pub window: Option<glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::Window>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub events_loop: Option<glutin::EventsLoop>,
    #[cfg(not(target_arch = "wasm32"))]
    pub render_loop: glow::native::RenderLoop,
    #[cfg(target_arch = "wasm32")]
    pub render_loop: glow::web::RenderLoop,
    pub clear_color: [f32; 4],
    pub last_scissor: [i32; 4],
    pub last_viewport: [i32; 4],
    pub gl_to_rs_map:HashMap<u32,ACTIVE_INFO_TYPE>,
}

impl<T: glow::Context> RendererPlatform<T> {
    #[cfg(all(target_arch = "wasm32", feature = "webgl1"))]
    pub fn new_webgl1(title: &str) -> RendererPlatform<glow::web::Context> {
        use wasm_bindgen::JsCast;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let webgl_context = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGlRenderingContext>()
            .unwrap();
        let render_loop = glow::web::RenderLoop::from_request_animation_frame();
        let gl = glow::web::Context::from_webgl1_context(webgl_context);

        let mut r = RendererPlatform {
            gl,
            clear_color: [0.0, 0.0, 0.0, 1.0],
            last_scissor: [0; 4],
            last_viewport: [0; 4],
            render_loop,
            gl_to_rs_map:HashMap::new(),
        };
        r.initialize();
        r
    }
    #[cfg(all(target_arch = "wasm32", not(feature = "webgl1")))]
    pub fn new_webgl2(title: &str) -> RendererPlatform<glow::web::Context> {
        use wasm_bindgen::JsCast;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let webgl2_context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();
        let render_loop = glow::web::RenderLoop::from_request_animation_frame();
        let gl = glow::web::Context::from_webgl2_context(webgl2_context);

        let mut r = RendererPlatform {
            gl,
            clear_color: [0.0, 0.0, 0.0, 1.0],
            last_scissor: [0; 4],
            last_viewport: [0; 4],
            render_loop,
            gl_to_rs_map:HashMap::new(),
        };
        r.initialize();
        r
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_opengl(title: &str) -> RendererPlatform<glow::native::Context> {
        let mut events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new().with_title(title);

        let context_builder = glutin::ContextBuilder::new();

        let window = context_builder
            .build_windowed(window_builder, &events_loop)
            .unwrap();

        let windowed_context = unsafe { window.make_current().unwrap() };

        let context = glow::native::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });

        let render_loop = glow::native::RenderLoop::from_window();

        let mut r = RendererPlatform {
            gl: context,
            window: Some(windowed_context),
            events_loop: Some(events_loop),
            clear_color: [0.0, 0.0, 0.0, 1.0],
            last_scissor: [0; 4],
            last_viewport: [0; 4],
            render_loop,
            gl_to_rs_map:HashMap::new(),
        };
        r.initialize();
        r
    }
    pub fn set_shader_program(&self, shader: &mut Shader<T>) {
        if !shader.ready {
            shader.link();
        }
        unsafe {
            self.gl.use_program(shader.program);
        }
    }
    pub fn set_vertex_buffer(&self, vertex_buffer: &mut VertexBuffer<T>) {
        vertex_buffer.bind(self);
    }
    /// 参数 0~1 数值
    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.clear_color[0] = r;
        self.clear_color[1] = g;
        self.clear_color[2] = b;
        self.clear_color[3] = a;
        unsafe { self.gl.clear_color(r, g, b, a) }
    }
    // 左下角。最初（0,0）
    pub fn set_view_port(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.last_viewport[0] = x;
        self.last_viewport[1] = y;
        self.last_viewport[2] = w;
        self.last_viewport[3] = h;
        unsafe {
            self.gl.viewport(x, y, w, h);
        }
    }
    /// 左下角。最初（0,0） 裁剪
    pub fn set_scissor(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.last_scissor[0] = x;
        self.last_scissor[1] = y;
        self.last_scissor[2] = w;
        self.last_scissor[3] = h;
        unsafe {
            self.gl.scissor(x, y, w, h);
        }
    }
    fn initialize(&mut self) {
        let [r, g, b, a] = self.clear_color;
        unsafe {
            self.gl.clear_color(r, g, b, a);
        }
        self.gl_to_rs_map.insert(glow::BOOL, ACTIVE_INFO_TYPE::BOOL);
        self.gl_to_rs_map.insert(glow::INT, ACTIVE_INFO_TYPE::INT);
        self.gl_to_rs_map.insert(glow::FLOAT, ACTIVE_INFO_TYPE::FLOAT);
        self.gl_to_rs_map.insert(glow::FLOAT_VEC2, ACTIVE_INFO_TYPE::FLOAT_VEC2);
        self.gl_to_rs_map.insert(glow::FLOAT_VEC3, ACTIVE_INFO_TYPE::FLOAT_VEC3);
        self.gl_to_rs_map.insert(glow::FLOAT_VEC4, ACTIVE_INFO_TYPE::FLOAT_VEC4);
        self.gl_to_rs_map.insert(glow::INT_VEC2, ACTIVE_INFO_TYPE::INT_VEC2);
        self.gl_to_rs_map.insert(glow::INT_VEC3, ACTIVE_INFO_TYPE::INT_VEC3);
        self.gl_to_rs_map.insert(glow::INT_VEC4, ACTIVE_INFO_TYPE::INT_VEC4);
        self.gl_to_rs_map.insert(glow::FLOAT_MAT2, ACTIVE_INFO_TYPE::FLOAT_MAT2);
        self.gl_to_rs_map.insert(glow::FLOAT_MAT3, ACTIVE_INFO_TYPE::FLOAT_MAT3);
        self.gl_to_rs_map.insert(glow::FLOAT_MAT4, ACTIVE_INFO_TYPE::FLOAT_MAT4);
        self.gl_to_rs_map.insert(glow::SAMPLER_2D, ACTIVE_INFO_TYPE::SAMPLER_2D);
        self.gl_to_rs_map.insert(glow::SAMPLER_CUBE, ACTIVE_INFO_TYPE::SAMPLER_CUBE);

        #[cfg(not(feature = "webgl1"))]
        {
            self.gl_to_rs_map.insert(glow::SAMPLER_2D_SHADOW, ACTIVE_INFO_TYPE::SAMPLER_2D_SHADOW);
            self.gl_to_rs_map.insert(glow::SAMPLER_CUBE_SHADOW, ACTIVE_INFO_TYPE::SAMPLER_CUBE_SHADOW);
            self.gl_to_rs_map.insert(glow::SAMPLER_3D, ACTIVE_INFO_TYPE::SAMPLER_3D);
        }
    }
    pub fn draw() {}
}

