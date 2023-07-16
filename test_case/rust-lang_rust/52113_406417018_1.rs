rust
fn foo<'a, 'b>(x: &'a u32) {
  let closure = move || {
    let v: &'b u32 = x; // should be an error...
  };
  ...
}
