rust
struct T;

trait A {
    type Type;
    fn create(&self) -> Self::Type;
}

impl A for T {
    type Type = i32;
    fn create(&self) -> Self::Type {
        10
    }
}

fn t<X: A>(x: X) -> X::Type {
    x.create()
}

fn main() {}
