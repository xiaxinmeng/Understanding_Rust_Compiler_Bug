 rust
pub trait Foo<T> {
}

pub trait Bar : Send {
    type A;
    type B: Foo<Self::A>;
}

pub fn main() {}
