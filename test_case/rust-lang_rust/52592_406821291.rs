plain
[00:07:32]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:55] error[E0282]: type annotations needed
[00:07:55]    --> librustc/middle/resolve_lifetime.rs:399:36
[00:07:55]     |
[00:07:55] 394 |     let mut defs = FxHashMap();
[00:07:55]     |         -------- consider giving `defs` a type
[00:07:55] ...
[00:07:55] 399 |         Lrc::get_mut(map).unwrap().insert(hir_id.local_id, v);
[00:07:55]     |                                    ^^^^^^ cannot infer type for `T`
[00:07:55]     |
[00:07:55]     = note: type must be known at this point
elease/build/backtrace-sys-7205e6adb9d66e6b/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-27571ee36438df44/out` (exit code: 101)
[00:08:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:04] expected success, got: exit code: 101
[00:08:04] expected success, got: exit code: 101
[00:08:04] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:08:04] travis_fold:end:stage0-rustc

[00:08:04] travis_time:end:stage0-rustc:start=1532204293804408973,finish=1532204473876744923,duration=180072335950


[00:08:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:04] Build completed unsuccessfully in 0:04:02
[00:08:04] Makefile:28: recipe for target 'all' failed
[00:08:04] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0add39bc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
