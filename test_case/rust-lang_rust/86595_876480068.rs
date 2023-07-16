plain
    Checking test v0.0.0 (/checkout/library/test)
error[E0658]: use of unstable library feature 'allocator_api'
   --> library/test/src/lib.rs:272:53
    |
272 |     let mut timeout_queue: VecDeque<TimeoutEntry> = VecDeque::new();
    |
    = note: see issue #32838 <https://github.com/rust-lang/rust/issues/32838> for more information
    = note: see issue #32838 <https://github.com/rust-lang/rust/issues/32838> for more information
    = help: add `#![feature(allocator_api)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
error: could not compile `test`
