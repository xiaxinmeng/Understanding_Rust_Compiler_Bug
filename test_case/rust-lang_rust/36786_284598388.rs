rust
use std::sync::Mutex;

pub struct Foo(Mutex<Box<FnMut()>>);

impl Foo {
    pub fn foo(&self) {
        // self.0.lock().unwrap()(); // cannot borrow immutable `Box` content as mutable
        // (*self.0.lock().unwrap())(); // cannot borrow immutable `Box` content as mutable
        // (&mut self.0.lock().unwrap())(); // cannot borrow immutable `Box` content as mutable
        (&mut *self.0.lock().unwrap())();
    }
}

fn main() {
    Foo(Mutex::new(Box::new(|| println!("called")))).foo()
}
