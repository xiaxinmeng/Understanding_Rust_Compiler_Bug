plain
    Checking clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
  --> src/tools/clippy/src/driver.rs:86:52
   |
86 |         config.opts.debugging_opts.mir_opt_level = 0;
   |                                                    |
   |                                                    expected enum `Option`, found integer
   |                                                    expected enum `Option`, found integer
   |                                                    help: try using a variant of the expected enum: `Some(0)`
   = note: expected enum `Option<usize>`
   = note: expected enum `Option<usize>`
              found type `{integer}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy`
