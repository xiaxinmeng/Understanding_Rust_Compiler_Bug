
error[E0311]: the parameter type `R` may not live long enough
 --> test.rs:3:9
  |
3 |         (|| u)();
  |         ^^^^^^^^
  |
  = help: consider adding an explicit lifetime bound for `R`
note: the parameter type `R` must be valid for the anonymous lifetime #1 defined on the body at 2:16...
 --> test.rs:2:17
  |
2 |       |u: &mut R| {
  |  _________________^
3 | |         (|| u)();
4 | |     };
  | |_____^
note: ...so that the reference type `&mut R` does not outlive the data it points at
 --> test.rs:3:9
  |
3 |         (|| u)();
  |         ^^^^^^^^

error: aborting due to previous error
