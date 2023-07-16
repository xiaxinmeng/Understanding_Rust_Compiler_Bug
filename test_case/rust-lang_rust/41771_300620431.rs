
[00:01:40] error: unused variable: `count`
[00:01:40]     --> /checkout/src/libcollections/vec.rs:1337:17
[00:01:40]      |
[00:01:40] 1337 |             let count = new_len - len;
[00:01:40]      |                 ^^^^^
[00:01:40]      |
[00:01:40]      = note: #[deny(unused_variables)] implied by #[deny(warnings)]
[00:01:40] note: lint level defined here
[00:01:40]     --> /checkout/src/libcollections/lib.rs:30:9
[00:01:40]      |
[00:01:40] 30   | #![deny(warnings)]
[00:01:40]      |         ^^^^^^^^
[00:01:40] 
[00:01:40] error: aborting due to previous error
