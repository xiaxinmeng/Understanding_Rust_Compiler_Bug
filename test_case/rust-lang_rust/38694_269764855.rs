rust
pub fn f3() -> bool {
  let v = [1,2,3];
  v.iter().any(|&x| x == 2)
}

pub fn f4() -> bool {
  [1,2,3].iter().any(|&x| x == 2)
}
