#[cfg(test)]

#[test]
fn override_classes() {
    use std::collections::HashMap;
    use crate::class_db::*;
    println!("Running Override class test");
    let mut db = get_class_db().lock().unwrap();
    println!("{:#?}", db.classes);
    let new_class = ClassInfo {
        name: String::from("OldClass"),
        properties: HashMap::new(),
        methods: HashMap::new(),
    };
    // db.classes
    //     .insert(String::from("NewClass"), new_class.clone());
    db.register_class(new_class.to_owned());

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

    // Add a class ergonomically
    match db.add_class("MyClass", properties, methods) {
        Ok(_) => println!("Class 'MyClass' added successfully."),
        Err(err) => panic!("Error adding class: {:?}", err),
    }
}
