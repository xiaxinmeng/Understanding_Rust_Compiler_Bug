rust
// Rust definitions
trait CloneAs<T: ?Sized> {
    fn clone_as(&self) -> T;
}
impl<T:  Trait + Clone> CloneAs<dyn Trait> for T {
    fn clone_as(&self) -> dyn Trait { self.clone() }
}
trait Trait: CloneAs<dyn Trait> {}
