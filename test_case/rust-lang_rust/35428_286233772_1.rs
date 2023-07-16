rust
pub trait Iterator {
    ...
    fn is_empty(&self) -> bool
        where Self: IsEmpty
    {
        IsEmpty::is_empty(self)
    }
}
