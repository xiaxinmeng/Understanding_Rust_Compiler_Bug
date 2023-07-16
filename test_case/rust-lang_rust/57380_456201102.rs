
[00:08:22] error[E0308]: mismatched types
[00:08:22]   --> src/libstd/sys/windows/time.rs:46:22
[00:08:22]    |
[00:08:22] 46 |         Instant { t: 0 }
[00:08:22]    |                      ^ expected struct `core::time::Duration`, found integral variable
[00:08:22]    |
[00:08:22]    = note: expected type `core::time::Duration`
[00:08:22]               found type `{integer}`
[00:08:22] 
[00:08:22] error: aborting due to previous error
[00:08:22] 
[00:08:22] For more information about this error, try `rustc --explain E0308`.
[00:08:22] error: Could not compile `std`.
