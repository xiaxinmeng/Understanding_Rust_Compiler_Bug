 rust
pub struct Privatizer<T> { t: marker::InvariantType<T> }

pub trait InternalImplsOnly {
    fn force_internal() -> Privatizer<T>;
}

pub trait Foo: InternalImplsOnly {}
