use class_db::{node::Node, register_class, ClassDB, ClassInfo};



pub mod class_db;
pub mod scene_tree;
pub mod tests;
pub mod renderer;
fn main() {
    let test_class = ClassInfo {
        name:"Root".to_string(),
        ..Default::default()
    };
    let test_class2 = ClassInfo {
        name:"Child1".to_string(),
        ..Default::default()
    };
    let test_class3 = ClassInfo {
        name:"Child2".to_string(),
        ..Default::default()
    };
    let mut root_node = Node::new(class_db::get_root(),0,"root");
    let mut child_node = Node::new(test_class2, 2,"child");
    let mut child_node2 = Node::new(test_class3, 6,"other_child");
    root_node.add_child(child_node);
    for _ in 0..100 {
        root_node.add_child(child_node2.clone());
    }
    root_node.print_node(0);
    loop {
    }
}
