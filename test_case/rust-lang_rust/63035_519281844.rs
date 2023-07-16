rust
fn test(b: bool, x: fn() -> !) {
  yield;
  if b { return; }
  let val = x();
  yield;
  drop(val);
}
