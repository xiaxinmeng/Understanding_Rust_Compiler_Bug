rust
pub trait DispatchFromDyn<T> { }

impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Rc<U>> for Rc<T> {}
