 rust
fn f_mut_bad<'s>(&'s mut self) -> &'s mut Self {
    let ret = unsafe { transmute::<&'s Self, &'s mut Self>(self.f()) };
    ret // β-expansion should have no effect
}
