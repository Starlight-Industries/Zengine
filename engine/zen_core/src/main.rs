use std::borrow::Borrow;

use class_db::{node::{self, Node, SceneTree}, ClassInfo};
pub mod class_db;
pub mod scene_tree;
pub mod tests;
pub mod renderer;

fn main() {

    let root_node: Box<Node> = Box::new(Node::new(class_db::get_root(),"root"));
    let scene_tree = SceneTree::new(&root_node);
    for _ in 0..1 {
        let new_child = Node::new(ClassInfo::new("NewClass", "root"),"sd" );
        let new_nod = scene_tree.root.find_node_id(0);

        if new_nod.is_some() {
            println!("new nod is!!{:#?}",new_nod)
        } else {
            panic!("node 0 doesnt exist!")
        }
    }
    


    scene_tree.print_tree();
    class_db::get_all();
    root_node.print_node(0);

}