
warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/main.rs:1:12
  |
1 | #![feature(generic_associated_types)]
  |            ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

error[E0308]: mismatched types
  --> src/main.rs:26:5
   |
26 |     future::<'a, S, _>(f::<'a, S>());
   |     ^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected type `Send`
              found type `Send`

error: aborting due to previous error; 1 warning emitted
