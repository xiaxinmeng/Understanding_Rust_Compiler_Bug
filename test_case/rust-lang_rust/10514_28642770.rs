 rust
impl<T: Freeze> Rc<Mut<T>> {
    fn from_mut(val: Mut<T>) -> Rc<Mut<T>> { ... }
}
