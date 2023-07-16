
#![feature(core)]

use std::any::Any;

trait Foo: Any {
    fn quux(&self) -> bool;
}

impl PartialEq for Foo {
    fn eq(&self, rhs: &Foo) -> bool {
        self.get_type_id() == rhs.get_type_id()
    }
}

struct Bar;
impl Foo for Bar {
    fn quux(&self) -> bool { true }
}

struct Baz;
impl Foo for Baz {
    fn quux(&self) -> bool { false }
}

fn main() {
    println!("{}", &Bar as &Foo == &Bar as &Foo); //true
    println!("{}", &Baz as &Foo == &Baz as &Foo); //true
    println!("{}", &Bar as &Foo == &Baz as &Foo); //false
}
