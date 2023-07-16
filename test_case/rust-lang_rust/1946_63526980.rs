 rust
use foo::Foo;
use bar::Bar;

mod foo {
    pub trait Foo {
        fn baz(&self) {}
    }
}

mod bar {
    pub trait Bar {
        fn baz(&self) {}
    }
}

struct Quux;

#[cfg(not(error))]
impl Quux {
    fn baz(&self) {}
}

impl foo::Foo for Quux {}
impl bar::Bar for Quux {}

fn main() {
    Quux.baz();
}
