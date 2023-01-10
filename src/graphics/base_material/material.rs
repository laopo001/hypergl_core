use wgpu::Device;

use super::shader::{BaseShader, FragmentUniformInput, VertexUniformInput};
use crate::ecs::components::camera::CameraComponent;
use crate::graphics::mesh::Mesh;
use crate::graphics::texture::Texture;
use crate::{app::App, Vec3};
use crate::{Float, Vec4};
use std::collections::HashMap;

use handlebars::Handlebars;
lazy_static! {
    pub static ref TEMPLATES: Handlebars<'static> = {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("shade.wgsl.hbs", include_str!("./shade.wgsl.hbs"))
            .unwrap();
        handlebars
    };
}

#[derive(Debug)]
pub struct Material {
    pub shader: BaseShader,
    pub render_pipeline: Option<wgpu::RenderPipeline>,
}

impl Material {
    pub fn new() -> Self {
        Self {
            shader: BaseShader::new(),
            render_pipeline: None,
        }
    }
    pub fn set_color(&mut self, color: Vec3) {
        self.shader.fragment_uniform_input.color.x = color.x;
        self.shader.fragment_uniform_input.color.y = color.y;
        self.shader.fragment_uniform_input.color.z = color.z;
    }
    pub fn get_color(&mut self) -> Vec3 {
        use glam::Vec4Swizzles;
        return self.shader.fragment_uniform_input.color.xyz();
    }
    pub fn set_opacity(&mut self, opacity: Float) {
        self.shader.fragment_uniform_input.color.w = opacity;
    }
    pub fn get_opacity(&mut self) -> Float {
        return self.shader.fragment_uniform_input.color.w;
    }
    pub fn create_render_pipeline(
        &mut self,
        device: &wgpu::Device,
        mesh: &Mesh,
        texture_format: wgpu::TextureFormat,
    ) {
        let mut data = HashMap::new();
        for (_, v) in mesh.attribute_map.iter() {
            data.insert(v.attribute.name, "enable");
        }
        let source = TEMPLATES.render("shade.wgsl.hbs", &data).unwrap();
        dbg!(&source);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &VertexUniformInput::bind_group_layout(device),
                    &FragmentUniformInput::bind_group_layout(device),
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",  // 1.
                buffers: &[mesh.desc()], // 2.
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: texture_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        self.render_pipeline = Some(render_pipeline);
    }
}
