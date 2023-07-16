
[01:12:57]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:13:47] error: function is never used: `into_rc`
[01:13:47]     --> libstd/path.rs:4184:5
[01:13:47]      |
[01:13:47] 4184 |     fn into_rc() {
[01:13:47]      |     ^^^^^^^^^^^^
[01:13:47]      |
[01:13:47] note: lint level defined here
[01:13:47]     --> libstd/lib.rs:232:31
[01:13:47]      |
[01:13:47] 232  | #![cfg_attr(not(stage0), deny(warnings))]
[01:13:47]      |                               ^^^^^^^^
[01:13:47]      = note: #[deny(dead_code)] implied by #[deny(warnings)]
