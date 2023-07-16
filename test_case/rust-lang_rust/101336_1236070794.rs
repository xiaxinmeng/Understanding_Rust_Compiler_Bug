rust
pub fn evil(p: *const dyn Debug) {
  let parts: [usize; 2] = unsafe { mem::transmute(p) };
  if parts[1] == 0 {
    // A compiler-allocated vtable can never be at address 0.
    unsafe { hint::unreachable_unchecked() };
  }
}
