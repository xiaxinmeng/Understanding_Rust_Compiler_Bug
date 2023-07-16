
[00:57:24] stderr:
[00:57:24] ------------------------------------------
[00:57:24] warning: unused `std::result::Result` which must be used
[00:57:24]    --> /checkout/src/test/run-pass/impl-trait/example-calendar.rs:315:13
[00:57:24]     |
[00:57:24] 315 |             write!(s, "{}", e);
[00:57:24]     |             ^^^^^^^^^^^^^^^^^^^
[00:57:24]     |
[00:57:24]     = note: #[warn(unused_must_use)] on by default
[00:57:24]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:57:24] 
[00:57:24] warning: unused `std::result::Result` which must be used
[00:57:24]    --> /checkout/src/test/run-pass/impl-trait/example-calendar.rs:318:17
[00:57:24]     |
[00:57:24] 318 |                 write!(s, "{}", e);
[00:57:24]     |                 ^^^^^^^^^^^^^^^^^^^
[00:57:24]     |
[00:57:24]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:57:24] 
[00:57:24] warning: unused `std::result::Result` which must be used
[00:57:24]    --> /checkout/src/test/run-pass/impl-trait/example-calendar.rs:542:13
[00:57:24]     |
[00:57:24] 542 |             write!(buf, " {:>2}", d.day());
[00:57:24]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:57:24]     |
[00:57:24]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
