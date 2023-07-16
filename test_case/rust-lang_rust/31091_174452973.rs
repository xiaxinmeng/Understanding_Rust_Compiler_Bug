 Rust
fn main() {
  let v = Some(vec![1, 2, 3]);
  let s = match v {
      Some(ref v) => &v[..],
      None => &[] as &[_],
  };
  println!("{:?}", s);
}
