plain
[00:03:54]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:03:56] error[E0308]: mismatched types
[00:03:56]    --> libcore/time.rs:535:20
[00:03:56]     |
[00:03:56] 535 |             nanos: nanos as u64,
[00:03:56]     |                    ^^^^^^^^^^^^ expected u32, found u64
[00:03:56] error[E0308]: mismatched types
[00:03:56]    --> libcore/time.rs:561:20
[00:03:56]     |
[00:03:56]     |
[00:03:56] 561 |             nanos: nanos as u64,
[00:03:56]     |                    ^^^^^^^^^^^^ expected u32, found u64
[00:03:56] error[E0308]: mismatched types
[00:03:56]    --> libcore/time.rs:610:20
[00:03:56]     |
[00:03:56]     |
[00:03:56] 610 |             nanos: nanos as u64,
[00:03:56]     |                    ^^^^^^^^^^^^ expected u32, found u64
[00:03:56] error: aborting due to 3 previous errors
[00:03:56] 
[00:03:56] For more information about this error, try `rustc --explain E0308`.
[00:03:56] error: Could not compile `core`.
