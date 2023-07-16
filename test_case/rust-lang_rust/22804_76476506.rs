 rust
pub mod foo {
    struct X;
    pub type Foo = X;

    impl Foo {
        pub fn foo() { println!("Hello!"); }
    }
}

use foo::Foo;

fn main() {
    Foo::foo();
}
