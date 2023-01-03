use std::any::Any;
#[allow(unused_imports)]
use std::cell::{RefCell, UnsafeCell};
use std::fmt::Debug;
#[allow(unused_imports)]
use std::ops::Deref;
use std::ptr::NonNull;
#[allow(unused_imports)]
use std::rc::{Rc, Weak};

use crate::app::App;
use crate::{Float, Mat4, Quat, Vec3};
pub trait NodeTrait: Sync + Send + Debug {
    fn add_child(&mut self, child: Box<dyn NodeTrait>);
    fn set_parent(&mut self, parent: Option<NonNull<dyn NodeTrait>>);
    fn as_any(&mut self) -> &mut dyn Any;
    fn to_node(&mut self) -> &mut Node;
    // fn sync(&mut self);
    fn parent(&mut self) -> Option<NonNull<dyn NodeTrait>>;
    fn children(&mut self) -> &mut Vec<Box<dyn NodeTrait>>;
    fn root(&mut self) -> NonNull<dyn NodeTrait>;
}
unsafe impl Sync for Node {}
unsafe impl Send for Node {}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub location_position: Vec3,
    pub location_rotation: Quat,
    pub local_scale: Vec3,
    pub world_transform: Mat4,
    _dirty_world: bool,
    pub parent: Option<NonNull<dyn NodeTrait>>,
    pub children: Vec<Box<dyn NodeTrait>>,
    pub enabled: bool,
    pub attached: bool,
    pub app: Option<NonNull<App>>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        return Self {
            location_position: Vec3::new(0.0, 0.0, 0.0),
            location_rotation: Quat::IDENTITY,
            local_scale: Vec3::new(1.0, 1.0, 1.0),
            // local_transform: Transform3::default(),
            world_transform: Mat4::IDENTITY,
            parent: None,
            children: vec![],
            _dirty_world: false,
            name: name.to_string(),
            enabled: true,
            attached: false,
            app: None,
        };
    }
    pub fn set_local_position(&mut self, x: Float, y: Float, z: Float) {
        self.location_position.x = x;
        self.location_position.y = y;
        self.location_position.z = z;
        self._dirty_world = false;
    }
    pub fn get_local_position(&self) -> &Vec3 {
        &self.location_position
    }
    pub fn set_local_euler_angle(&mut self, x: Float, y: Float, z: Float) {
        self.location_rotation = Quat::from_euler(glam::EulerRot::XYZ, x, y, z);
    }
    pub fn get_local_euler_angle(&self) -> (Float, Float, Float) {
        self.location_rotation.to_euler(glam::EulerRot::XYZ)
    }
    pub fn set_local_rotation(&mut self, q: Quat) {
        self.location_rotation = q;
        self._dirty_world = false;
    }
    pub fn get_local_rotation(&self) -> &Quat {
        return &self.location_rotation;
    }
    pub fn set_local_scale(&mut self, x: Float, y: Float, z: Float) {
        self.local_scale.x = x;
        self.local_scale.y = y;
        self.local_scale.z = z;
    }
    pub fn get_local_scale(&self) -> &Vec3 {
        &self.local_scale
    }
    pub fn look_at_lh(&mut self, target: Vec3, up: Vec3) {
        let p = self.get_position();

        let mut m = Mat4::look_at_lh(Vec3::new(p.x, p.y, p.z), target, up);
        m = m.inverse();
        self.set_rotation(Quat::from_mat4(&m));
        self._dirty_world = false;
    }
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let p = self.get_position();

        let mut m = Mat4::look_at_rh(Vec3::new(p.x, p.y, p.z), target, up);
        m = m.inverse();
        // self.location_rotation = Quat::from_mat4(&m);
        self.set_rotation(Quat::from_mat4(&m));
        self._dirty_world = false;
    }
    pub fn get_position(&mut self) -> Vec3 {
        if !self._dirty_world {
            self.get_world_matrix();
        }
        let (scale, rotation, translation) = self.world_transform.to_scale_rotation_translation();
        return translation;
    }
    pub fn set_position(&mut self, x: Float, y: Float, z: Float) {
        todo!();
    }
    pub fn set_rotation(&mut self, q: Quat) {
        if self.parent.is_none() {
            self.location_rotation = q;
        } else {
            unsafe {
                let mut pr = self
                    .parent
                    .unwrap()
                    .as_mut()
                    .as_any()
                    .downcast_mut::<Node>()
                    .unwrap()
                    .get_rotation();
                pr = pr.inverse();
                self.location_rotation = pr * q;
            }
        }
    }
    pub fn get_rotation(&mut self) -> Quat {
        self.get_world_matrix();
        let (scale, rotation, translation) = self.world_transform.to_scale_rotation_translation();
        return rotation;
    }
    pub fn get_world_matrix(&mut self) -> Mat4 {
        if self._dirty_world {
            return self.world_transform;
        }
        unsafe {
            self.root().as_mut().to_node().sync();
            return self.world_transform;
        }
    }
    pub fn get_local_matrix(&self) -> Mat4 {
        return Mat4::from_scale_rotation_translation(
            self.local_scale,
            self.location_rotation,
            self.location_position,
        );
    }
    pub fn sync(&mut self) {
        unsafe {
            if self.parent.is_some() {
                unsafe {
                    let p = self.parent.unwrap().as_mut();
                    self.world_transform = p.to_node().get_local_matrix() * self.get_local_matrix();
                    self._dirty_world = true;
                }
            } else {
                self.world_transform = self.get_local_matrix();
                self._dirty_world = true;
            }
            for child in self.children.iter_mut() {
                child.as_mut().to_node().sync();
            }
        }
    }
}

impl NodeTrait for Node {
    fn add_child(&mut self, mut child: Box<dyn NodeTrait>) {
        child.set_parent(NonNull::new(self));
        self.children.push(child);
    }
    fn set_parent(&mut self, parent: Option<NonNull<dyn NodeTrait>>) {
        self.parent = parent;
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
    fn to_node(&mut self) -> &mut Node {
        return self;
    }
    fn parent(&mut self) -> Option<NonNull<dyn NodeTrait>> {
        return self.parent;
    }
    fn children(&mut self) -> &mut Vec<Box<dyn NodeTrait>> {
        return &mut self.children;
    }
    fn root(&mut self) -> NonNull<dyn NodeTrait> {
        unsafe {
            let mut curr = self.parent();

            loop {
                if curr.is_some() {
                    curr = curr.unwrap().as_mut().parent();
                } else {
                    return NonNull::new_unchecked(self);
                }
            }
        }
    }
}

pub fn relative_eq(a: Vec<Float>, b: Vec<Float>) -> bool {
    let epsilon = 1.0e-5 as f32;
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

#[test]
fn test_local_position() {
    let mut node = Node::new("root");

    node.set_local_position(1., 1., 1.);
    use crate::PI;
    node.set_local_euler_angle(0.5 * PI, 0., 0.);
    node.set_local_scale(1., 2., 1.);

    let mut child = Node::new("child");
    child.set_local_position(0., 2., 0.);
    node.add_child(Box::new(child));
    unsafe {
        let a = node.children[0]
            .as_mut()
            .as_any()
            .downcast_mut::<Node>()
            .unwrap()
            .get_position();

        let b = Vec3::new(1., 1., 5.);

        assert!(relative_eq(a.to_array().to_vec(), b.to_array().to_vec(),))
    }
}
