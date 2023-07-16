
error: no method named `sqrt` found for type `{float}` in the current scope
 --> src/main.rs:2:21
  |
2 |         let _ = 0.5.sqrt();
  |                     ^^^^
  |
  = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
  = help: candidate #1: `use rand::FloatMath;`

error: aborting due to previous error

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
