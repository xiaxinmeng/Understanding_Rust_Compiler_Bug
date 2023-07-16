
[01:03:21] error: use of deprecated item: replaced by `Iterator::step_by`
[01:03:21]    --> /checkout/src/libcollections/../libcollections/tests/vec_deque.rs:513:21
[01:03:21]     |
[01:03:21] 513 |     let seq = (0..).step_by(2).take(256);
[01:03:21]     |                     ^^^^^^^
[01:03:21]     |
[01:03:21]     = note: #[deny(deprecated)] implied by #[deny(warnings)]
[01:03:21] note: lint level defined here
[01:03:21]    --> /checkout/src/libcollections/../libcollections/tests/lib.rs:11:9
[01:03:21]     |
[01:03:21] 11  | #![deny(warnings)]
[01:03:21]     |         ^^^^^^^^
[01:03:21] 
[01:03:21] error: aborting due to previous error(s)
[01:03:21] 
[01:03:21] error: Could not compile `collections`.
