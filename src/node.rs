pub type Float = f32;
pub type Vector3 = na::Vector3<Float>;
pub type Quaternion = na::Quaternion<Float>;
pub type Matrix4 = na::Matrix4<Float>;
pub type Transform3 = na::Transform3<Float>;
pub type Point3 = na::Point3<Float>;
pub type Isometry3 = na::Isometry3<Float>;
pub type UnitQuaternion = na::UnitQuaternion<Float>;

#[allow(unused_imports)]
use std::cell::{RefCell, UnsafeCell};
#[allow(unused_imports)]
use std::ops::Deref;
#[allow(unused_imports)]
use std::rc::{Rc, Weak};

use na::ComplexField;

#[derive(Debug)]
pub struct Node {
    pub location_iso: Isometry3,
    pub local_scale: Vector3,
    // pub local_transform: Transform3,
    pub world_transform: Matrix4,
    pub parent: *mut Node,
    pub children: Vec<Node>,
    _dirty_world: bool,
    // enabled: bool,
}

impl Node {
    pub fn new() -> Node {
        return Node {
            location_iso: Isometry3::identity(),
            local_scale: Vector3::new(1.0, 1.0, 1.0),
            // local_transform: Transform3::default(),
            world_transform: Matrix4::identity(),
            parent: std::ptr::null_mut(),
            children: vec![],
            _dirty_world: false,
            // enabled: true,
        };
    }

    pub fn add_child(&mut self, mut child: Node) {
        child.parent = self;
        self.children.push(child);
    }
    pub fn set_local_position(&mut self, x: Float, y: Float, z: Float) {
        self.location_iso.translation.vector.x = x;
        self.location_iso.translation.vector.y = y;
        self.location_iso.translation.vector.z = z;
    }
    pub fn get_local_position(&self) -> &Vector3 {
        &self.location_iso.translation.vector
    }
    // The primitive rotations are applied in order: 1 roll − 2 pitch − 3 yaw.
    pub fn set_local_euler_angle(&mut self, x: Float, y: Float, z: Float) {
        self.location_iso.rotation = UnitQuaternion::from_euler_angles(x, y, z);
    }
    pub fn get_local_euler_angle(&self) -> (Float, Float, Float) {
        self.location_iso.rotation.euler_angles()
    }
    pub fn set_local_scale(&mut self, x: Float, y: Float, z: Float) {
        self.local_scale.x = x;
        self.local_scale.y = y;
        self.local_scale.z = z;
    }
    pub fn get_local_scale(&self) -> &Vector3 {
        &self.local_scale
    }
    pub fn get_position(&mut self) -> Vector3 {
        if !self._dirty_world {
            self.get_world_matrix();
        }
        let data = self.world_transform.data.as_slice();
        return Vector3::new(data[12], data[13], data[14]);
    }
    pub fn set_position(&mut self, x: Float, y: Float, z: Float) {
        if self.parent != std::ptr::null_mut() {
        } else {
            self.set_local_scale(x, y, z);
        }
    }
    pub fn get_local_matrix(&self) -> Matrix4 {
        self.location_iso
            .to_homogeneous()
            .prepend_nonuniform_scaling(&self.local_scale)
    }
    pub fn get_world_matrix(&mut self) -> Matrix4 {
        if self._dirty_world {
            return self.world_transform;
        }
        self.root()._sync();
        return self.world_transform;
    }
    pub fn _sync(&mut self) {
        if self.parent != std::ptr::null_mut() {
            unsafe {
                let p = &*self.parent;
                self.world_transform = p.get_local_matrix() * self.get_local_matrix();
                self._dirty_world = true;
            }
        } else {
            self.world_transform = self.get_local_matrix();
            self._dirty_world = true;
        }
        for child in self.children.iter_mut() {
            child._sync();
        }
    }
    pub fn parent(&mut self) -> Option<&mut Node> {
        unsafe {
            if self.parent != std::ptr::null_mut() {
                return Some(&mut *self.parent);
            } else {
                None
            }
        }
    }
    pub fn root(&self) -> &mut Node {
        unsafe {
            let mut curr = &mut *self.parent;
            loop {
                if curr.parent != std::ptr::null_mut() {
                    curr = &mut *curr.parent;
                } else {
                    return &mut *curr;
                }
            }
        }
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
    // use approx::RelativeEq;
    let mut node = Node::new();

    node.set_local_position(1., 1., 1.);
    node.set_local_euler_angle(0.5 * std::f32::consts::PI, 0., 0.);
    node.set_local_scale(1., 2., 1.);
    // dbg!(&node.get_matrix());
    let mut child = Node::new();
    child.set_local_position(0., 2., 0.);
    node.add_child(child);

    let a = node.children[0].get_position();
    let b = Vector3::new(1., 1., 5.);

    assert!(relative_eq(
        a.data.as_slice().to_vec(),
        b.data.as_slice().to_vec()
    ))
}
