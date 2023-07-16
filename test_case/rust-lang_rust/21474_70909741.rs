 rust
pub use inner::*;

mod inner {
    impl super::Blah for super::What { }
}

pub trait Blah { }

pub struct What;
