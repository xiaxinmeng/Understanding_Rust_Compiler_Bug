rust
impl<T> Mutex<T> {
  pub fn with<R, F: Fn(&mut T) -> R>(&self, f: F) -> Result<R, PoisonErr> {
    self.lock().map_ok(f)
  }
}

/// ...

foo.with(|locked| locked.bar()).unwrap();
