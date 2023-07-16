
[00:35:01] error: unnecessary parentheses around function argument
[00:35:01]    --> src/libstd/sys/unix/thread.rs:314:14
[00:35:01]     |
[00:35:01] 314 |           Some((libc::pthread_get_stackaddr_np(libc::pthread_self()) as usize -
[00:35:01]     |  ______________^
[00:35:01] 315 | |               libc::pthread_get_stacksize_np(libc::pthread_self())))
[00:35:01]     | |___________________________________________________________________^
[00:35:01]     |
[00:35:01] note: lint level defined here
[00:35:01]    --> src/libstd/lib.rs:232:9
[00:35:01]     |
[00:35:01] 232 | #![deny(warnings)]
[00:35:01]     |         ^^^^^^^^
[00:35:01]     = note: #[deny(unused_parens)] implied by #[deny(warnings)]
[00:35:01] help: remove these parentheses
[00:35:01]     |
[00:35:01] 314 |         Some(libc::pthread_get_stackaddr_np(libc::pthread_self()) as usize -
[00:35:01] 315 |      libc::pthread_get_stacksize_np(libc::pthread_self()))
[00:35:01]     |
