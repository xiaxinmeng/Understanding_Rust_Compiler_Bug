rust
warning: the feature `const_generics` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/lib.rs:1:12
  |
1 | #![feature(const_generics)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #44580 <https://github.com/rust-lang/rust/issues/44580> for more information

error: constant expression depends on a generic parameter
 --> src/lib.rs:8:32
  |
8 |     Condition<{ LHS <= RHS }>: True
  |                                ^^^^
  |
  = note: this may fail depending on what value the parameter takes

error: constant expression depends on a generic parameter
  --> src/lib.rs:18:42
   |
18 |     IsLessOrEqual<{ 8 - I }, { 8 - J }>: True,
   |                                          ^^^^
   |
   = note: this may fail depending on what value the parameter takes

error: aborting due to 2 previous errors; 1 warning emitted
