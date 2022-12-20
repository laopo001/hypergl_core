// use std::sync::Mutex;
// lazy_static! {
//     static ref NODES: Mutex<Vec<Box<dyn NodeTrait>>> = Mutex::new(vec![]);
// }
use std::{any::Any, ptr::NonNull};
pub trait NodeTrait: Sync + Send {
    fn add_child(&mut self, child: Box<dyn NodeTrait>);
    fn set_parent(&mut self, parent: Option<NonNull<dyn NodeTrait>>);
    fn as_any(&self) -> &dyn Any;
}
unsafe impl Sync for Node {}
unsafe impl Send for Node {}
pub struct Node {
    pub name: String,
    pub parent: Option<NonNull<dyn NodeTrait>>,
    pub children: Vec<Box<dyn NodeTrait>>,
}
impl Node {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            parent: None,
            children: vec![],
        };
    }
    pub fn say_hello(&self) {
        println!("hello");
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

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
    fn as_any(&self) -> &dyn Any {
        self
    }
}
use std::ops::Deref;

impl Deref for Entity {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        return &self.__node;
    }
}

#[test]
fn test() {
    unsafe {
        let mut root = Node::new("root");
        let mut child = Node::new("child");
        root.add_child(Box::new(child));

        let t = root.children[0]
            .as_ref()
            .as_any()
            .downcast_ref::<Node>()
            .unwrap()
            .parent
            .unwrap()
            .as_ref()
            .as_any()
            .downcast_ref::<Node>()
            .unwrap();
        dbg!(&t.name);

        let mut root = Entity::new("root2");
        let mut child = Entity::new("child2");
        root.add_child(Box::new(child));

        {
            let t = root.children[0]
                .as_ref()
                .as_any()
                .downcast_ref::<Entity>()
                .unwrap();
            dbg!(&t.name);
            t.say_hello();
        }

        {
            let t = root.children[0]
                .as_ref()
                .as_any()
                .downcast_ref::<Entity>()
                .unwrap()
                .parent
                .unwrap()
                .as_ref()
                .as_any()
                .downcast_ref::<Node>()
                .unwrap();
            dbg!(&t.name);
            t.say_hello();
        }
    }
}
