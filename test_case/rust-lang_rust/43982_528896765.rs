rust
pub trait Trait {
    type Assoc;
}

pub fn func() {
    fn _inner<U: Trait>(_: U::Assoc) {}
}

