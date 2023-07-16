plain
[00:59:12] [RUSTC-TIMING] syntax test:false 139.081
[00:59:38] [RUSTC-TIMING] racer test:false 28.143
[01:00:05] [RUSTC-TIMING] rustfmt_nightly test:false 52.651
[01:00:10] warning: variable does not need to be mutable
[01:00:10]   --> tools/rls/src/actions/hover.rs:38:13
[01:00:10]    |
[01:00:10] 38 |         let mut trimmed = line.trim();
[01:00:10]    |             |
[01:00:10]    |             help: remove this `mut`
[01:00:10]    |
[01:00:10]    = note: #[warn(unused_mut)] on by default
[01:00:10]    = note: #[warn(unused_mut)] on by default
[01:00:10] 
[01:00:10] warning: variable does not need to be mutable
[01:00:10]   --> tools/rls/src/actions/hover.rs:51:13
[01:00:10]    |
[01:00:10] 51 |         let mut line = if in_rust_codeblock && trimmed.starts_with("