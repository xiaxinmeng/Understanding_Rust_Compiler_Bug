 rust
// I have no idea what to call this
pub trait Foo {
    fn next_value(&self) -> Option<Self>;
    fn prev_value(&self) -> Option<Self>;
}

pub trait Countable : Foo + Bounded {
    fn count() -> uint; // or maybe u64?
}
