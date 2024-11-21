use std::fmt;

// We can use macro #[derive(Debug)] for simple printing, but
// without direct read of struct field we either have to:
// - make then internal with heading underscore
// - implement special printing method with specific use of each field
// - make them Public and use in another package
struct Object {
    name: String,
    data: i32,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Object {{ name: '{}'; data: {} }}", self.name, self.data)
    }
}

fn main() {
    let objects = get_objects();
    println!("{:#?}", objects);
}

fn get_objects() -> Vec<Object> {
    vec![
        Object {
            name: String::from("Alpha").to_lowercase(),
            data: 32,
        },
        Object {
            name: "bravo".to_lowercase().to_string(),
            data: 64,
        },
    ]
}
