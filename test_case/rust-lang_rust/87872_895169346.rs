rust
pub mod foo {
    #[derive(Default)]
    pub struct Foo {
        pub x: i32,
        y: i32,
    }
}

fn main() {
    let foo::Foo {} = foo::Foo::default();
}
