plain
[00:20:28]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:20:29]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:20:30]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:20:30]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:20:52] error[E0711]: feature `rust1` is declared stable since 1.30.0, but was previously declared stable since 1.0.0
[00:20:52]     |
[00:20:52]     |
[00:20:52] 497 |     #[stable(feature = "rust1", since = "1.30.0")]
[00:20:52] 
[00:20:54] error: aborting due to previous error
[00:20:54] 
[00:20:54] For more information about this error, try `rustc --explain E0711`.
[00:20:54] For more information about this error, try `rustc --explain E0711`.
[00:20:54] error: Could not compile `core`.
[00:20:54] 
[00:20:54] Caused by:
[00:20:54]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=aae624166adf9237 -C extra-filename=-aae624166adf9237 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[00:20:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:54] expected success, got: exit code: 101
[00:20:54] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:20:54] travis_fold:end:stage1-std

[00:20:54] travis_time:end:stage1-std:start=1535508085933991080,finish=1535508121542780170,duration=35608789090


[00:20:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:54] Build completed unsuccessfully in 0:15:58
[00:20:54] make: *** [all] Error 1
[00:20:54] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:205a652c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
