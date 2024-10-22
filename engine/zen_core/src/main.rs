
use class_db::*;

pub mod class_db;
pub mod config;
pub mod r#macro;

fn main() {
    match register_class("newclass", Some("Zobject")) {
        Ok(_) => printinfo!("it work"),
        Err(_) => printerr!("it didnt"),
    }
    match register_class("newclass2", Some("Zobject")) {
        Ok(_) => printinfo!("it work"),
        Err(_) => printerr!("it didnt"),
    }
    let new = get_class("newclass").unwrap().write().register_callback("testsdf",);
    
    print_debug();
    
}