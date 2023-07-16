 rust
fn deref_mut<'a>(&'a mut self) -> &'static mut T {
    unsafe { &mut *self._parent.value.get() }
}
