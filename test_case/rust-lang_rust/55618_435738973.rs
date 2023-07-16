rust
mod foo {
    pub struct Foo;
}

mod bar {
    pub struct Bar;
}

mod baz {
    // This declaration...
    use crate::*;

    // ...enables these:
    use foo::Foo;
    use bar::Bar;

    pub fn baz() {
        let _ = Foo;
        let _ = Bar;
    }
}

fn main() {}
