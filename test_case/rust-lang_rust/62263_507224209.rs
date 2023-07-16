rust
pub type Foo = dyn Bar<B = Self::B>;

pub trait Bar {
    type B;
}
