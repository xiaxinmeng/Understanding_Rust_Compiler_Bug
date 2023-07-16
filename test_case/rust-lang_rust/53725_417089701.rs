
[00:25:39] error: unused import: `libc`
[00:25:39]   --> libstd/sys/unix/rand.rs:32:9
[00:25:39]    |
[00:25:39] 32 |     use libc;
[00:25:39]    |         ^^^^
[00:25:39]    |
[00:25:39]    = note: `-D unused-imports` implied by `-D warnings`
[00:25:39] 
[00:25:39] error: unused import: `sys::os::errno`
[00:25:39]   --> libstd/sys/unix/rand.rs:33:9
[00:25:39]    |
[00:25:39] 33 |     use sys::os::errno;
[00:25:39]    |         ^^^^^^^^^^^^^^
[00:25:39] 
[00:25:43] error: aborting due to 2 previous errors
