rust
#![crate_type="rlib"]
pub fn f(l: &Vec<u64>) -> u64 {
  let mut sum = 0;
  for i in 0..l.len() {
    sum += l[i]
  }
  sum
}
