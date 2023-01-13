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
}

#[derive(Debug)]
pub struct LightComponent {
    pub light: LightType,
}

impl LightComponent {
    pub fn new_directional(direction: Vec3) -> Self {
        return Self {
            light: LightType::Directional(DirectionalLight { direction }),
        };
    }
    pub fn new_point(range: Float) -> Self {
        return Self {
            light: LightType::Point(PointLight { range }),
        };
    }
}
