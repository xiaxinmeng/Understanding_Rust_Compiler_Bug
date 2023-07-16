sh
cargo run
   Compiling test_47450 v0.1.0 (/home/michal/projects/tests/test_47450)
error[E0432]: unresolved import `crate::bar::Foo`
 --> src/main.rs:8:5
  |
8 | use crate::bar::Foo;
  |     ^^^^^^^^^^^^^^^ no `Foo` in `bar`
  |
help: consider importing this struct instead
  |
8 | use crate::foo::Foo;
  |     ~~~~~~~~~~~~~~~

For more information about this error, try `rustc --explain E0432`.
error: could not compile `test_47450` (bin "test_47450") due to previous error
