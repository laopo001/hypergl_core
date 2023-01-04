use crate::graphics::model::Model;

pub struct ModelComponent {
    pub model: Model,
}

impl ModelComponent {
    pub fn new(model: Model) -> Self {
        return Self { model };
    }
}
