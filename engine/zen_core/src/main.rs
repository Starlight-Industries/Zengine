use std::{ops::Deref, time::Instant};

use class_db::*;

pub mod class_db;
pub mod config;
pub mod r#macro;

fn main() {
    match register_class("newclass", Some("Zobject")) {
        Ok(_) => printinfo!("it work"),
        Err(_) => printerr!("it didnt"),
    }

    let mut test = get_class("newclass").unwrap();
    test.register_callback();
    print_debug();

    
}
