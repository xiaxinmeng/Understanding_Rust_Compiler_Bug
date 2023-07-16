rust
fn caller_abi(err: &uninit E) -> bool {
  let mut x = vec![1, 2, 3];
  println!("{:?}", x);
  
  let temp_ok;
  if !callee_abi(&uninit temp_ok, err) {
    return false;
  }

  drop(x);  //  Drop the old value *after* the call!
  x = temp_ok;
  
  println!("{:?}", x);

  drop(x);
  return true;
}
