
[00:04:15] error: constant item is never used: `HEAP_REALLOC_IN_PLACE_ONLY`
[00:04:15]    --> src\liballoc_system\old.rs:213:5
[00:04:15]     |
[00:04:15] 213 |     const HEAP_REALLOC_IN_PLACE_ONLY: DWORD = 0x00000010;
[00:04:15]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:15]     |
[00:04:15]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
[00:04:15] note: lint level defined here
[00:04:15]    --> src\liballoc_system\lib.rs:14:9
[00:04:15]     |
[00:04:15] 14  | #![deny(warnings)]
[00:04:15]     |         ^^^^^^^^
[00:04:15]
[00:04:15] error: aborting due to previous error(s)
[00:04:15]
[00:04:15] error: Could not compile `alloc_system`.
