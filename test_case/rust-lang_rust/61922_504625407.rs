rust
|| {
  let first = [0; 1024];
  yield;
  let second = first;
  yield;
  let _third = second;
  yield;
}
