use std::ptr::NonNull;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    event::*,
    event_loop::{self, ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{
    ecs::{
        entity::{self, Entity},
        system::System,
    },
    graphics::{
        base_material::material::Material,
        mesh::Mesh,
        model::{DrawModel, Model},
        texture::Texture,
    },
    Mat4, Vec3,
};

pub struct App {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub width: u32,
    pub height: u32,
    pub config: wgpu::SurfaceConfiguration,
    pub window: Window,
    pub root: Box<Entity>,
    pub system: System,
}

impl App {
    fn create_window(event_loop: &EventLoop<()>) -> Window {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }
        let window = WindowBuilder::new().build(event_loop).unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(450, 400));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }
        return window;
    }
    pub async fn new(event_loop: &EventLoop<()>, width: u32, height: u32) -> Self {
        let window = App::create_window(event_loop);

        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let entity = Entity::new("root");
        let mut app = App {
            width,
            height,
            surface,
            device,
            queue,
            config,
            window,
            root: entity,
            system: System {
                cameras: Vec::new(),
                models: Vec::new(),
            },
        };

        // println!("{:p}", &app);
        return app;
    }
    pub fn init(&mut self) {
        self.root.__node.app = NonNull::new(self);
    }
    pub async fn start(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    if !self.input(event) {
                        // UPDATED!
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                // state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_size is &&mut so w have to dereference it twice
                                // state.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    // self.update();
                    match self.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            // state.resize(state.size)
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    }
                }
                Event::RedrawEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
    }
    pub fn render(&mut self) -> std::result::Result<(), wgpu::SurfaceError> {
        let depth_texture = Texture::create_depth_texture(self, &self.config, "depth_texture");
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            let mut camera_view_proj = Mat4::IDENTITY;
            let mut camera = self.system.active_camera();
            unsafe {
                if camera.is_some() {
                    // camera.unwrap().as_mut().bind_group(&self.device);
                    camera_view_proj = camera.unwrap().as_mut().build_view_projection_matrix();
                }
            }
            for model_nonnull in self.system.models.iter_mut() {
                unsafe {
                    let model_component = model_nonnull.as_mut();
                    let model = &mut model_component.model;

                    for index in 0..model.meshes.len() {
                        let mesh = &model.meshes[index];
                        let material = &model.materials[mesh.material_index.unwrap()];

                        let material_ptr = material as *const Material as *mut Material;

                        (*material_ptr).shader.vertex_uniform_input.camera_view_proj =
                            camera_view_proj;
                        (*material_ptr).shader.vertex_uniform_input.model_matrix =
                            model_component.entity.unwrap().as_mut().get_world_matrix();

                        (*material_ptr).shader.bind_group(&self.device);

                        render_pass.set_pipeline(&material.render_pipeline);
                        if let Some(camera) = camera {
                            render_pass.draw_mesh_new(mesh, material);
                        } else {
                            panic!("No camera");
                        }
                    }
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn input(&self, event: &WindowEvent) -> bool {
        false
    }
}
