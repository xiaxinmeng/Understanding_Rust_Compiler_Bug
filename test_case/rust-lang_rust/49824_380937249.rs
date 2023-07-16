rust
fn foo(mut x: Vec<&'a u32>, y: &'b u32) {
  let c = || {
    x.push(y);
  };
  c();
}
