 rust
fn foo(x: u32) { }
fn main() {
  let f = foo;
  { f } (22)
}
