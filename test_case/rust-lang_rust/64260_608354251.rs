rust
trait TakeIf {
    fn take_if(self, b: bool) -> Option<Self> where Self: Sized;
}

impl<T> TakeIf for T {
    fn take_if(self, b: bool) -> Option<Self> where Self: Sized {
        b.then_some(self)
    }
}
