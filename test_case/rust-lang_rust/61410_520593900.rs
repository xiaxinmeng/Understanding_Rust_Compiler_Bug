
error[E0632]: cannot provide explicit type parameters when `impl Trait` is used in argument position.
  --> src/main.rs:23:16
   |
23 |     assert_eq!(f::<4usize>(Usizable), 20usize);
   |                ^^^^^^^^^^^
