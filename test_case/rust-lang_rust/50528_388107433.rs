plain
[00:32:13]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:32:13]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:32:13]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:32:15]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:33:26] thread 'main' panicked at 'missing CrateMetadata in DecodeContext', libcore/option.rs:914:5
[00:33:26] 
[00:33:26] error: internal compiler error: unexpected panic
[00:33:26] 
[00:33:26] 
[00:33:26] note: the compiler unexpectedly panicked. this is a bug.
[00:33:26] 
[00:33:26] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:33:26] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[00:33:26] 
[00:33:26] 
[00:33:26] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=3 -C prefer-dynamic -C panic=abort -C debug-assertions=no -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:33:26] 
[00:33:26] note: some of the compiler flags provided by cargo are hidden
[00:33:26] error: Could not compile `compiler_builtins`.
[00:33:26] 
[00:33:26] Caused by:
[00:33:26] Caused by:
[00:33:26]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name compiler_builtins rustc/compiler_builtins_shim/../../libcompiler_builtins/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 --cfg feature="c" --cfg feature="compiler-builtins" --cfg feature="default" --cfg feature="rustbuild" -C metadata=b4e196030fb3059f -C extra-filename=-b4e196030fb3059f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-07c899b6c3d44697.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-3c62409c53174beb/out --cfg use_c -l static=compiler-rt` (exit code: 101)
[00:33:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:33:26] expected success, got: exit code: 101
[00:33:26] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:33:26] travis_fold:end:stage1-std

[00:33:26] travis_time:end:stage1-std:start=1525969611605273142,finish=1525969705086182894,duration=93480909752


[00:33:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:33:26] Build completed unsuccessfully in 0:27:10
[00:33:26] make: *** [all] Error 1
[00:33:26] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0835e98c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
