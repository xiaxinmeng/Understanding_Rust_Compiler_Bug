 Rust
pub trait Trait {
    type FooT;
    type BarT;
}

pub type Object = Option<Box<Trait<FooT=(),BarT=()>>>;
