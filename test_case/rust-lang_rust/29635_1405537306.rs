rust
mod foo {
    pub struct Foo;
}

mod bar {
    pub trait Bar {
        fn bar(&self);
    }

    impl Bar for super::Foo {
        fn bar(&self) {
            println!("Bar.bar");
        }
    }
}

use foo::Foo;
use bar::Bar;

fn main() {
    Foo.bar();
}
