text
   Compiling playground v0.0.1 (/playground)
warning: trait objects without an explicit `dyn` are deprecated
 --> src/main.rs:4:15
  |
4 | let x: u32 = <Default>::default();
  |               ^^^^^^^ help: use `dyn`: `dyn Default`
  |
  = note: #[warn(bare_trait_objects)] on by default

error[E0277]: the size for values of type `dyn std::default::Default` cannot be known at compilation time
 --> src/main.rs:4:14
  |
4 | let x: u32 = <Default>::default();
  |              ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `dyn std::default::Default`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: required by `std::default::Default::default`

error[E0038]: the trait `std::default::Default` cannot be made into an object
 --> src/main.rs:4:15
  |
4 | let x: u32 = <Default>::default();
  |               ^^^^^^^ the trait `std::default::Default` cannot be made into an object
  |
  = note: the trait cannot require that `Self : Sized`

error[E0308]: mismatched types
 --> src/main.rs:4:14
  |
4 | let x: u32 = <Default>::default();
  |              ^^^^^^^^^^^^^^^^^^^^ expected u32, found trait std::default::Default
  |
  = note: expected type `u32`
             found type `dyn std::default::Default`

error[E0038]: the trait `std::default::Default` cannot be made into an object
 --> src/main.rs:4:14
  |
4 | let x: u32 = <Default>::default();
  |              ^^^^^^^^^^^^^^^^^^^^ the trait `std::default::Default` cannot be made into an object
  |
  = note: the trait cannot require that `Self : Sized`

error: aborting due to 4 previous errors
