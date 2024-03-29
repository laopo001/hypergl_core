#[macro_use]
extern crate lazy_static;

pub mod app;
pub mod ecs;
pub mod graphics;
pub mod node;
// mod test;

pub type Float = f32;
pub type Vec3 = glam::Vec3;
pub type Vec4 = glam::Vec4;
pub type Quat = glam::Quat;
pub type Mat4 = glam::Mat4;
pub type Mat3 = glam::Mat3;

pub const PI: f32 = std::f32::consts::PI;

pub type Color = wgpu::Color;
