use std::any::Any;
#[allow(unused_imports)]
use std::cell::{RefCell, UnsafeCell};
#[allow(unused_imports)]
use std::ops::Deref;
use std::ptr::NonNull;
#[allow(unused_imports)]
use std::rc::{Rc, Weak};

use crate::{Float, Isometry3, Matrix4, UnitQuaternion, Vector3, PI};
pub trait NodeTrait: Sync + Send {
    fn add_child(&mut self, child: Box<dyn NodeTrait>);
    fn set_parent(&mut self, parent: Option<NonNull<dyn NodeTrait>>);
    fn as_any(&mut self) -> &mut dyn Any;
}
unsafe impl Sync for Node {}
unsafe impl Send for Node {}

pub struct Node {
    pub name: String,
    pub location_iso: Isometry3,
    pub local_scale: Vector3,
    // pub local_transform: Transform3,
    pub world_transform: Matrix4,
    // pub parent: *mut Node,
    // pub children: Vec<Node>,
    _dirty_world: bool,
    pub parent: Option<NonNull<dyn NodeTrait>>,
    pub children: Vec<Box<dyn NodeTrait>>,
    // enabled: bool,
}

impl Node {
    pub fn new(name: &str) -> Self {
        return Self {
            location_iso: Isometry3::identity(),
            local_scale: Vector3::new(1.0, 1.0, 1.0),
            // local_transform: Transform3::default(),
            world_transform: Matrix4::identity(),
            parent: None,
            children: vec![],
            _dirty_world: false,
            name: name.to_string(),
            // enabled: true,
        };
    }
    fn set_local_position(&mut self, x: Float, y: Float, z: Float) {
        self.location_iso.translation.vector.x = x;
        self.location_iso.translation.vector.y = y;
        self.location_iso.translation.vector.z = z;
    }
    fn get_local_position(&self) -> &Vector3 {
        &self.location_iso.translation.vector
    }
    // The primitive rotations are applied in order: 1 roll − 2 pitch − 3 yaw.
    fn set_local_euler_angle(&mut self, x: Float, y: Float, z: Float) {
        self.location_iso.rotation = UnitQuaternion::from_euler_angles(x, y, z);
    }
    fn get_local_euler_angle(&self) -> (Float, Float, Float) {
        self.location_iso.rotation.euler_angles()
    }
    fn set_local_scale(&mut self, x: Float, y: Float, z: Float) {
        self.local_scale.x = x;
        self.local_scale.y = y;
        self.local_scale.z = z;
    }
    fn get_local_scale(&self) -> &Vector3 {
        &self.local_scale
    }
    fn get_position(&mut self) -> Vector3 {
        if !self._dirty_world {
            self.get_world_matrix();
        }
        let data = self.world_transform.data.as_slice();
        return Vector3::new(data[12], data[13], data[14]);
    }
    fn set_position(&mut self, x: Float, y: Float, z: Float) {
        todo!();
    }
    fn get_local_matrix(&self) -> Matrix4 {
        self.location_iso
            .to_homogeneous()
            .prepend_nonuniform_scaling(&self.local_scale)
    }
    fn get_world_matrix(&mut self) -> Matrix4 {
        if self._dirty_world {
            return self.world_transform;
        }
        self.root().sync();
        return self.world_transform;
    }
    fn sync(&mut self) {
        unsafe {
            if self.parent.is_some() {
                unsafe {
                    let p = self
                        .parent
                        .unwrap()
                        .as_mut()
                        .as_any()
                        .downcast_mut::<Node>()
                        .unwrap();
                    self.world_transform = p.get_local_matrix() * self.get_local_matrix();
                    self._dirty_world = true;
                }
            } else {
                self.world_transform = self.get_local_matrix();
                self._dirty_world = true;
            }
            for child in self.children.iter_mut() {
                child
                    .as_mut()
                    .as_any()
                    .downcast_mut::<Node>()
                    .unwrap()
                    .sync();
            }
        }
    }
    fn parent(&mut self) -> Option<&mut Self> {
        unsafe {
            if self.parent.is_some() {
                return Some(
                    self.parent
                        .unwrap()
                        .as_mut()
                        .as_any()
                        .downcast_mut::<Node>()
                        // .downcast_ref::<Node>()
                        .unwrap(),
                );
            } else {
                None
            }
        }
    }
    fn root(&mut self) -> &mut Self {
        unsafe {
            let mut curr = self.parent();
            loop {
                if curr.is_some() {
                    curr = curr.unwrap().parent();
                } else {
                    return &mut *self;
                }
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

#[test]
fn test_local_position() {
    let mut node = Node::new("root");

    node.set_local_position(1., 1., 1.);
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
        let b = Vector3::new(1., 1., 5.);
        assert!(relative_eq(
            a.data.as_slice().to_vec(),
            b.data.as_slice().to_vec()
        ))
    }
}
