use class_db::{node::Node, ClassInfo};
pub mod class_db;
pub mod scene_tree;
pub mod tests;
pub mod renderer;

fn main() {
    let test_class = ClassInfo {
        name: "Child1".to_string(),
        ..Default::default()
    };
    let test_class2 = ClassInfo {
        name: "Child1".to_string(),
        ..Default::default()
    };
    let test_class3 = ClassInfo {
        name: "Child2".to_string(),
        ..Default::default()
    };

    let mut root_node = Node::new(class_db::get_root(), 0, "root");
    let mut child_node = Node::new(test_class2, 2, "child");
    let mut child_node2 = Node::new(test_class3, 6, "other_child");
    let child_node3 = Node::new(test_class, 7, "hasdfas");


    child_node2.add_child(child_node3);
    child_node.add_child(child_node2);
    root_node.add_child(child_node);
    

    root_node.print_node(0);
}