
error[E0275]: overflow evaluating the requirement `[closure@src/bin/compare_damping.rs:47:13: 47:21]: Fn<(Complex<f64>,)>`
  |
  = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`compare_damping`)
  = note: required for `&[closure@src/bin/compare_damping.rs:47:13: 47:21]` to implement `Fn<(Complex<f64>,)>`
  = note: 128 redundant requirements hidden
  = note: required for `&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&[closure@src/bin/compare_damping.rs:47:13: 47:21]` to implement `Fn<(Complex<f64>,)>`

For more information about this error, try `rustc --explain E0275`.
