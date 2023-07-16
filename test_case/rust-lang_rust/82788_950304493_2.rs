text
error[E0603]: unit struct `Foo` is private
   --> src/lib.rs:117:20
    |
4   | let lib::Foo = lib::Foo::default();
    |          ^^^ private unit struct
    |
note: the unit struct `Foo` is defined here
   --> path
    |
121 | pub struct Foo;
    | ^^^^^^^^^^^^^^^
