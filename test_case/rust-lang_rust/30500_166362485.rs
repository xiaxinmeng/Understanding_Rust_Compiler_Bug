 rust
fn f(&self) -> &Self;

fn f_mut(&mut self) -> &mut Self {
    unsafe { transmute(self.f()) }
}
