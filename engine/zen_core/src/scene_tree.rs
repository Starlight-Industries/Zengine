use std::{collections::HashMap, f32, process::Child, sync::{Arc, Mutex}};

use crate::class_db::{ClassInfo, ValueType};



#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    class: ClassInfo,
    children: Vec<Arc<Mutex<Node>>>,
    parent: Option<Arc<Mutex<Node>>>,
}

impl Node {
    pub fn new(name: String, class_name: String) -> Self {
        Node {
            name,
            children: Vec::new(),
            parent: None,
            class: ClassInfo::default(),
        }
    }
    pub fn add_child(&mut self, child: Arc<Mutex<Node>>) {
        let mut child_lock = child.lock().unwrap();
        child_lock.parent = Some(Arc::new(Mutex::new(self.clone())));
        self.children.push(child.to_owned());
    }
    pub fn remove_child(&mut self, child_name: &str) {
        self.children.retain(|child| child.lock().unwrap().name != child_name);
    }
    pub fn update(&mut self, delta: f32) {
        println!("Updating {}!", self.name);
        for child in &self.children {
            child.lock().unwrap().update(delta);
        }
    }
}