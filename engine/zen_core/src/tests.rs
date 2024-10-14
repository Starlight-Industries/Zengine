#[cfg(test)]
#[test]
fn override_classes() {
    use std::collections::HashMap;
    use crate::class_db::*;
    println!("Running Override class test");
    let mut db = get_class_db().lock().unwrap();
    println!("Test: Override, current classes in ClassDB: {:#?}", db.classes);
    let new_class = ClassInfo {
        name: String::from("OldClass"),
        properties: HashMap::new(),
        methods: HashMap::new(),
        parent: Some(String::from("Zobject"))
    };

    let _ = db.register_class(new_class.clone());

    match db.override_class(new_class, "OldClass") {
        Ok(_) => println!("Class successfully overridden."),
        Err(err) => panic!("Error: {:?}", err),
    }
}
#[test]
fn add_class_abstraction() {
    use crate::class_db::*;
    use std::collections::HashMap;
    println!("Running Override class test");
    let mut db = get_class_db().lock().unwrap();
    let properties = HashMap::new();
    let methods = HashMap::new();
    let parent = String::from("Zobject");

    // Add a class ergonomically
    match db.add_class("MyClass", properties, methods,Some(parent)) {
        Ok(_) => println!("Class 'MyClass' added successfully."),
        Err(err) => panic!("Error adding class: {:?}", err),
    }
}

#[test]
pub fn inheritance_test() {
    use crate::class_db::*;
    use std::collections::HashMap;
    println!("Running Inheritance test");
    let mut db = get_class_db().lock().unwrap();
    let properties = HashMap::new();
    let methods = HashMap::new();
    
    match db.add_class("ClassToInherit", properties.clone(), methods.clone(), Some(String::from("Zobject"))) {
        Ok(_) => println!("Class 'MyClass' added successfully."),
        Err(err) => panic!("Error adding class: {:?}. Current class stack: {:#?}", err,db.classes),
    }
    
    match db.add_class("MyInheritiedClass", properties, methods, Some(String::from("ClassToInherit"))) {
        Ok(_) => println!("Class 'OtherClass' added successfully,{:#?}",db.get_class("MyInheritiedClass")),
        Err(err) => panic!("Error adding class: {:?}", err),
    }
}