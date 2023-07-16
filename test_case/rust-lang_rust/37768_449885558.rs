rust
use std::thread;

pub struct A {
    a: String,
    b: i32,
}

impl A {
    pub fn foo(&self) {
        thread::spawn(move || {
            println!("something {}",&self.a);
        });
    }
}
