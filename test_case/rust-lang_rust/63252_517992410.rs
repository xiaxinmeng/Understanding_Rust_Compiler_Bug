rust
trait CloneRef : Clone {
    fn clone_ref(&self) -> Self { self.clone() }
}

impl<'a, T: ?Sized> CloneRef for &'a T {}
impl<T: ?Sized> CloneRef for std::rc::Rc<T> {}
impl<T: ?Sized> CloneRef for std::rc::Weak<T> {}
impl<T: ?Sized> CloneRef for std::sync::Arc<T> {}
impl<T: ?Sized> CloneRef for std::sync::Weak<T> {}
