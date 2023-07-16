rust
pub trait Trait {
    type Assoc;
}

pub fn func() {
    fn _inner<U: Trait>() -> impl Fn(U::Assoc) { |_| unimplemented!() }
}
