
   Compiling playground v0.0.1 (/playground)
error[E0223]: ambiguous associated type
 --> src/main.rs:4:5
  |
4 |     A::B::new();
  |     ^^^^^^^^^ help: use fully-qualified syntax: `<A as Trait>::B`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0223`.
error: could not compile `playground`

To learn more, run the command again with --verbose.

