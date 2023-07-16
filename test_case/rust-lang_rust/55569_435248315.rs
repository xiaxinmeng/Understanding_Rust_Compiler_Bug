
[00:36:38]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:36:43] error: unused `std::result::Result` that must be used
[00:36:43]    --> librustc_resolve/resolve_imports.rs:785:17
[00:36:43]     |
[00:36:43] 785 |                 write!(suggestion_choices, "`::{}`", name);
[00:36:43]     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:36:43]     |
[00:36:43]     = note: `-D unused-must-use` implied by `-D warnings`
[00:36:43]     = note: this `Result` may be an `Err` variant, which should be handled
[00:36:43]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:36:43]
[00:36:43] error: unused `std::result::Result` that must be used
[00:36:43]    --> librustc_resolve/resolve_imports.rs:793:17
[00:36:43]     |
[00:36:43] 793 |                 write!(suggestion_choices, "`self::{}`", name);
[00:36:43]     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:36:43]     |
[00:36:43]     = note: this `Result` may be an `Err` variant, which should be handled
[00:36:43]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:36:43]
[00:36:43] error: aborting due to 2 previous errors
[00:36:43]
[00:36:43] error: Could not compile `rustc_resolve`.
