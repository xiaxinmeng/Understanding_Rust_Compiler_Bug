rust
error[E0428]: the name `foo_` is defined multiple times
 --> src/lib.rs:2:1
  |
1 | pub fn foo_() {}
  | ------------- previous definition of the value `foo_` here
2 | pub fn foo_() {}
  | ^^^^^^^^^^^^^ `foo_` redefined here
  |
  = note: `foo_` must be defined only once in the value namespace of this module

error[E0252]: the name `foo` is defined multiple times
 --> src/lib.rs:4:5
  |
3 | use foo_ as foo;
  |     ----------- previous import of the value `foo` here
4 | use foo_ as foo;
  |     ^^^^^^^^^^^ `foo` reimported here
  |
  = note: `foo` must be defined only once in the value namespace of this module
help: You can use `as` to change the binding name of the import
  |
4 | use foo_ as foo as other_foo;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
