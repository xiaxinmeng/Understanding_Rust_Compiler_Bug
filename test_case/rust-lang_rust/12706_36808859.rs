 rust
impl<T: DeepClone> DeepClone for Rc<T> {
    fn deep_clone(&self) -> Rc<T> {
        Rc::new(self.borrow().deep_clone())
    }
}
