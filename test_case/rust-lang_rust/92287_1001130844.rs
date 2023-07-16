plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0658]: use of unstable library feature 'slice_remainder'
    --> library/core/tests/slice.rs:2244:20
     |
2244 |   assert_eq!(split.remainder(), &[3,4,5,6]);
     |
     |
     = help: add `#![feature(slice_remainder)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
