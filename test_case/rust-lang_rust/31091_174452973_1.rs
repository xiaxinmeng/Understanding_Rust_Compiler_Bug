 Rust
fn main() {
  let v = Some(vec![1, 2, 3]);
  let s: &[_] = match v {
      Some(ref v) => &v[..],
      None => &[],
  };
  println!("{:?}", s);
}
