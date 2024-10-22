use std::ops::Deref;

use class_db::*;

pub mod class_db;
pub mod config;
pub mod r#macro;

fn main() {
    register_class("Node",None);
    register_class("Node",None);
    let test = get_class("Zobject").unwrap();


    
}
