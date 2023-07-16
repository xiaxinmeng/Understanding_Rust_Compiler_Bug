Shell
error[E0603]: unit struct `Foo` is private
  --> first_lib/src/lib.rs:37:25
   |
37 |         second_lib::Foo => {
   |                     ^^^ private unit struct
   |
note: the unit struct `Foo` is defined here
  --> /.../second_lib/src/lib.rs:25:1
   |
25 | pub struct Foo;
   | ^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
