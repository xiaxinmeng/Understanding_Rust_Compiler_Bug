plain
[00:21:21]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:21:21]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:21:22]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:21:22]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:22:00] error: this feature has been stable since 1.27.0. Attribute no longer needed
[00:22:00]    |
[00:22:00]    |
[00:22:00] 79 | #![feature(fn_must_use)]
[00:22:00]    |
[00:22:00]    |
[00:22:00]    = note: `-D stable-features` implied by `-D warnings`
[00:22:03] error: aborting due to previous error
[00:22:03] 
[00:22:03] error: Could not compile `core`.
[00:22:03] 
[00:22:03] 
[00:22:03] Caused by:
[00:22:03]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=64878136ec7adadb -C extra-filename=-64878136ec7adadb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:22:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:22:03] expected success, got: exit code: 101
[00:22:03] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:22:03] travis_fold:end:stage1-std

[00:22:03] travis_time:end:stage1-std:start=1524723582418145126,finish=1524723637263463266,duration=54845318140

