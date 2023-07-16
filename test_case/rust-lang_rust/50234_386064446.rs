plain
[00:03:20]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:03:25]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:03:27]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:03:28]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:03:28] error[E0599]: no method named `for_each` found for type `T` in the current scope
[00:03:28]     |
[00:03:28]     |
[00:03:28] 358 |         iter.for_each(drop)
[00:03:28]     |
[00:03:28]     |
[00:03:28]     = note: the method `for_each` exists but the following trait bounds were not satisfied:
[00:03:28]             `&mut T : iter::iterator::Iterator`
[00:03:28]     = help: items from traits can only be used if the trait is implemented and in scope
[00:03:28]     = note: the following trait defines an item `for_each`, perhaps you need to implement it:
[00:03:28]             candidate #1: `iter::iterator::Iterator`
[00:03:29]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:03:30] error: aborting due to previous error
[00:03:30] 
[00:03:30] For more information about this error, try `rustc --explain E0599`.
[00:03:30] For more information about this error, try `rustc --explain E0599`.
[00:03:30] error: Could not compile `core`.
[00:03:30] 
[00:03:30] Caused by:
[00:03:30]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=fb1e36473ec4786e -C extra-filename=-fb1e36473ec4786e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:30] warning: build failed, waiting for other jobs to finish...
[00:03:38] error: build failed
[00:03:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:38] expected success, got: exit code: 101
[00:03:38] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:38] travis_fold:end:stage0-std

[00:03:38] travis_time:end:stage0-std:start=1525283519440056779,finish=1525283546076280245,duration=26636223466


[00:03:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:38] Build completed unsuccessfully in 0:00:28
[00:03:38] Makefile:79: recipe for target 'tidy' failed
[00:03:38] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:030b9ea8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
