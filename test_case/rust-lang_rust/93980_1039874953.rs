
error[E0070]: invalid left-hand side of assignment
 --> t.rs:3:27
  |
3 |     x.last_mut().unwrap() = ();
  |     --------------------- ^
  |     |
  |     cannot assign to this expression

error[E0308]: mismatched types
 --> t.rs:3:29
  |
3 |     x.last_mut().unwrap() = ();
  |     ---------------------   ^^ expected `&mut ()`, found `()`
  |     |
  |     expected due to the type of this binding
  |
help: consider dereferencing here to assign to the mutable borrowed piece of memory
  |
3 |     *x.last_mut().unwrap() = ();
  |     +

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0070, E0308.
For more information about an error, try `rustc --explain E0070`.
