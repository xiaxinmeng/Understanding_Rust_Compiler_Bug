plain
[02:25:57] test thread::tests::test_try_panic_message_owned_str ... ok
[02:25:57] test thread::tests::test_try_panic_message_static_str ... ok
[02:25:57] test thread::tests::test_try_panic_message_unit_struct ... ok
[02:25:57] test thread::tests::test_unnamed_thread ... ok
[02:25:57] thread '<unnamed>' panicked at 'specified instant was later than self', libstd/sys/unix/time.rs:292:17
[02:25:57] test time::tests::instant_elapsed ... ok
[02:25:57] test time::tests::instant_math ... ok
[02:25:57] test time::tests::instant_monotonic ... ok
[02:25:57] test time::tests::since_epoch ... ok
---
[02:43:47] [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: "ui", suite: "rustdoc-ui", path: None, compare_mode: None } -- 0.495
[02:43:47] Build completed successfully in 1:43:21
[02:43:47] Distcheck rust-src
[02:43:47]     Updating registry `https://github.com/rust-lang/crates.io-index`
[02:44:27] warning: spurious network error (2 tries remaining): curl error: Couldn't resolve host 'github.com'
[02:44:27] ; class=Net (12)
[02:44:47] warning: spurious network error (1 tries remaining): curl error: Couldn't resolve host 'github.com'
[02:44:47] ; class=Net (12)
[02:45:07] error: failed to load source for a dependency on `rand`
[02:45:07] Caused by:
[02:45:07]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[02:45:07] 
[02:45:07] Caused by:
[02:45:07] Caused by:
[02:45:07]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[02:45:07] 
[02:45:07] Caused by:
[02:45:07]   curl error: Couldn't resolve host 'github.com'
[02:45:07] ; class=Net (12)
[02:45:07] 
[02:45:07] 
[02:45:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[02:45:07] 
[02:45:07] 
[02:45:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[02:45:07] Build completed unsuccessfully in 2:42:19
