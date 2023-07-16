
   Compiling foo v0.1.0 (file:///home/mark/BuildDisk/testing/foo)
error[E0603]: struct `Foo` is private
 --> examples/test.rs:1:31
  |
1 | extern crate foo; fn main() { foo::Foo::bar() }
  |                               ^^^^^^^^^^^^^

error: aborting due to previous error(s)

error: Could not compile `foo`.

To learn more, run the command again with --verbose.
