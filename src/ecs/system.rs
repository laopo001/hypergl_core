use std::ptr::NonNull;

use super::components::camera::CameraComponent;

#[derive(Debug)]
pub struct System {
    pub cameras: Vec<NonNull<CameraComponent>>,
    pub test: Vec<i32>,
}

impl System {
    pub fn add_camera(&mut self, c: NonNull<CameraComponent>) {
        self.cameras.push(c);
    }
    pub fn active_camera(&self) -> Option<NonNull<CameraComponent>> {
        return if self.cameras.len() == 0 {
            None
        } else {
            Some(self.cameras[0])
        };
    }
}
