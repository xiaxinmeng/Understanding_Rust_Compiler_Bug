rust
const fn totally_safe_fn(x: &i32) {
  unsafe { transmute::<&i32, usize>(x) / 2 }
}
