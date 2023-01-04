use std::ops::{Deref, DerefMut};
use std::{any::Any, ptr::NonNull};

use super::components::camera::CameraComponent;
use crate::app::App;
use crate::node::relative_eq;
use crate::{
    node::{Node, NodeTrait},
    Float, Mat4, Vec3, PI,
};

unsafe impl Sync for Entity {}
unsafe impl Send for Entity {}
#[derive(Debug)]
pub struct Entity {
    pub __node: Box<Node>,
    pub camera: Option<CameraComponent>,
}
impl Entity {
    pub fn new(name: &str) -> Box<Self> {
        return Box::new(Entity {
            __node: Node::new(name),
            camera: None,
        });
    }
    pub fn add_camera(&mut self, mut camera: CameraComponent) {
        camera.entity = NonNull::new(self);
        self.camera = Some(camera);
    }
    pub fn get_app(&mut self) -> Option<NonNull<App>> {
        unsafe {
            // dbg!(&self.root().as_mut().as_any().downcast_mut::<Node>());
            return self
                .root()
                .as_mut()
                .as_any()
                .downcast_mut::<Entity>()
                .unwrap()
                .__node
                .app;
        }
    }
}

unsafe fn run(e: &mut Entity, app: Option<NonNull<App>>) {
    e.__node.attached = true;
    if app.is_some() && e.camera.is_some() {
        // dbg!(&app.unwrap());
        app.unwrap()
            .as_mut()
            .system
            .add_camera(NonNull::new_unchecked(e.camera.as_mut().unwrap()))
    }
    e.children.iter_mut().for_each(|c| {
        let ptr = c.as_mut().as_any().downcast_mut::<Entity>().unwrap();
        run(ptr, app);
    });
}

impl NodeTrait for Entity {
    fn add_child(&mut self, mut child: Box<dyn NodeTrait>) {
        unsafe {
            run(
                child.as_mut().as_any().downcast_mut::<Entity>().unwrap(),
                self.get_app(),
            );

            let ptr = &mut child as *mut Box<dyn NodeTrait>;
            self.__node.add_child(child);

            (*ptr).set_parent(NonNull::new(self));
        }
    }
    fn set_parent(&mut self, parent: Option<NonNull<dyn NodeTrait>>) {
        self.__node.parent = parent;
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
    fn to_node(&mut self) -> &mut Node {
        &mut self.__node
    }
    fn children(&mut self) -> &mut Vec<Box<dyn NodeTrait>> {
        return self.__node.children();
    }
    fn parent(&mut self) -> Option<NonNull<dyn NodeTrait>> {
        self.__node.parent()
    }
    fn root(&mut self) -> NonNull<dyn NodeTrait> {
        unsafe {
            let mut root: *mut dyn NodeTrait = self as *mut dyn NodeTrait;
            let mut curr = self.parent();

            loop {
                if curr.is_some() {
                    // dbg!(&curr.as_mut().unwrap().as_mut().to_node().name);
                    // root = curr.clone();
                    root = curr.unwrap().as_mut() as *mut dyn NodeTrait;
                    curr = (*root).parent();
                } else {
                    return NonNull::new_unchecked(root);
                }
            }
        }
    }
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
    node.add_child((child));
    unsafe {
        // dbg!(&node.root().as_mut().as_any().downcast_mut::<Entity>());

        let a = node.children[0]
            .as_mut()
            .as_any()
            .downcast_mut::<Entity>()
            .unwrap()
            .get_position();
        let b = Vec3::new(1., 1., 5.);
        assert!(relative_eq(a.to_array().to_vec(), b.to_array().to_vec()))
    }
}

#[test]
fn test_root() {
    let mut node = Entity::new("root");
    let mut child = Entity::new("child");
    let mut childchild = Entity::new("childchild");

    child.add_child((childchild));
    node.add_child((child));

    unsafe {
        dbg!(
            &node.children()[0].children()[0]
                .as_mut()
                .as_any()
                .downcast_mut::<Entity>()
                .unwrap()
                .name
        );
        assert!(
            node.children()[0].children()[0]
                .parent()
                .unwrap()
                .as_mut()
                .parent()
                .unwrap()
                .as_mut()
                .as_any()
                .downcast_mut::<Entity>()
                .unwrap()
                .to_node()
                .name
                == "root"
        );
        assert!(
            node.children()[0].children()[0]
                .as_mut()
                .as_any()
                .downcast_mut::<Entity>()
                .unwrap()
                .root()
                .as_mut()
                .as_any()
                .downcast_mut::<Entity>()
                .unwrap()
                .to_node()
                .name
                == "root"
        );
    }
}
