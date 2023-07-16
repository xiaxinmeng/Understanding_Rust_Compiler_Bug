
warning: the feature `inherent_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
 --> more.rs:1:12
  |
1 | #![feature(inherent_associated_types)]
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #8995 <https://github.com/rust-lang/rust/issues/8995> for more information
  = note: `#[warn(incomplete_features)]` on by default

error[E0601]: `main` function not found in crate `more`
 --> more.rs:9:2
  |
9 | }
  |  ^ consider adding a `main` function to `more.rs`

error: overflow evaluating associated type `Carrier<'b>::Focus<i32>`
 --> more.rs:8:25
  |
8 |     pub type Focus<T> = &'a mut User;
  |                         ^^^^^^^^^^^^

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0601`.
