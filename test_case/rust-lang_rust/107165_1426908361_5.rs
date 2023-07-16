Shell
error[E0603]: tuple struct constructor `Foo` is private
  --> first_lib/src/lib.rs:37:25
   |
37 |         second_lib::Foo(..) => {
   |                     ^^^ private tuple struct constructor
   |
  ::: /.../second_lib/src/lib.rs:25:16
   |
25 | pub struct Foo(pub i64, i64);
   |                ------------ a constructor is private if any of the fields is private
   |
note: the tuple struct constructor `Foo` is defined here
  --> /.../second_lib/src/lib.rs:25:1
   |
25 | pub struct Foo(pub i64, i64);
   | ^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
