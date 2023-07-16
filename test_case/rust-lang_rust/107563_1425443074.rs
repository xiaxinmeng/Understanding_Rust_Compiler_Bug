rust
error[E0252]: the name `Foo` is defined multiple times
  --> issue-107563.rs:13:9
   |
12 | pub use a::*;
   |         ---- previous import of the type `Foo` here
13 | pub use b::*;
   |         ^^^^
   |         |
   |         `Foo` reimported here
   |         you can use `as` to change the binding name of the import
   |
   = note: `Foo` must be defined only once in the type namespace of this module
