use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
fn main() {
    println!("Hello, world!");
}

pub struct ClassInfo {
    pub name: String,
    pub properties: HashMap<String, ValueType>,
    pub methods: HashMap<String, fn(&mut Self)>,
}
#[derive(Debug)]
pub enum Error {
    ClassNotFound,
}
pub enum ValueType {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

pub struct ClassDB {
    classes: HashMap<String, ClassInfo>,
}

impl ClassDB {
    pub fn new() -> Self {
        ClassDB {
            classes: HashMap::new(),
        }
    }
    pub fn register_class(&mut self, class: ClassInfo) {
        self.classes.insert(class.name.clone(), class);
    }
    pub fn get_class(&self, class_name: &str) -> Option<&ClassInfo> {
        self.classes.get(class_name)
    }
    pub fn override_class(&mut self, new_class: ClassInfo, old_class: &str) -> Result<(), Error> {
        if let Some(class) = self.classes.get_mut(old_class) {
            *class = new_class;
            Ok(())
        } else {
            Err(Error::ClassNotFound)
        }
    }
}

static CLASS_DB: OnceLock<Mutex<ClassDB>> = OnceLock::new();
fn get_class_db() -> &'static Mutex<ClassDB> {
    CLASS_DB.get_or_init(|| {
        Mutex::new(ClassDB {
            classes: HashMap::new(),
        })
    })
}
