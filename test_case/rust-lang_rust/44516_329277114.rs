Rust
struct Ref<'a>(&'a ());
fn foo(x:fn(Ref, Ref), y: Vec<&u8>, z: &u8) {
  y.push(z);
}
