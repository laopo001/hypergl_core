use std::{any::Any, ptr::NonNull};

use crate::{
    node::{Node, NodeTrait},
    Float, Isometry3, Matrix4, UnitQuaternion, Vector3,
};

pub struct Entity {
    pub __node: Node,
}
impl Entity {
    pub fn new(name: &str) -> Self {
        return Entity {
            __node: Node::new(name),
        };
    }
}
impl NodeTrait for Entity {
    fn add_child(&mut self, mut child: Box<dyn NodeTrait>) {
        let ptr = &mut child as *mut Box<dyn NodeTrait>;
        self.__node.add_child(child);
        unsafe { (*ptr).set_parent(NonNull::new(self)) }
    }
    fn set_parent(&mut self, parent: Option<NonNull<dyn NodeTrait>>) {
        self.__node.parent = parent;
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
