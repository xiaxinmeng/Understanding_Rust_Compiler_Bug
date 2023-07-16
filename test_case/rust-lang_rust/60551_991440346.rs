rust
pub trait Foo {
    const COUNT: usize;
    const NAMES: [&'static str; Self::COUNT];
}

pub struct Bar<T: Foo> {
    name_lookup: [resource::UniformLocation; T::COUNT]
}
