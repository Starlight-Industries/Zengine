use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
#[derive(Debug, Clone,Default)]
pub struct ClassInfo {
    
    pub name: String,
    pub properties: HashMap<String, ValueType>,
    pub parent: Option<String>,
    pub methods: HashMap<String, fn(&mut Self)>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ClassNotFound(Box<str>),
    ClassAlreadyExists(Box<str>),
    ParentClassNotFound(Box<str>),
    PropertyAlreadyDefined(Box<str>),
    MethodAlreadyDefined(Box<str>),
    InvalidRootClass(Box<str>)
}
#[derive(Debug, Clone)]
pub enum ValueType {
    Int(i32),
    Float(f32),
    Double(f64),
    String(String),
    Bool(bool),
    Vector2(f32,f32),
    Vector3(f32,f32,f32)
}
#[derive(Debug, Clone,Default)]
pub struct ClassDB {
    pub classes: HashMap<String, ClassInfo>,
}

impl ClassDB {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register_class(&mut self, class: ClassInfo) -> Result<(), Error> {
        if self.classes.contains_key(&class.name) {
            Err(Error::ClassAlreadyExists(class.name.as_str().into()))
        } else {
            self.classes.insert(class.name.clone(), class);
            Ok(())
        }
    }
    pub fn get_class(&self, class_name: &str) -> Option<&ClassInfo> {
        self.classes.get(class_name)
    }
    pub fn add_class(
        &mut self,
        class_name: &str,
        properties: HashMap<String, ValueType>,
        methods: HashMap<String, fn(&mut ClassInfo)>,
        parent: Option<String>,
    ) -> Result<(), Error> {
        if parent.is_none() && class_name != "Zobject" {
            return Err(Error::InvalidRootClass(class_name.into())); 
        } // There can only be one free standing class in play at a time, that being Zobject (base class)
        if self.classes.contains_key(class_name) {
            return Err(Error::ClassAlreadyExists(class_name.into()));
        }

        if let Some(ref parent_name) = parent {
            if !self.classes.contains_key(parent_name) {
                    return Err(Error::ParentClassNotFound(parent_name.as_str().into()));
            }

            if let Some(parent_class) = self.classes.get(parent_name) {
                for property in properties.keys() {
                    if parent_class.properties.contains_key(property) {
                        return Err(Error::PropertyAlreadyDefined(property.as_str().into()));
                    }
                }
                for method in methods.keys() {
                    if parent_class.properties.contains_key(method) {
                        return Err(Error::MethodAlreadyDefined(method.as_str().into()));
                    }
                }
                let mut inherited_properties = parent_class.properties.clone();

                let mut inherited_methods = parent_class.methods.clone();

                inherited_properties.extend(properties); 
                inherited_methods.extend(methods);

                let class_info = ClassInfo {
                    name: class_name.to_string(),
                    properties: inherited_properties,
                    methods: inherited_methods,
                    parent
                };
                self.classes.insert(class_name.to_string(), class_info);
                return Ok(())
        } else {
            return Err(Error::ParentClassNotFound(parent_name.as_str().into()));
        } 
        } else {
        let class_info = ClassInfo {
            name: class_name.to_string(),
            properties,
            methods,
            parent: None,
        };

        self.classes.insert(class_name.to_string(), class_info);
        Ok(())
    }

}
    pub fn override_class(&mut self, new_class: ClassInfo, old_class: &str) -> Result<(), Error> {
        if let Some(class) = self.classes.get_mut(old_class) {
            *class = new_class;
            Ok(())
        } else {
            Err(Error::ClassNotFound(old_class.into()))
        }
    }
}

pub static CLASS_DB: OnceLock<Mutex<ClassDB>> = OnceLock::new();
pub fn get_class_db() -> &'static Mutex<ClassDB> {
    CLASS_DB.get_or_init(|| {
        let db = Mutex::new(ClassDB {
            classes: HashMap::new(),
        });
        let z_object = ClassInfo {
            name: String::from("Zobject"),
            ..Default::default()
        };

        //let base_properties = HashMap::new();
        //let base_methods = HashMap::new();
        //let _ = db.lock().unwrap().add_class("Zobject", base_properties, base_methods, None);
        let _ = db.lock().unwrap().register_class(z_object);
        db
    })
}
