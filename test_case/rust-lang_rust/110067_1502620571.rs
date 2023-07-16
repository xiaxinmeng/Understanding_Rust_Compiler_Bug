
error[[E0658]](https://doc.rust-lang.org/stable/error_codes/E0658.html): const closures are experimental
 --> src/lib.rs:2:13
  |
2 |     let _ = const move |a, b| NeverShortCircuit(f(a, b));
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: [see issue #106003 <https://github.com/rust-lang/rust/issues/106003>](https://github.com/rust-lang/rust/issues/106003) for more information
