rust
fn main() {
  // let mut s = State::<T1>::new();
  // s.test_cb(1);
  let cb: Option<Box<dyn Callback<T1>>> = None;
  (cb.unwrap())(1);
}
