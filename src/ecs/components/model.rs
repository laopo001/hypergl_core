use crate::ecs::entity::Entity;
use crate::graphics::model::Model;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct ModelComponent {
    pub model: Model,
    pub entity: Option<NonNull<Entity>>,
}

impl ModelComponent {
    pub fn new(model: Model) -> Self {
        return Self {
            model,
            entity: None,
        };
    }
}
