
error[E0284]: type annotations needed: cannot satisfy `<f64 as std::ops::Add<_>>::Output == f64`
 --> src/main.rs:2:7
  |
2 |     a + (b - a) * c.into() / d.into()
  |       ^ cannot satisfy `<f64 as std::ops::Add<_>>::Output == f64`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0284`.
