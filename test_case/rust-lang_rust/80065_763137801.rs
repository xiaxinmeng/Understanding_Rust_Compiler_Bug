
error[E0658]: default values for const generic parameters are experimental
  --> $DIR/default_trait_param.rs:1:28
   |
LL | trait Foo<const KIND: bool = true> {}
<<<<<<< HEAD
   |                            ^^^^^^
   |
   = note: see issue #44580 <https://github.com/rust-lang/rust/issues/44580> for more information
   = help: add `#![feature(const_generics_defaults)]` to the crate attributes to enable
=======
   |                       ---- ^ expected one of 7 possible tokens
   |                       |
   |                       maybe try to close unmatched angle bracket
>>>>>>> d228142f35f (add and update tests)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
