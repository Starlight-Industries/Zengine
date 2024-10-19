use std::{fmt, rc::Rc};
use parking_lot::{Mutex, RwLock};

use super::get_root;
use super::ClassInfo;
use super::ClassDB;

#[derive(Debug)]
pub struct Node {
    pub class: ClassInfo,
    pub name: String,
    pub id: usize,
    pub children: Vec<Box<Node>>
}

impl Node {
    pub fn new(class: ClassInfo, name: &str) -> Self {
        let node_id: usize = fastrand::usize(1..1024*32);
        Node {
            class,
            name: name.to_string(),
            id: node_id,
            children: Vec::new(),
        }
    }

    pub fn print_node(&self, level: usize) {
        let indent = "  ".repeat(level);
        println!("{}Node name: {}, Node class: {}, Node ID: {}", indent, self.name, self.class.name,self.id);
        for child in &self.children {
            child.print_node(level + 1);
        }
    }

    pub fn add_child(&mut self, child: Node) {
        // for for child in self.
        self.children.push(Box::new(child));
    }

    pub fn remove_child(&mut self, child_id: usize) -> Option<Node> {
        if let Some(pos) = self.children.iter().position(|child| child.id == child_id) {
            Some(*self.children.remove(pos))
        } else {
            None
        }
    }

    pub fn find_node_id(&self, node_id: usize) -> Option<&Node> {
        if self.id == node_id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_node_id(node_id) {
                return Some(found);
            }
        }
        None
    }
}
#[derive(Debug,)]
pub struct SceneTree<'a> {
    pub root: &'a Box<Node>,
}

impl <'a>SceneTree<'a> {
    pub fn new() -> Self {
        
        let root_node: Box<Node> = Box::new(Node::new(get_root(),"root"));
        let new_tree = SceneTree { root: &mut *Box::new(root_node) };
        return new_tree;
    }
    fn fmt_node(&self, node: &Node, level: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = "  ".repeat(level);
        writeln!(f, "{}🟠 Node name: {:#?}, ID: {:#?}", indent, node.name, node.id)?;
        for child in &node.children {
            self.fmt_node(child, level + 1, f)?;
        }
        Ok(())
    }

    pub fn print_tree(&self) {
        self.root.print_node(0);
    }
}

impl fmt::Display for SceneTree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_node(&self.root, 0, f)
    }
}

