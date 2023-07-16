
error[E0658]: const trait impls are experimental
 --> src/lib.rs:5:24
  |
5 | const fn foo() -> impl ~const FnMut(usize, usize) -> NeverShortCircuit {
  |                        ^^^^^^
  |
  = note: see issue #67792 <https://github.com/rust-lang/rust/issues/67792> for more information

error[E0658]: const closures are experimental
 --> src/lib.rs:6:5
  |
6 |     const move |a, b| NeverShortCircuit(f(a, b))
  |     ^^^^^
  |
  = note: see issue #106003 <https://github.com/rust-lang/rust/issues/106003> for more information
