rust
fn foo(mut x: T) {
  // ... do stuff with x that doesn't transfer ownership
  T::drop(&mut x); // if T implements Drop
  mem::forget(x);
}
