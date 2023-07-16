rust
|| {
  let x = "hello".to_string();
  yield;
  let y = "world".to_string();;
  drop(x);
  yield;
  drop(y);
}
