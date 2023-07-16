 rust
pub type BoxedFn = Box<Fn(&Trait)>;

pub trait Trait {}

pub fn new() -> Option<(BoxedFn, BoxedFn)> {
    None
}
