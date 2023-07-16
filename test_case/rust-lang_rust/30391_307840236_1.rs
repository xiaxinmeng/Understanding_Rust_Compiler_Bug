
error[E0599]: no method named `foo` found for type `Foo` in the current scope
 --> test.rs:9:9
  |
9 |     Foo.foo();
  |         ^^^
  |
  = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: candidate #1 is defined in an impl for the type `Foo`
 --> test.rs:4:5
  |
4 | /     fn foo() {
5 | |     }
  | |_____^

error: aborting due to previous error(s)
