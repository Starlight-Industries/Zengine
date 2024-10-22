use core::sync;
use std::{any::{self, Any}, fmt, sync::{Arc, LazyLock}};
use ahash::{HashMap, HashMapExt};
use parking_lot::RwLock;

use crate::{printinfo, printwarn, throw};
#[derive(Debug)]
pub struct ClassDB<'a> {
    classes: HashMap<String,RwLock<Arc<Class<'a>>>>, // dear god help
}

#[derive(Debug, Clone)]
pub struct Class<'a> {
    pub parent: Option<&'a Class<'static>>,
    //pub methods: HashMap<String, fn(&[Value]) -> Value>,
    pub callbacks: HashMap<String,Callback>,
    pub properties: HashMap<String,Value>
}


#[derive(Debug, Clone)]
pub enum Value {
    Float(f32),
    Double(f64),
    Int(i64),
    String(Box<str>),
    Bool(bool),
    Vector2(f32, f32),
    Vector3(f32, f32, f32),
    Void,
}

impl Class<'static> {
    pub fn print_data(&self) {
        printinfo!("Printing data for thing")
    }
    // pub fn bind_method(&mut self, method_name: &str, method: fn(Value) -> Value) {
    //     printinfo!("Binding method '{}'",method_name)
    // }
    pub fn register_callback(&mut self) {}
    pub fn add_property(&mut self) {}
}

type ZClass<'a> = Class<'static>;
pub static CLASS_DB: LazyLock<RwLock<ClassDB>> = LazyLock::new(|| {
    let mut db = ClassDB {
        classes: HashMap::new()
    };

    db.classes.insert(String::from("Zobject",), RwLock::new(Arc::new(Class {
        parent: None,
        callbacks: HashMap::new(),
        properties: HashMap::new() 
    })));
    RwLock::new(db)
});

pub fn register_class(class_name: &str, parent: Option<&str>) -> Result<(), Error> {
    printinfo!("Attempting to register class '{}'", class_name);
    if class_name == "Zobject" {
        printwarn!("You cannot override the base class. Try choosing a different name");
        return throw!(Error::CannotOverrideBaseclass(String::from("Zobject")));
    }
    let mut find_parent: bool = false;
    if parent.is_some() { find_parent = true}
    for class in CLASS_DB.read().classes.keys() {
        if class == class_name {
            printwarn!("A class with the same name was registered previously. Did you mean to override it?");
            return throw!(Error::ClassAlreadyExists(class_name.to_string()));
        }
    }
    if find_parent {
        for class in &CLASS_DB.read().classes {
            if Some(class.0.as_str()) == parent  {
                printinfo!("thing");
            }
        }
    }
    drop(CLASS_DB.read());

    let new_class = Class {
        parent: None,
        callbacks: HashMap::new(),
        properties: HashMap::new(),
    };
    printinfo!("Attempting to unlock classDB for write access");
    CLASS_DB.write().classes.insert(String::from(class_name),RwLock::new(Arc::new(new_class)));
    printinfo!("Registered class '{:#?} to ClassDB'", class_name);
    Ok(())
}

pub fn get_class(class_name: &str,) -> Result<Arc<Class>, Error> {
    printinfo!("Attempting to locate class '{}'", class_name);
    if let Some(found_class) = CLASS_DB.read().classes.get(class_name){
        printinfo!("Class found");
        Ok(Arc::clone(&found_class.read()))
    } else {
        printwarn!("Class wasnt found. Is it registered?");
        return throw!(Error::ClassNotFound(class_name.to_string()));
    }
}
pub fn print_debug() {
    let classes = &CLASS_DB.read().classes;
    printinfo!("{:#?}", classes)
}
#[derive(Debug)]
pub enum Error {
    ClassAlreadyExists(String),
    ClassNotFound(String),
    CannotOverrideBaseclass(String),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ClassAlreadyExists(name) => write!(f, "Class already exists: {}", name),
            Error::CannotOverrideBaseclass(name) => {
                write!(f, "Cannot override baseclass: {}", name)
            }
            Error::ClassNotFound(name) => write!(f,"Cannot find class '{}' within ClassDB",name)
        }
    }
}

pub fn test() -> Result<Error, Error> {
    return throw!(Error::CannotOverrideBaseclass(String::from("Zobject")));
}
#[derive(Debug, Clone)]
struct Callback {
    name: String,
    event: fn(&(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>>


}