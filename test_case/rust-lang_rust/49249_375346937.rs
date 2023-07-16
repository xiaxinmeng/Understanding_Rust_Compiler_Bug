
[00:20:22] error: unreachable expression
[00:20:22]    --> librustc_trans/builder.rs:918:13
[00:20:22]     |
[00:20:22] 918 |             instr
[00:20:22]     |             ^^^^^
[00:20:22]     |
[00:20:22] note: lint level defined here
[00:20:22]    --> librustc_trans/lib.rs:20:9
[00:20:22]     |
[00:20:22] 20  | #![deny(warnings)]
[00:20:22]     |         ^^^^^^^^
[00:20:22]     = note: #[deny(unreachable_code)] implied by #[deny(warnings)]
[00:20:22] 
[00:20:22] error: unreachable expression
[00:20:22]    --> librustc_trans/builder.rs:926:13
[00:20:22]     |
[00:20:22] 926 |             instr
[00:20:22]     |             ^^^^^
[00:20:22] 
[00:20:24] error: aborting due to 2 previous errors
[00:20:24] 
[00:20:24] error: Could not compile `rustc_trans`.
