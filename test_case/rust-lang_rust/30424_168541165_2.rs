 rust
fn unsafe cast_mut_away<T>(x: &mut T) -> &T { transmute(x) } // make sure transmute does not change the lifetime
fn f_mut_dunno<'s>(&'s mut self) -> &'s mut Self {
    let local = &mut *self; // just to get a local re-borrow with a lifetime shorter than the function body
    unsafe { &mut *(cast_mut_away(local).f() as *const Self as *mut Self) }
}
