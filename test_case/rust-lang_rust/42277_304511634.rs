
[00:06:12] error: variable does not need to be mutable
[00:06:12]     --> /checkout/src/librustc/session/config.rs:1634:9
[00:06:12]      |
[00:06:12] 1634 |     let mut emit_metadata = false;
[00:06:12]      |         ^^^^^^^^^^^^^^^^^
[00:06:12]      |
[00:06:12]      = note: #[deny(unused_mut)] implied by #[deny(warnings)]
[00:06:12] note: lint level defined here
[00:06:12]     --> /checkout/src/librustc/lib.rs:23:9
[00:06:12]      |
[00:06:12] 23   | #![deny(warnings)]
[00:06:12]      |         ^^^^^^^^
[00:06:12] 
[00:06:12] error: aborting due to previous error
