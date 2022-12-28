use std::ops::{Deref, DerefMut};
use std::{any::Any, ptr::NonNull};

use super::components::camera::CameraComponent;
use crate::{
    node::{Node, NodeTrait},
    Float, Isometry3, Matrix4, UnitQuaternion, Vector3, PI,
};

unsafe impl Sync for Entity {}
unsafe impl Send for Entity {}
pub struct Entity {
    pub __node: Node,
    pub camera: Option<CameraComponent>,
}
impl Entity {
    pub fn new(name: &str) -> Self {
        return Entity {
            __node: Node::new(name),
            camera: None,
        };
    }
    pub fn add_camera(&mut self, mut camera: CameraComponent) {
        camera.entity = NonNull::new(self);
        self.camera = Some(camera);
    }
}
impl NodeTrait for Entity {
    fn add_child(&mut self, mut child: Box<dyn NodeTrait>) {
        // if child
        //     .as_mut()
        //     .as_any()
        //     .downcast_mut::<Entity>()
        //     .unwrap()
        //     .attached
        // {
        //     return;
        // }
        // if self.attached {
        //     child
        //         .as_mut()
        //         .as_any()
        //         .downcast_mut::<Entity>()
        //         .unwrap()
        //         .attached = true;
        // }
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
    fn sync(&mut self) {
        self.__node.sync();
    }
    fn get_local_matrix(&self) -> Matrix4 {
        self.__node.get_local_matrix()
    }
    fn parent(&mut self) -> Option<NonNull<dyn NodeTrait>> {
        self.__node.parent()
    }

    fn root(&mut self) -> NonNull<dyn NodeTrait> {
        self.__node.root()
    }
}

fn relative_eq(a: Vec<Float>, b: Vec<Float>) -> bool {
    let epsilon = 1.0e-8 as f32;
    if a.len() != b.len() {
        dbg!(&a.len(), &b.len());

        return false;
    }
    for i in 0..a.len() {
        if (a[i] - b[i]).abs() > epsilon {
            dbg!(&a);
            dbg!(&b);
            return false;
        }
    }
    return true;
}

impl Deref for Entity {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        return &self.__node;
    }
}
impl DerefMut for Entity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.__node;
    }
}

#[test]
fn test_local_position() {
    let mut node = Entity::new("root");

    node.set_local_position(1., 1., 1.);
    node.set_local_euler_angle(0.5 * PI, 0., 0.);
    node.set_local_scale(1., 2., 1.);

    let mut child = Entity::new("child");
    child.set_local_position(0., 2., 0.);
    node.add_child(Box::new(child));
    unsafe {
        let a = node.children[0]
            .as_mut()
            .as_any()
            .downcast_mut::<Entity>()
            .unwrap()
            .get_position();
        let b = Vector3::new(1., 1., 5.);
        assert!(relative_eq(
            a.data.as_slice().to_vec(),
            b.data.as_slice().to_vec()
        ))
    }
}
