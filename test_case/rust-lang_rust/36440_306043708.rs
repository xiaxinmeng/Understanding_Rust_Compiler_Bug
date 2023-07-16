
error: `Foo::Bar` is being called, but it is not a function
 --> test.rs:4:12
  |
4 | fn foo() { Foo::Bar("World") }
  |            ^^^^^^^^^^^^^^^^^
  |
  = help: did you mean to write `Foo::Bar`?
note: defined here
 --> test.rs:1:12
  |
1 | enum Foo { Bar }
  |            ^^^

error: aborting due to previous error(s)
