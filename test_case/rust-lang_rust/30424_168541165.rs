 rust
fn f_mut_unsure<'s>(&'s mut self) -> &'s mut Self {
    let ret = unsafe { transmute::<&Self, &'s mut Self>(self.f()) };
    ret // β-expansion should have no effect
}
fn f_mut_should_work<'s>(&'s mut self) -> &'s mut Self {
    let ret = unsafe { &mut *(self.f() as *const Self as *mut Self) };
    ret // β-expansion should have no effect
}
