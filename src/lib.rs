#[macro_use]
extern crate lazy_static;

extern crate nalgebra as na;

pub mod app;
pub mod camera;
pub mod ecs;
pub mod graphics;
pub mod node;
mod test;

pub type Float = f32;
pub type Vector3 = na::Vector3<Float>;
pub type Quaternion = na::Quaternion<Float>;
pub type Matrix4 = na::Matrix4<Float>;
pub type Transform3 = na::Transform3<Float>;
pub type Point3 = na::Point3<Float>;
pub type Isometry3 = na::Isometry3<Float>;
pub type UnitQuaternion = na::UnitQuaternion<Float>;

pub const PI: f32 = std::f32::consts::PI;

pub type Color = wgpu::Color;
