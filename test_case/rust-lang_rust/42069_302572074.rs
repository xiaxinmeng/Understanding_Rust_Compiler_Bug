
[00:01:36] Command failed. Attempt 2/5:
[00:01:37]    Compiling bootstrap v0.0.0 (file:///Users/travis/build/rust-lang/rust/src/bootstrap)
[00:01:37] error[E0308]: mismatched types
[00:01:37]    --> src/bootstrap/lib.rs:128:31
[00:01:37]     |
[00:01:37] 128 |             libc::setpriority(PRIO_PGRP, 0, 10);
[00:01:37]     |                               ^^^^^^^^^ expected i32, found u32
[00:01:37] 
[00:01:39] error: aborting due to previous error
[00:01:39] 
[00:01:39] error: Could not compile `bootstrap`.
[00:01:39] 
[00:01:39] To learn more, run the command again with --verbose.
[00:01:39] Build completed unsuccessfully in 0:00:02
[00:01:39] make: *** [prepare] Error 101
