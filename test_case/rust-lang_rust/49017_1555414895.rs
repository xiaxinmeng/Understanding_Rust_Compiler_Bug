rust
pub trait Encodable {}

pub trait Inner
where
    for<'a> &'a Self: Encodable, {
}

pub trait Outer {
    type Inner: Inner;
}

struct SomeStruct {}

impl<'a> Encodable for SomeStruct 
where
    &'a Self: Encodable, {
}
