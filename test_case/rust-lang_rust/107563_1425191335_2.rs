rust
error[E0252]: the name `foo` is defined multiple times
  --> src/lib.rs:10:9
   |
9  | pub use a::foo;
   |         ------ previous import of the value `foo` here
10 | pub use b::foo;
   |         ^^^^^^ `foo` reimported here
   |
   = note: `foo` must be defined only once in the value namespace of this module
help: you can use `as` to change the binding name of the import
   |
10 | pub use b::foo as other_foo;
   |         ~~~~~~~~~~~~~~~~~~~

For more information about this error, try `rustc --explain E0252`.
