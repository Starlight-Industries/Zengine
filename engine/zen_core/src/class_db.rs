use std::sync::LazyLock;

use smallvec::SmallVec;

use parking_lot::RwLock;
pub mod node;

#[derive(Debug)]
pub enum Error {
    ClassAlreadyExists(String),
    CannotOverrideBaseclass(String)
}
#[derive(Debug)]
pub struct ClassDB {
    pub classes: Vec<ClassInfo>
}
#[derive(Debug,PartialEq,Default,Clone)]
pub struct ClassInfo {
    pub name: String,
    pub methods: SmallVec<[Method;32]>, // you can define 32 methods untill they are managed in the heap
    pub properties: SmallVec<[Property;32]>,
    pub parent: Option<String>,
}
#[derive(Debug,PartialEq,Clone)]
pub struct Method {
    pub name: String,
    pub parameters: SmallVec<[ValueType;6]>,
    //pub ptr: &'a fn(dyn any) -> ValueType,

}
#[derive(Debug,PartialEq,Clone)]
pub struct Property {
    pub name: String,
    pub value: ValueType,
}

#[derive(Debug,PartialEq,Clone)]
pub enum ValueType {
    Float(f32),
    String(String),
    Int(i32),
    Bool(bool),
    Vec2(f32,f32)
}


static CLASS_DB: LazyLock<RwLock<ClassDB>> = LazyLock::new(||{
    let mut db = ClassDB { classes: Vec::new() };

    // Add the Zobject class upon initialization
    db.classes.push(ClassInfo {
        name: "Zobject".to_string(),
        methods: SmallVec::new(),
        parent: None,
        properties: SmallVec::new()
    });
    RwLock::new(db)
});

pub fn rand() {
    println!("{:#?}",std::time::SystemTime::now())
}

pub fn register_class(class_info: &ClassInfo) -> Result<(),Error> {
    for class in &CLASS_DB.read().classes {
        if class.name == class_info.name {
            return Err(Error::ClassAlreadyExists(class_info.name.clone()));
        }
    }
    if class_info.name == "Zobject" {
        eprintln!("You cannot override the base class. Try choosing a different name");
        return Err(Error::CannotOverrideBaseclass(class_info.name.clone()));
    }
    CLASS_DB.write().classes.push(class_info.to_owned());
    
    Ok(())
}

pub fn get_all() -> Vec<ClassInfo> {
    let mut vec_to_return:  Vec<ClassInfo> = vec![];
    for class in &CLASS_DB.read().classes {
        vec_to_return.push(class.clone());
    };
    
    vec_to_return
}
pub fn get_root() -> ClassInfo {
    CLASS_DB.read().classes.first().unwrap().to_owned()
}

impl ClassInfo {
    pub fn new(class_name: &str,parent:&str) -> Self {
        let new_class = ClassInfo {
            name: String::from(class_name),
            parent: Some(String::from(parent)),
            ..Default::default()
        };
        match register_class(&new_class) {
            Ok(_) => new_class,
            Err(e) => panic!("Failed to register class '{:#?}': {:#?}",class_name,e),
        }
    }
}