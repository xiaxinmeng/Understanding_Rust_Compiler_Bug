
fn f_mut_pedantic<'s>(&'s mut self) -> &'s mut Self {
    let captured_self = self as *mut Self;
    unsafe { &mut *((&*captured_self).f() as *const Self as *mut Self) }
}
