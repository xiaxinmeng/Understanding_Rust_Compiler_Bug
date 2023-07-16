 rust
mod foo {
    pub struct Foo;

    impl Foo {
        fn bar(&self) {
            println!("private bar in foo");
        }
    }

    pub trait Baz {
        fn bar(&self);
    }

    impl Baz for Foo {
        fn bar(&self) {
            println!("public bar in foo from Baz");
        }
    }
}

fn main() {
    let mut f = foo::Foo;
    f.bar(); // doesn't work

    let b: &mut foo::Baz = &mut f;

    b.bar(); // works
}
