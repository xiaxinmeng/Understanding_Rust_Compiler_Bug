
[00:04:39] error: unused import: `self`
[00:04:39]   --> src/libstd/sys/unix/rand.rs:31:14
[00:04:39]    |
[00:04:39] 31 |     use io::{self, Read};
[00:04:39]    |              ^^^^
[00:04:39]    |
[00:04:39] note: lint level defined here
[00:04:39]   --> src/libstd/lib.rs:232:9
[00:04:39]    |
[00:04:39] 232| #![deny(warnings)]
[00:04:39]    |         ^^^^^^^^
[00:04:39]    = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[00:04:39] 
[00:04:43] error: aborting due to previous error
[00:04:43] 
[00:04:43] error: Could not compile `std`.
