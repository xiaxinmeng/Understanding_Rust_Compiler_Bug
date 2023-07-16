 rust
use std::mem;

type Foo = extern "C" fn() -> i32;

fn main() {
  // Get a pointer from a C library.
  let a: Foo = unsafe { mem::transmute(15usize) };

  let b = a;
  let c = a; // Copy works fine

  let d = a.clone(); // But Clone (which is a prerequisite for Copy) doesn't?
}
