plain
[00:24:37]    Compiling atty v0.2.8
[00:24:37]    Compiling backtrace-sys v0.1.16
[00:24:41]    Compiling miniz-sys v0.1.10
[00:24:41]    Compiling rustc_cratesio_shim v0.0.0 (file:///checkout/src/librustc_cratesio_shim)
[00:24:41] error: failed to run custom build command for `syntax v0.0.0 (file:///checkout/src/libsyntax)`
[00:24:41] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/syntax-5f3de57df7d70275/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
[00:24:46] error: build failed
[00:24:46] error: build failed
[00:24:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:24:46] expected success, got: exit code: 101
[00:24:46] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:24:46] travis_fold:end:stage1-rustc

[00:24:46] travis_time:end:stage1-rustc:start=1524662791826013199,finish=1524662812586433926,duration=20760420727


[00:24:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:24:46] Build completed unsuccessfully in 0:19:52
[00:24:46] Makefile:28: recipe for target 'all' failed
[00:24:46] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02fafc58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
