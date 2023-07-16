rust
trait Trait {
    fn f(&self);
}

impl<T> Trait for T where T: ?Sized + Trait {
    fn f(&self) {
        let _ = self as &Trait;
    }
}
