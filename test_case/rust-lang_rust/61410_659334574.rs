
warning: the feature `const_generics` is incomplete and may not be safe to use and/or cause compiler crashes
  --> $DIR/issue-61410.rs:1:12
   |
LL | #![feature(const_generics)]
   |            ^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #44580 <https://github.com/rust-lang/rust/issues/44580> for more information

error[E0632]: cannot provide explicit generic arguments when `impl Trait` is used in argument position

error[E0747]: constant provided when a type was expected
  --> $DIR/issue-61410.rs:20:20
   |
LL |     assert_eq!(f::<4usize>(Usizable), 20usize);
   |                    ^^^^^^
   |
   = note: type arguments must be provided before constant arguments
   = help: reorder the arguments: types, then consts: `<impl Usizer, N>`

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0747`.
