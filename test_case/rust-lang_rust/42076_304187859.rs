
error[E0428]: the name `Foo` is defined multiple times
 --> example.rs:2:1
  |
1 | trait Foo { }
  | ------------- previous definition of the trait `Foo` here
2 | struct Foo { }
  | ^^^^^^^^^^^^^^ `Foo` redefined here
  = note: `Foo` must be defined only once in the type namespace of this module

error: aborting due to previous error
