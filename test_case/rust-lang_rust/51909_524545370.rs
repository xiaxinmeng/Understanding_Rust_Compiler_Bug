rust
const fn mk_int() -> usize {
  let v = 42;
  &v as *const _ as usize
}
