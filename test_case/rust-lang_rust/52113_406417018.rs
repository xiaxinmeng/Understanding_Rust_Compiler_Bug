rust
fn foo<'a>(x: &'a u32, mut y: Vec<&'a u32>) {
  let closure = move || y.push(x);
  ...
}
