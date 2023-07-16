
fn foo(x: &mut T) {
  let y = x as *mut _; // reborrows x for the shortest possible lifetime, which is this single statement
  let z = x as *mut _; // does the same again
}
