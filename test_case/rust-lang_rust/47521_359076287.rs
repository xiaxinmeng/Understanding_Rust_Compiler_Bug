
[00:16:46] error: unnecessary `unsafe` block
[00:16:46]     --> librustc_trans/back/write.rs:1307:35
[00:16:46]      |
[00:16:46] 1274 |         unsafe {
[00:16:46]      |         ------ because it's nested under this `unsafe` block
[00:16:46] ...
[00:16:46] 1307 |                 Lto::ThinLocal => unsafe {
[00:16:46]      |                                   ^^^^^^ unnecessary `unsafe` block
[00:16:46]      |
[00:16:46] note: lint level defined here
[00:16:46]     --> librustc_trans/lib.rs:20:9
[00:16:46]      |
[00:16:46] 20   | #![deny(warnings)]
[00:16:46]      |         ^^^^^^^^
[00:16:46]      = note: #[deny(unused_unsafe)] implied by #[deny(warnings)]
[00:16:46] 
[00:16:46] error: aborting due to previous error
[00:16:46] 
[00:16:46] error: Could not compile `rustc_trans`.
