 rust
mod foo {
    fn bar() -> super::Foo {
        super::Foo { x: 1 }
    }
    impl super::Foo { }
}

pub struct Foo { x: int }

fn main() {}
