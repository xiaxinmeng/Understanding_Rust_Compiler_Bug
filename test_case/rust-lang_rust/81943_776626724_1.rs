
  error[E0308]: mismatched types
   --> src/main.rs:2:49
    |
  2 |     let sum: i32 = [1, 2, 3].iter().inspect(|n| dbg!(n)).sum();
    |                                                 ^^^^^^^
    |                                                 |
    |                                                 expected `()`, found `&&{integer}`
    |                                                 expected this to be `()`
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
  