use std::{any::{self, Any}, fmt, sync::{Arc, LazyLock}};
use ahash::{HashMap, HashMapExt};
use parking_lot::RwLock;

use crate::{printinfo, printwarn, throw};

#[derive(Debug)]
pub struct ClassDB {
    classes: HashMap<String, Arc<RwLock<Class>>>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub parent: Option<Arc<RwLock<Class>>>,
    pub callbacks: HashMap<String, Callback>,
    pub properties: HashMap<String, Value>
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

impl Class {
    pub fn print_data(&self) {
        printinfo!("Printing data for thing")
    }
    pub fn register_callback(&mut self,callback_name: &str,) -> Result<(), Error> {
        printinfo!("Attempting to register callback '{}' for ''",callback_name);
        if self.callbacks.contains_key(callback_name) {
            return throw!(Error::CallbackAlreadyExists(callback_name.to_string()))
        }
        self.callbacks.insert(callback_name.to_string(), Callback { name: "test".to_string(), event: "newevent".to_string() });
        printinfo!("Callback '{}' succsessfully registered",callback_name);
        Ok(())
    }
    pub fn add_property(&mut self) {}
}

pub static CLASS_DB: LazyLock<RwLock<ClassDB>> = LazyLock::new(|| {
    let mut db = ClassDB {
        classes: HashMap::new()
    };

    db.classes.insert(
        String::from("Zobject"),
        Arc::new(RwLock::new(Class {
            parent: None,
            callbacks: HashMap::new(),
            properties: HashMap::new() 
        }))
    );
    RwLock::new(db)
});

pub fn register_class(class_name: &str, parent: Option<&str>) -> Result<(), Error> {
    printinfo!("Attempting to register class '{}'", class_name);
    if class_name == "Zobject" {
        printwarn!("You cannot override the base class. Try choosing a different name");
        return throw!(Error::CannotOverrideBaseclass(String::from("Zobject")));
    }

    let db_read = CLASS_DB.read();
    if db_read.classes.contains_key(class_name) {
        printwarn!("A class with the same name was registered previously. Did you mean to override it?");
        return throw!(Error::ClassAlreadyExists(class_name.to_string()));
    }

    let parent_class = if let Some(parent_name) = parent {
        if let Some(parent) = db_read.classes.get(parent_name) {
            Some(Arc::clone(parent))
        } else {
            return throw!(Error::ParentClassNotFound(parent_name.to_string()));
        }
    } else {
        None
    };

    drop(db_read);

    let new_class = Class {
        parent: parent_class,
        callbacks: HashMap::new(),
        properties: HashMap::new(),
    };
    
    let new_class_arc = Arc::new(RwLock::new(new_class));

    printinfo!("Attempting to unlock classDB for write access");
    CLASS_DB.write().classes.insert(String::from(class_name), new_class_arc);
    printinfo!("Registered class '{:#?}' to ClassDB", class_name);
    Ok(())
}

pub fn get_class(class_name: &str) -> Result<Arc<RwLock<Class>>, Error> {
    printinfo!("Attempting to locate class '{}'", class_name);
    if let Some(found_class) = CLASS_DB.read().classes.get(class_name) {
        printinfo!("Class found");
        Ok(Arc::clone(found_class))
    } else {
        printwarn!("Class wasn't found. Is it registered?");
        throw!(Error::ClassNotFound(class_name.to_string()))
    }
}

// pub fn print_debug() {
//     let classes = &CLASS_DB.read().classes;
//     let parent_name = classes.iter().d.parent.as_ref()
//     .and_then(|parent| 
//         classes.iter()
//             .find(|(_, c)| Arc::ptr_eq(c, parent))
//             .map(|(name, _)| name.clone())
//     )
//     .unwrap_or_else(|| String::from("None"));
//     sprintinfo!("{:#?}", classes)
// }



#[derive(Debug)]
pub enum Error {
    ClassAlreadyExists(String),
    ClassNotFound(String),
    CannotOverrideBaseclass(String),
    ParentClassNotFound(String),
    CallbackAlreadyExists(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ClassAlreadyExists(name) => write!(f, "Class already exists: {}", name),
            Error::CannotOverrideBaseclass(name) => write!(f, "Cannot override baseclass: {}", name),
            Error::ClassNotFound(name) => write!(f, "Cannot find class '{}' within ClassDB", name),
            Error::ParentClassNotFound(name) => write!(f, "Parent class '{}' not found", name),
            Error::CallbackAlreadyExists(name) => write!(f,"Callback with the same name '{}' was previously registered",name)
        }
    }
}

pub fn test() -> Result<Error, Error> {
    return throw!(Error::CannotOverrideBaseclass(String::from("Zobject")));
}

#[derive(Debug, Clone)]
pub struct Callback {
    name: String,
    //event: fn(&(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>>
    event: String //temporary
}
impl  Callback {
    pub fn bind_event(&mut self) {}
    
}

pub fn print_debug() {
    let db_read = CLASS_DB.read();
    println!("└─ClassDB Contents:");
    for (idx, (name, class_arc)) in db_read.classes.iter().enumerate() {
        let is_last_class = idx == db_read.classes.len() - 1;
        let class = class_arc.read();
        let parent_name = class.parent.as_ref()
            .and_then(|parent|
                db_read.classes.iter()
                    .find(|(_, c)| Arc::ptr_eq(c, parent))
                    .map(|(name, _)| name.as_str())
            )
            .unwrap_or("None");
            
        if is_last_class {
            println!("  └─Class: {}", name);
            println!("    ├─Parent {}", parent_name);
            println!("    ├─Properties");
        } else {
            println!("  ├─Class: {}", name);
            println!("  │ ├─Parent {}", parent_name);
            println!("  │ ├─Properties");
        }

        if class.properties.is_empty() {
            if is_last_class {
                println!("    │ └─None");
            } else {
                println!("  │ │ └─None");
            }
        } else {
            for (prop_idx, (prop_name, value)) in class.properties.iter().enumerate() {
                let is_last_prop = prop_idx == class.properties.len() - 1;
                if is_last_class {
                    if is_last_prop {
                        println!("    │ └─{}: {:?}", prop_name, value);
                    } else {
                        println!("    │ ├─{}: {:?}", prop_name, value);
                    }
                } else {
                    if is_last_prop {
                        println!("  │ │ └─{}: {:?}", prop_name, value);
                    } else {
                        println!("  │ │ ├─{}: {:?}", prop_name, value);
                    }
                }
            }
        }
        
        if is_last_class {
            println!("    └─Callbacks");
        } else {
            println!("  │ └─Callbacks");
        }

        if class.callbacks.is_empty() {
            if is_last_class {
                println!("      └─None");
            } else {
                println!("  │   └─None");
            }
        } else {
            for (cb_idx, (callback_name, callback)) in class.callbacks.iter().enumerate() {
                let is_last_cb = cb_idx == class.callbacks.len() - 1;
                if is_last_class {
                    if is_last_cb {
                        println!("      └─{}: {}", callback_name, callback.name);
                    } else {
                        println!("      ├─{}: {}", callback_name, callback.name);
                    }
                } else {
                    if is_last_cb {
                        println!("  │   └─{}: {}", callback_name, callback.name);
                    } else {
                        println!("  │   ├─{}: {}", callback_name, callback.name);
                    }
                }
            }
        }
    }
}