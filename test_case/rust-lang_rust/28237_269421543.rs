rust
pub trait RangeArgument<T> {
    fn start(&self) -> Option<&T> { ... }
    fn end(&self) -> Option<&T> { ... }
}
