
fn foo(x: &mut T) {
  let y = { let x' = x; x' as *mut _ };
  let z = { let x' = x; x' as *mut _ }
}
