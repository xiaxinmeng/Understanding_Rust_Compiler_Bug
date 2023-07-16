Rust
use std::boxed::Box;

fn Box<T>(t: T) -> Box<T> {
    Box::new(t)
}

mod foo {
    pub struct MyFoo<T>(T);
    impl<T> MyFoo<T> {
        pub fn new(t: T) -> Self {
            MyFoo(t)
        }
    }
}

use foo::MyFoo;

fn MyFoo<T>(t: T) -> MyFoo<T> {
    MyFoo::new(t)
}

fn main() {
    let MyFoo = MyFoo(5);
}
