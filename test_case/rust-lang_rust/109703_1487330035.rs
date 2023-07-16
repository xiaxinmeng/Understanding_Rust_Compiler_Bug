rust
let x = x;

/*
error[E0425]: cannot find value `x` in this scope
 --> ...
  |
2 |     let x = x;
  |             ^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `...` due to previous error
*/
