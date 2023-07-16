rust
use std::cell::Cell;
type SVec<T: Send> = Vec<T>;
fn foo(_x: SVec<&Cell<i32>>) {}
pub fn bar() {
  foo(Vec::new());
}
