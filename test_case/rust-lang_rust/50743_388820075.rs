plain
[00:03:19]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:03:19]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:03:21]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:03:21]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:03:25] error[E0016]: blocks in constant functions are limited to items and tail expressions
[00:03:25]     --> libcore/ptr.rs:2690:23
[00:03:25]      |
[00:03:25] 2690 |             let ptr = mem::align_of::<T>() as *mut T;
[00:03:25] 
[00:03:26] error: aborting due to previous error
[00:03:26] 
[00:03:26] For more information about this error, try `rustc --explain E0016`.
[00:03:26] For more information about this error, try `rustc --explain E0016`.
[00:03:26] error: Could not compile `core`.
[00:03:26] 
[00:03:26] Caused by:
[00:03:26]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=fb1e36473ec4786e -C extra-filename=-fb1e36473ec4786e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:26] warning: build failed, waiting for other jobs to finish...
[00:03:33] error: build failed
[00:03:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:33] expected success, got: exit code: 101
[00:03:33] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:33] travis_fold:end:stage0-std

[00:03:33] travis_time:end:stage0-std:start=1526305113470499331,finish=1526305141511264064,duration=28040764733


[00:03:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:33] Build completed unsuccessfully in 0:00:29
[00:03:33] make: *** [tidy] Error 1
[00:03:33] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12069520
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
