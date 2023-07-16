
cargo +nightly check
    Checking fcg3 v0.1.0 (/home/trevor/try/fcg3)
warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/lib.rs:1:12
  |
1 | #![feature(generic_const_exprs)]
  |            ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information

error[E0391]: cycle detected when type-checking the const argument `<impl at src/lib.rs:11:1: 18:2>::{constant#0}`
  --> src/lib.rs:13:8
   |
13 |     If<{ FRAC <= 8 }>: True,
   |        ^^^^^^^^^^^^^
   |
note: ...which requires trying to unify the generic constants <impl at src/lib.rs:11:1: 18:2>::{constant#0} and <impl at src/lib.rs:11:1: 18:2>::{constant#0}...
  --> src/lib.rs:13:8
   |
13 |     If<{ FRAC <= 8 }>: True,
   |        ^^^^^^^^^^^^^
note: ...which requires building an abstract representation for the const argument <impl at src/lib.rs:11:1: 18:2>::{constant#0}...
  --> src/lib.rs:13:8
   |
13 |     If<{ FRAC <= 8 }>: True,
   |        ^^^^^^^^^^^^^
note: ...which requires building THIR for `<impl at src/lib.rs:11:1: 18:2>::{constant#0}`...
  --> src/lib.rs:13:8
   |
13 |     If<{ FRAC <= 8 }>: True,
   |        ^^^^^^^^^^^^^
   = note: ...which again requires type-checking the const argument `<impl at src/lib.rs:11:1: 18:2>::{constant#0}`, completing the cycle
note: cycle used when building MIR for `<impl at src/lib.rs:11:1: 18:2>::{constant#0}`
  --> src/lib.rs:13:8
   |
13 |     If<{ FRAC <= 8 }>: True,
   |        ^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0391`.
warning: `fcg3` (lib) generated 1 warning
error: could not compile `fcg3` due to previous error; 1 warning emitted
