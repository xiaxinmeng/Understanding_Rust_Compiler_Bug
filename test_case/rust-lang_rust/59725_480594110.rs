rust
trait Wake {
   fn wake(self: Arc<Self>);

   // default impl overrideable if you dont need ownership
   fn wake_by_ref(self: &Arc<Self>) {
      self.clone().wake()
   }
}

impl Waker {
    pub fn new(wake: Arc<dyn Wake>) -> Waker { ... }
}
