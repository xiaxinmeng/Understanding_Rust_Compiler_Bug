none
error[E0004]: non-exhaustive patterns: type `(Infallible, Infallible)` is non-empty
 --> src/lib.rs:9:11
  |
9 |     match (a, b) {}
  |           ^^^^^^
  |
  = note: the matched value is of type `(Infallible, Infallible)`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown
  |
9 ~     match (a, b) {
10+         _ => todo!(),
11+     }
  |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `playground` due to previous error
