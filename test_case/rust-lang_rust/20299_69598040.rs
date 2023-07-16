 rust
pub trait Trait {
    type Out;
}

pub fn foo<T>(_: T) where T: Trait<Out=i32> { }
