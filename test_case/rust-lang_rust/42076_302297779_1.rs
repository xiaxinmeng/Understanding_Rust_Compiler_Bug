
error[E0428]: a trait named `Foo` has already been defined in this module
 --> example.rs:2:1
  |
1 | trait Foo { }
  | ------------- previous definition of `Foo` here
2 | struct Foo { }
  | ^^^^^^^^^^^^^^ `Foo` already defined

error: aborting due to previous error
