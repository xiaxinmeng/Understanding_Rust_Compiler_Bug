
error[E0223]: ambiguous associated type
 --> <source>:3:5
  |
3 |     S::A::<f> {}
  |     ^^^^^^^^^ help: use fully-qualified syntax: `<S as Trait>::A`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0223`.
