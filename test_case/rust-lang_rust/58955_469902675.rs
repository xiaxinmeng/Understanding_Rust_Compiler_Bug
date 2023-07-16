rust
use std::any::Any;

struct A;

impl A {
    pub fn by_mut(&mut self) {
        self.type_id();
    }

    fn type_id(&self) {
        println!("hi");
    }
}

fn main() {
    A.by_mut();
}
