rust
fn caller() -> Result<(), E> {
  let mut x = vec![1, 2, 3];
  println!("{:?}", x);
  x = callee()?;
  println!("{:?}", x);
}
