rust
pub enum Void {}

pub fn foo(x: Void) {
  match x {
    _ => {} // This arm shouldn't be permitted.
  };
  let _xyz = (); // This should be warned as unreachable, but isn't.
}
