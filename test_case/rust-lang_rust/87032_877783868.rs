rust
pub fn check(x: &NotBothZero) {
  if x.0 == 0 && x.1 == 0 { unsafe { unreachable_unchecked(); } }
}
