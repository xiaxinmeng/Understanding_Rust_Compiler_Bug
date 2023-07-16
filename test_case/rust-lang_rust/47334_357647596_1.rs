
[00:51:13] error: function is never used: `on_resolver_failure`
[00:51:13]    --> libstd/sys/wasm/net.rs:300:1
[00:51:13]     |
[00:51:13] 300 | pub fn on_resolver_failure(e: io::Error) -> io::Error { e }
[00:51:13]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:51:13]     |
[00:51:13] note: lint level defined here
[00:51:13]    --> libstd/lib.rs:232:31
[00:51:13]     |
[00:51:13] 232 | #![cfg_attr(not(stage0), deny(warnings))]
[00:51:13]     |                               ^^^^^^^^
[00:51:13]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
