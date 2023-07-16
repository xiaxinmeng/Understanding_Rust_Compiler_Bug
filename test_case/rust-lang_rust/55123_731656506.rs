rust
pub enum Void {}

pub fn foo(x: Void) {
  let _ = (); // Frankly this also could be warned as unreachable.
  match x {};
  let _ = (); // This could be warned as unreachable, but isn't.
}
