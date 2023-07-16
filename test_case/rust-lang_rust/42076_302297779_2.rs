
error[E0428]: the name `Foo` is defined twice in this module
 --> example.rs:2:1
  |
1 | trait Foo { }
  | ------------- previous definition of the trait `Foo` here
2 | struct Foo { }
  | ^^^^^^^^^^^^^^ `Foo` redefined here

error: aborting due to previous error
