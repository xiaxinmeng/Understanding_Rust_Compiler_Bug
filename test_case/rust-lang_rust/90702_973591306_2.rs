
error[E0463]: can't find crate for `foo`
 --> src\main.rs:1:1
  |
1 | extern crate foo;
  | ^^^^^^^^^^^^^^^^^ can't find crate

error[E0463]: can't find crate for `bar`
 --> src\main.rs:2:1
  |
2 | extern crate bar;
  | ^^^^^^^^^^^^^^^^^ can't find crate

error[E0432]: unresolved import `boo`
 --> src\main.rs:5:5
  |
5 | use boo::buzz;
  |     ^^^ use of undeclared crate or module `boo`

error[E0433]: failed to resolve: use of undeclared crate or module `boo`
 --> src\main.rs:9:5
  |
9 |     boo::buzz();
  |     ^^^ use of undeclared crate or module `boo`

error[E0433]: failed to resolve: use of undeclared crate or module `baz`
  --> src\main.rs:11:5
   |
11 |     baz::buzz();
   |     ^^^ use of undeclared crate or module `baz`

Some errors have detailed explanations: E0432, E0433, E0463.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `issue-90702-fix` due to 5 previous errors
