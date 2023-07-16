rust
error[E0428]: the name `foo` is defined multiple times
 --> src/lib.rs:2:1
  |
1 | pub fn foo() {}
  | ------------ previous definition of the value `foo` here
2 | pub fn foo() {}
  | ^^^^^^^^^^^^ `foo` redefined here
  |
  = note: `foo` must be defined only once in the value namespace of this module`
