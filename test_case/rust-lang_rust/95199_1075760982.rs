plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0433]: failed to resolve: use of undeclared crate or module `ptr`
   --> library/std/src/backtrace.rs:496:31
    |
496 |             RawFrame::Fake => ptr::invalid_mut(1),
    |                               ^^^ use of undeclared crate or module `ptr`
For more information about this error, try `rustc --explain E0433`.
error: could not compile `std` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
