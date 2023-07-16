rust
impl Box<MaybeUninit<T>> { // not sure if this works but you get the idea
  pub fn uninit() -> Self { Box::new(MaybeUninit::new()) } // with some hack to avoid a stack allocation
  // also: zeroed, new

  unsafe pub fn assume_init(self) -> Box<T> { mem::transmute(self) }

  pub fn write(&mut self, value: T) -> &mut Box<T> {
    MaybeUninit::write(&mut *self, value);
    unsafe { mem::transmute(self) }
  }
}
