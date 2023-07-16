rust
warning: the feature `const_generics` is incomplete and may cause the compiler to crash
  --> $DIR/issue-68977.rs:1:12
   |
LL | #![feature(const_generics)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default

error: constant expression depends on a generic parameter
  --> $DIR/issue-68977.rs:31:44
   |
LL |     FxpStorageHelper<INT_BITS, FRAC_BITS>: FxpStorage,
   |                                            ^^^^^^^^^^
   |
   = note: this may fail depending on what value the parameter takes

error: aborting due to previous error

