
error[E0463]: can't find crate for `foo`
 --> src\main.rs:1:1
  |
1 | extern crate foo;
  | ^^^^^^^^^^^^^^^^^ can't find crate

error[E0433]: failed to resolve: use of undeclared crate or module `bar`
 --> src\main.rs:6:5
  |
6 |     bar::buzz();
  |     ^^^ use of undeclared crate or module `bar`

Some errors have detailed explanations: E0433, E0463.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `issue-90702-fix` due to 2 previous errors
