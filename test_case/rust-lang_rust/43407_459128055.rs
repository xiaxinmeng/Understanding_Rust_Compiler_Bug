
error[E0506]: cannot assign to `x` because it is borrowed
 --> src/main.rs:6:9
  |
3 |     if let Some(&ref p) = x.as_ref() {
  |                           - borrow of `x` occurs here
...
6 |         x = Some(2); // error: x is still borrowed
  |         ^^^^^^^^^^^ assignment to borrowed `x` occurs here
