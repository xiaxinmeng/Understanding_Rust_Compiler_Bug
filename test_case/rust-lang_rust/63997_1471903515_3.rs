
error: expected identifier, found keyword `fn`
 --> src/lib.rs:2:24
  |
2 | const fn foo(f: ~const fn() -> i32) -> i32 {
  |                        ^^
  |
help: use `Fn` to refer to the trait
  |
2 | const fn foo(f: ~const Fn() -> i32) -> i32 {
  |                        ~~

error: `~const` is not allowed here
 --> src/lib.rs:2:17
  |
2 | const fn foo(f: ~const fn() -> i32) -> i32 {
  |                 ^^^^^^^^^^^^^^^^^^
  |
  = note: trait objects cannot have `~const` trait bounds

error[E0782]: trait objects must include the `dyn` keyword
 --> src/lib.rs:2:17
  |
2 | const fn foo(f: ~const fn() -> i32) -> i32 {
  |                 ^^^^^^^^^^^^^^^^^^
  |
help: add `dyn` keyword before this trait
  |
2 | const fn foo(f: dyn ~const fn() -> i32) -> i32 {
  |                 +++

For more information about this error, try `rustc --explain E0782`.
error: could not compile `playground` (lib) due to 3 previous errors
