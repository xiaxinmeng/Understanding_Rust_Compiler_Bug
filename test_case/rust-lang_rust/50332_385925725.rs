plain
[00:21:37]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:21:38]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:21:38]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:21:42] error: build failed
[00:21:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:42] expected success, got: exit code: 101
[00:21:42] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:21:42] travis_fold:end:stage1-std

[00:21:42] travis_time:end:stage1-std:start=1525254900604237184,finish=1525254919203219074,duration=18598981890


[00:21:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:42] Build completed unsuccessfully in 0:16:58
[00:21:42] make: *** [all] Error 1
[00:21:42] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25cddfe1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
