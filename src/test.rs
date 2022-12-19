// use std::sync::Mutex;
// lazy_static! {
//     static ref NODES: Mutex<Vec<Box<dyn NodeTrait>>> = Mutex::new(vec![]);
// }
use std::ptr::NonNull;
pub trait NodeTrait: Sync + Send {
    fn add_child(&mut self, child: Box<dyn NodeTrait>);
    fn set_parent(&mut self, parent: Option<NonNull<dyn NodeTrait>>) {}
}
unsafe impl Sync for Node {}
unsafe impl Send for Node {}
pub struct Node {
    pub name: String,
    pub parent: Option<NonNull<dyn NodeTrait>>,
    pub children: Vec<Box<dyn NodeTrait>>,
}
impl NodeTrait for Node {
    fn add_child(&mut self, mut child: Box<dyn NodeTrait>) {
        child.set_parent(NonNull::new(self));
        self.children.push(child);
    }
    fn set_parent(&mut self, parent: Option<NonNull<dyn NodeTrait>>) {
        self.parent = parent;
    }
}

// pub struct Entity {
//     pub __node: Node,
// }
// impl NodeTrait for Entity {
//     fn add_child(&mut self, child: Self) {
//         self.__node.add_child(child);
//     }
// }

fn main() {
    unsafe {
        let mut root = Node {
            name: "root".to_string(),
            parent: None,
            children: vec![],
        };
        let mut child = Node {
            name: "child".to_string(),
            parent: None,
            children: vec![],
        };
        root.add_child(Box::new(child));
        // let t = std::mem::transmute::<Box<dyn NodeTrait>, Box<Node>>(Box::new(root));
        // dbg!(t.name);
    }
}
