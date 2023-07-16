rust
impl<T: 'static + Send + Clone + Display> Value for T {
    fn box_clone(&self) -> Box<dyn Value> {
        Box::new((*self).clone())  // THIS LINE is main.rs:26
    }
}
