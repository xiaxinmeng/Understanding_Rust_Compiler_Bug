 rust
fn quux() {
  {
    hi();
  } // this is fine

  let x = {
    foo();
  }; // this causes the warning

  bar({
    foo();
  }); // this causes the warning too
}
