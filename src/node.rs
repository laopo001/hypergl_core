use cgmath::prelude::*;
use cgmath::{Deg, Euler, Matrix4, Point3, Quaternion, Vector3, Zero};

pub type Vec3 = Vector3<f32>;
pub type Quat = Quaternion<f32>;
pub type Mat4 = Matrix4<f32>;
pub type Point3f32 = Point3<f32>;
pub type Eulerf32 = Euler<Deg<f32>>;

#[allow(unused_imports)]
use std::cell::{RefCell, UnsafeCell};
#[allow(unused_imports)]
use std::ops::Deref;
#[allow(unused_imports)]
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub local_position: Vec3,
    pub local_rotation: Quat,
    pub local_scale: Vec3,
    pub local_euler_angle: Vec3,
    pub local_transform: Mat4,
    pub world_position: Vec3,
    pub world_rotation: Quat,
    pub world_euler_angle: Vec3,
    // pub world_scale: Box<Vec3>,
    pub world_transform: Mat4,
    pub parent: *mut Node,
    pub children: Vec<*mut Node>,
    _dirty_local: bool,
    _dirty_world: bool,
    enabled: bool,
}

impl Node {
    pub fn new() -> Node {
        return Node {
            local_position: Vec3::zero(),
            local_rotation: Quat::new(1.0, 0.0, 0.0, 0.0),
            local_euler_angle: Vec3::zero(),
            local_scale: Vec3::new(1.0, 1.0, 1.0),
            local_transform: Mat4::from_translation(Vec3::zero()),
            world_position: Vec3::zero(),
            world_rotation: Quat::new(1.0, 0.0, 0.0, 0.0),
            world_euler_angle: Vec3::zero(),
            // world_scale: box Vec3::new(1.0, 1.0, 1.0),
            world_transform: Mat4::from_translation(Vec3::zero()),
            parent: std::ptr::null_mut(),
            children: vec![],
            _dirty_local: false,
            _dirty_world: false,
            enabled: true,
        };
    }

    pub fn add_child(&mut self, child: &mut Node) {
        child.parent = self;
        self.children.push(child);
    }

    pub fn set_local_position(&mut self, x: f32, y: f32, z: f32) {
        self.local_position = Vec3::new(x, y, z);
        if !self._dirty_local {
            self._dirtify(true);
        }
    }
    pub fn get_local_position(&self) -> Vec3 {
        self.local_position
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        unsafe {
            if self.parent.is_null() {
                self.local_position = Vec3::new(x, y, z);
            } else {
                let mut inv_parent_transform = (*(*self.parent).get_world_transform()).clone();
                inv_parent_transform.invert();
                self.local_position = inv_parent_transform
                    .transform_point(Point3f32::new(x, y, z))
                    .to_vec();
            }
            if !self._dirty_local {
                self._dirtify(true);
            }
        }
    }
    pub fn get_position(&mut self) -> &Vec3 {
        unsafe {
            let world_transform_ptr = self.get_world_transform();
            self.world_position = (*world_transform_ptr).get_translate();
        }
        return &self.world_position;
    }
    fn get_rotation(&mut self) -> &Quat {
        unsafe {
            let world_transform_ptr = self.get_world_transform();
            self.world_rotation.set_from_mat4(&*world_transform_ptr);
            return &self.world_rotation;
        }
    }

    pub fn set_local_euler_angles(&mut self, x: f32, y: f32, z: f32) {
        self.local_rotation = Quat::from(Eulerf32::new(Deg(x), Deg(x), Deg(x)));
        // self.local_rotation.set_from_euler_angles(x, y, z);
        if !self._dirty_local {
            self._dirtify(true);
        }
    }
    pub fn get_local_euler_angles(&mut self) -> &Vec3 {
        self.local_rotation
            .get_euler_angles(self.local_euler_angle.as_mut());
        return self.local_euler_angle.as_ref();
    }

    pub fn set_euler_angles(&mut self, x: f32, y: f32, z: f32) {
        self.local_rotation.set_from_euler_angles(x, y, z);
        unsafe {
            if !self.parent.is_null() {
                let mut inv_parent_rotation = (*self.parent).get_rotation().clone();
                inv_parent_rotation.invert();
                self.local_rotation.as_mut().mul(&inv_parent_rotation);
            }
            if !self._dirty_local {
                self._dirtify(true);
            }
        }
    }
    pub fn get_euler_angles(&mut self) -> &Vec3 {
        unsafe {
            let world_transform = &*self.get_world_transform();
            world_transform.get_euler_angles(self.world_euler_angle.as_mut());
            return self.world_euler_angle.as_ref();
        }
    }

    pub fn get_world_transform(&mut self) -> *mut Mat4 {
        if self._dirty_local == false && self._dirty_world == false {
            return &mut self.world_transform;
        }
        if !self.parent.is_null() {
            unsafe {
                (*self.parent).get_world_transform();
            }
        }
        self._sync();
        return &mut self.world_transform;
    }

    pub fn get_local_transform(&mut self) -> *mut Mat4 {
        if self._dirty_local {
            self._sync();
        }
        return self.local_transform.get();
    }

    pub fn set_local_scale(&mut self, x: f32, y: f32, z: f32) {
        self.local_scale.as_mut().set(x, y, z);
        if !self._dirty_local {
            self._dirtify(true);
        }
    }
    fn get_local_scale(&mut self) -> &Vec3 {
        self.local_scale.as_ref()
    }

    fn _dirtify(&mut self, local: bool) {
        if local {
            self._dirty_local = true;
        }
        if !self._dirty_world {
            self._dirty_world = true;
            for item in self.children.iter() {
                unsafe {
                    (**item)._dirtify(false);
                }
            }
        }
    }

    pub fn sync_hierarchy(&mut self) {
        if !self.enabled {
            return;
        }
        if self._dirty_local || self._dirty_world {
            self._sync();
        }
        for i in 0..(self.children.len()) {
            unsafe {
                (*self.children[i]).sync_hierarchy();
            }
        }
    }
    pub fn _sync(&mut self) {
        let local_transform_ptr = self.local_transform.get();
        let world_transform_ptr = self.world_transform.get();
        unsafe {
            if self._dirty_local {
                (*local_transform_ptr).set_from_trs(
                    &self.local_position,
                    &self.local_rotation,
                    &self.local_scale,
                );
                self._dirty_local = false;
            }
            if self._dirty_world {
                if self.parent.is_null() {
                    let temp = &*local_transform_ptr;
                    // release 编译会无限循环 所以加上了clone
                    (*world_transform_ptr).copy(&temp.clone());
                } else {
                    let parent_world_transform_ptr = (*self.parent).world_transform.get();
                    (*world_transform_ptr)
                        .mul2(&*parent_world_transform_ptr, &*local_transform_ptr);
                }
                self._dirty_world = false;
            }
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Node {
        let mut c = Node::new();
        c.local_position = self.local_position.clone();
        c.local_rotation = self.local_rotation.clone();
        c.local_scale = self.local_scale.clone();
        // c._dirty_local = false;
        // c._dirty_world = false;
        for child in self.children.iter() {
            unsafe {
                let mut clone_child = (**child).clone();
                c.add_child(&mut clone_child);
            }
        }
        return c;
    }
}

trait HyperMat4 {
    fn get_translate(&self) -> Vec3;
}

impl HyperMat4 for Mat4 {
    fn get_translate(&self) -> Vec3 {
        return Vec3::new(self.z.x, self.z.y, self.z.z);
    }
}
