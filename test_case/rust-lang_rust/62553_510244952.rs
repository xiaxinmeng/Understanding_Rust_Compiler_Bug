rust
fn into_inner(self) -> Box<[u8]> {
    let this = mem::ManuallyDrop::new(self);
    unsafe {
        ptr::read(&this.inner)
    }
}
