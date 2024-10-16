use std::fmt;

use super::ClassInfo;
#[derive(Debug,PartialEq,Default,Clone)]
pub struct Node {
    class: ClassInfo,
    name: String,
    id: usize,
    children: Vec<Node>,
}

impl Node {
    pub fn new(class: ClassInfo, id: usize,name: &str) -> Self {
        Node {
            class,
            name: name.to_string(),
            id,
            children: Vec::new(),
        }
    }
    pub fn print_node(&self, level: usize) {
        let indent = "  ".repeat(level);
        println!("{}Node: {}, Class: {}", indent, self.name, self.class.name);
        for child in &self.children {
            child.print_node(level + 1);
        }
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child_id: usize) -> Option<Node> {
        if let Some(pos) = self.children.iter().position(|child| child.id == child_id) {
            Some(self.children.remove(pos))
        } else {
            None
        }
    }

    pub fn find_node(&self, node_id: usize) -> Option<&Node> {
        if self.id == node_id {
            return Some(self);
        }

        for child in &self.children {
            if let Some(found) = child.find_node(node_id) {
                return Some(found);
            }
        }

        None
    }
}
#[derive(Debug,PartialEq,Default,Clone)]
pub struct SceneTree {
    root: Node,
}

impl SceneTree {
    pub fn new(root: Node) -> Self {
        SceneTree { root }
    }
}

impl fmt::Display for SceneTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_node(&self.root, 0, f)
    }
}

impl SceneTree {
    fn fmt_node(&self, node: &Node, level: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = "  ".repeat(level);
        writeln!(f, "{} Node id: {}", indent, node.id)?;

        for child in &node.children {
            self.fmt_node(child, level + 1, f)?;
        }

        Ok(())
    }
}