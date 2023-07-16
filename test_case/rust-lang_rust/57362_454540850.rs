
error[E0308]: mismatched types
  --> src/main.rs:13:7
   |
13 |     a.f();
   |       ^ one type is more general than the other
   |
   = note: expected `Trait` to be implemented for `for<'r> &'r u8`
              found `Trait` implemented for `&u8`
