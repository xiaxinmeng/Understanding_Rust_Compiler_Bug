 Rust
fn f_mut(&mut self) -> &mut Self {
    unsafe { &mut *(self.f() as *const Self as *mut Self) }
}
