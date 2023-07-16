
[01:06:05] error[E0599]: no method named `try_into` found for type `i64` in the current scope
[01:06:05]   --> libstd/sys/unix/fd.rs:80:35
[01:06:05]    |
[01:06:05] 80 |             if let Ok(o) = offset.try_into() {
[01:06:05]    |                                   ^^^^^^^^
[01:06:05]    |
[01:06:05]    = help: items from traits can only be used if the trait is in scope
[01:06:05]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:06:05]            candidate #1: `use core::convert::TryInto;`
[01:06:05] 
[01:06:05] error[E0599]: no method named `try_into` found for type `i64` in the current scope
[01:06:05]    --> libstd/sys/unix/fd.rs:127:35
[01:06:05]     |
[01:06:05] 127 |             if let Ok(o) = offset.try_into() {
[01:06:05]     |                                   ^^^^^^^^
[01:06:05]     |
[01:06:05]     = help: items from traits can only be used if the trait is in scope
[01:06:05]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:06:05]             candidate #1: `use core::convert::TryInto;`
[01:06:05] 
[01:06:05] error: aborting due to 2 previous errors
