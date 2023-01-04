use std::ptr::NonNull;

use super::components::{camera::CameraComponent, model::ModelComponent};

#[derive(Debug)]
pub struct System {
    pub cameras: Vec<NonNull<CameraComponent>>,
    pub models: Vec<NonNull<ModelComponent>>,
}

impl System {
    // pub fn add_camera(&mut self, c: NonNull<CameraComponent>) {
    //     self.cameras.push(c);
    // }
    // pub fn add_model(&mut self, c: NonNull<ModelComponent>) {
    //     self.models.push(c);
    // }
    pub fn active_camera(&self) -> Option<NonNull<CameraComponent>> {
        return if self.cameras.len() == 0 {
            None
        } else {
            Some(self.cameras[0])
        };
    }
}
