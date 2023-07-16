
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
 --> file.rs:6:20
  |
6 |     Box::new(items.iter())
  |                    ^^^^
  |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the function body at 5:6...
 --> file.rs:5:6
  |
5 | fn a<'a, T>(items: &'a [T]) -> Box<impl Iterator> {
  |      ^^
note: ...so that reference does not outlive borrowed content
 --> file.rs:6:14
  |
6 |     Box::new(items.iter())
  |              ^^^^^
  = note: but, the lifetime must be valid for the static lifetime...
note: ...so that return value is valid for the call
 --> file.rs:5:36
  |
5 | fn a<'a, T>(items: &'a [T]) -> Box<impl Iterator> {
  |                                    ^^^^^^^^^^^^^
