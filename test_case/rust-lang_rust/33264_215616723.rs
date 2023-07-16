 rust
fn add(&self, vec: &Self) -> Self {
  unsafe {
    let mut ret: Self = std::mem::uninitialized();
    asm!("
      movups $1, %xmm1,
      movups $2, %xmm2,
      addps %xmm1, %xmm2,
      movups %xmm1, $0
      "
      : "=*m"(&mut ret)
      : "*m"(self), "*m"(vec)
      : "xmm1", "xmm2"
  }
}
