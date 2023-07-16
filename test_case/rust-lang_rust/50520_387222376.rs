plain
[00:21:03]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:21:03]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:21:04]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:21:04]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:21:44] thread 'main' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:21:44] 
[00:21:44] error: internal compiler error: unexpected panic
[00:21:44] 
[00:21:44] 
[00:21:44] note: the compiler unexpectedly panicked. this is a bug.
[00:21:44] 
[00:21:44] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:21:44] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[00:21:44] 
[00:21:44] 
[00:21:44] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=3 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:21:44] 
[00:21:44] note: some of the compiler flags provided by cargo are hidden
[00:21:44] error: Could not compile `core`.
[00:21:44] 
[00:21:44] Caused by:
[00:21:44]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=07c899b6c3d44697 -C extra-filename=-07c899b6c3d44697 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:21:44]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=07c899b6c3d44697 -C extra-filename=-07c899b6c3d44697 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:21:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:44] expected success, got: exit code: 101
[00:21:44] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:21:44] travis_fold:end:stage1-std

[00:21:44] travis_time:end:stage1-std:start=1525730947458649905,finish=1525731000868459000,duration=53409809095


[00:21:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:44] Build completed unsuccessfully in 0:16:56
[00:21:44] make: *** [all] Error 1
[00:21:44] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03a7d0d4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
