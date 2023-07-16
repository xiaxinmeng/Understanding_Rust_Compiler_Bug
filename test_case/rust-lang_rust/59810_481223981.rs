`
[01:42:31] test client_use_statement_completion_doesnt_suggest_arguments ... ok
[01:42:31] test client_test_simple_workspace ... ok
[01:42:32] test client_workspace_symbol ... ok
[01:42:32] test client_workspace_symbol_duplicates ... ok
[01:42:56] thread panicked while panicking. aborting.
[01:42:56] [2019-04-09T11:54:19Z ERROR rls::server] Can't read message
[01:42:56] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/libcore/result.rs:997:5
[01:42:56] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:56] error: process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/client-55bfb9e00fb32a17` (signal: 4, SIGILL: illegal instruction)
[01:42:56]
[01:42:56]
[01:42:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--"
[01:42:56] expected success, got: exit code: 101
[01:42:56]
[01:42:56]
[01:42:56] [TIMING] Rls { stage: 2, host: "x86_64-unknown-linux-gnu" } -- 212.034
Building stage2 tool rustfmt (x86_64-unknown-linux-gnu)
[01:42:57]    Compiling rustfmt-nightly v1.2.0 (/checkout/src/tools/rustfmt)
[01:44:08] [RUSTC-TIMING] rustfmt_nightly test:false 70.169
[01:44:23] [RUSTC-TIMING] rustfmt_format_diff test:false 14.716
[01:44:32] [RUSTC-TIMING] cargo_fmt test:false 23.426
[01:44:33] [RUSTC-TIMING] git_rustfmt test:false 24.930
[01:44:36] [RUSTC-TIMING] rustfmt test:false 27.628
