text
error[E0428]: the name `foo` is defined multiple times
 --> <source>:6:9
  |
5 | #[test] fn foo(){}
  |         -------- previous definition of the value `foo` here
6 | #[test] fn foo(){}
  |         ^^^^^^^^ `foo` redefined here
  |
  = note: `foo` must be defined only once in the value namespace of this module
error: aborting due to previous error
For more information about this error, try `rustc --explain E0428`.
Compiler returned: 1
