 rust
impl<T> Rc<T> {
    pub fn downgrade(&mut self) { ... }
    pub fn get(&mut self) -> &'a T { ... }
}
