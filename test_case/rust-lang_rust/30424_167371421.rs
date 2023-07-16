 Rust
fn f_mut_pedantic<'s>(&'s mut self) -> &'s mut Self {
    let captured_self = self as *mut Self;
    unsafe { &mut *(Self::f(&*captured_self) as *const Self as *mut Self) }
}
