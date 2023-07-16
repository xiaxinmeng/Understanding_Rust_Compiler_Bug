plain
[00:23:12]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:23:16]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:23:16]    Compiling cmake v0.1.30
[00:23:16]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:23:20] thread 'main' panicked at 'attempt to subtract with overflow', librustc/middle/resolve_lifetime.rs:144:38
[00:23:20] 
[00:23:20] error: internal compiler error: unexpected panic
[00:23:20] 
[00:23:20] 
[00:23:20] note: the compiler unexpectedly panicked. this is a bug.
[00:23:20] 
[00:23:20] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:23:20] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[00:23:20] 
[00:23:20] 
[00:23:20] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=3 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:23:20] 
[00:23:20] note: some of the compiler flags provided by cargo are hidden
[00:23:21] error: Could not compile `core`.
[00:23:21] 
[00:23:21] Caused by:
[00:23:21]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=07c899b6c3d44697 -C extra-filename=-07c899b6c3d44697 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:23:21]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=07c899b6c3d44697 -C extra-filename=-07c899b6c3d44697 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:23:21] warning: build failed, waiting for other jobs to finish...
[00:23:22] error: build failed
[00:23:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:22] expected success, got: exit code: 101
[00:23:22] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:23:22] travis_fold:end:stage1-std

[00:23:22] travis_time:end:stage1-std:start=1525781799613647394,finish=1525781813472467161,duration=13858819767


[00:23:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:22] Build completed unsuccessfully in 0:18:24
[00:23:22] Makefile:28: recipe for target 'all' failed
[00:23:22] make: *** [all] Error 1
55380 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53592 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
48604 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
47892 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
