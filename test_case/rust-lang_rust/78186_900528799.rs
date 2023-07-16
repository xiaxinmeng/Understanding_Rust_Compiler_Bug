rust
impl<T> ReentrantMutex<T> {
  pub fn new_boxed() -> Pin<Box<Self>> { unsafe {
    let mut m = Box::pin(Self::new());
    m.as_mut().init();
    m
  }
}
