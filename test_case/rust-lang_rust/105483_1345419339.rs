rust
trait Trait {
    fn test(&self) -> Option<&'_ Self>;
}
impl<T> Trait for T {
    fn test(&self) -> Option<&'static Self> {
        None
    }
}
