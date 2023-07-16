Rust
#![crate_type="rlib"]
pub fn f(l: &usize) -> u64 {
  let mut sum = 0;
  let len_1 = *l;
  let mut i = 0;
  while i < len_1 {
    let len_2 = *l;
    assert!(i < len_2);
    i += 1;
  }
  sum
}
