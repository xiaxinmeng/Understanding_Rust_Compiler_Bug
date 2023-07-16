
impl Window {
  pub fn DoSomething(&self) {
    unsafe { ffi::SDL_DoSomething(self.handle); }
  }
}
