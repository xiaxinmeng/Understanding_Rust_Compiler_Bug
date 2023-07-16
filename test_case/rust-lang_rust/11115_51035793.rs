 rust
#![crate_type="lib"]

pub struct Foo {
    pub x: Bar
}

impl Foo {
    pub fn new() -> Foo {
        Foo { x: Bar}
    }
    pub fn get_x(&self) -> Bar {
        self.x
    }
}

struct Bar;

impl Bar {
    pub fn bar(&self) {
        println!("onoez");
    }
}
