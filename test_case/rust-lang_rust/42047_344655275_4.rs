rust
fn caller_abi(err: &uninit E) -> bool {
  let mut x = vec![1, 2, 3];
  println!("{:?}", x);
  
  drop(x);  // Drop the old value *before* the call
  if !callee_abi(&uninit x, err) {
    return false;
  }

  println!("{:?}", x);

  drop(x);
  return true;
}
