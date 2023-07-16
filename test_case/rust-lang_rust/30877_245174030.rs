 rust
pub enum Bound<T> {
    Unbound,
    Inclusive(T),
    Exclusive(T),
}

pub trait RangeArgument<T> {
    fn lower(&self) -> Bound<&T>;

    fn upper(&self) -> Bound<&T>;
}
