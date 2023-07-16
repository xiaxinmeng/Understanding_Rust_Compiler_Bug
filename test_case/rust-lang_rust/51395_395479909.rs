plain
[00:42:43]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:42:43]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:42:44]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:42:44]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:43:19] error: this feature has been stable since 1.28.0. Attribute no longer needed
[00:43:19]     |
[00:43:19]     |
[00:43:19] 104 | #![feature(repr_transparent)]
[00:43:19]     |
[00:43:19]     = note: `-D stable-features` implied by `-D warnings`
[00:43:19] 
[00:43:21] error: aborting due to previous error
[00:43:21] error: aborting due to previous error
[00:43:21] 
[00:43:21] error: Could not compile `core`.
[00:43:21] 
[00:43:21] Caused by:
[00:43:21]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=e3f2b558276b578b -C extra-filename=-e3f2b558276b578b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:43:21] warning: build failed, waiting for other jobs to finish...
[00:43:49] error: build failed
[00:43:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:43:49] expected success, got: exit code: 101
[00:43:49] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:43:49] travis_fold:end:stage1-std

[00:43:49] travis_time:end:stage1-std:start=1528388177147867332,finish=1528388254729928162,duration=77582060830


[00:43:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:43:49] Build completed unsuccessfully in 0:17:13
[00:43:49] Makefile:28: recipe for target 'all' failed
[00:43:49] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fabf665
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
