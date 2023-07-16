 rust
fn deref_mut<'a>(&'a mut self) -> &'a mut T {
    unsafe { &mut *self._parent.value.get() }
}
