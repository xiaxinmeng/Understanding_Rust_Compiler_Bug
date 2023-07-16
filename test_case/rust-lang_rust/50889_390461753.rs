
[01:30:32] error[E0605]: non-primitive cast: `rustc_target::abi::Size` as `usize`
[01:30:32]    --> tools/clippy/clippy_lints/src/consts.rs:435:34
[01:30:32]     |
[01:30:32] 435 |                     let offset = ptr.offset as usize;
[01:30:32]     |                                  ^^^^^^^^^^^^^^^^^^^
[01:30:32]     |
[01:30:32]     = note: an `as` expression can only be used to convert between primitive types. Consider using the `From` trait
[01:30:32]
[01:30:36] error: aborting due to previous error
[01:30:36]
[01:30:36] For more information about this error, try `rustc --explain E0605`.
