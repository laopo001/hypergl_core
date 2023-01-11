use crate::{Float, Vec3};
#[derive(Debug)]
pub struct DirectionalLight {
    pub direction: Vec3,
}
#[derive(Debug)]
pub struct PointLight {
    pub range: Float,
}
#[derive(Debug)]
pub enum LightType {
    Directional(DirectionalLight),
    Point(PointLight),
    Spot,
}

#[derive(Debug)]
pub struct LightComponent {
    pub light: LightType,
}
