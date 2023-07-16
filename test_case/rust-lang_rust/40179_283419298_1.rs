rust
#![feature(pub_restricted)]

#[derive(Debug)]
enum Foo { // Foo is `pub(crate)` too if it's defined in the crate root
    Var1,
    Var2
}

mod bar {
    use super::Foo;

    pub(crate) fn get_foo() -> Foo {
        Foo::Var1
    }
}

fn main() {
    println!("{:?}", bar::get_foo());
    println!("Hello, world!");
}
