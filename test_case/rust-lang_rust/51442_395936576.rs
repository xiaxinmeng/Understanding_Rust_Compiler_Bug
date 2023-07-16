plain
[00:03:18]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:03:20] error[E0412]: cannot find type `T` in this scope
[00:03:20]   --> libcore/task.rs:60:18
[00:03:20]    |
[00:03:20] 60 | impl Poll<Result<T, E>> {
[00:03:20]    |                  ^ not found in this scope
[00:03:20] error[E0412]: cannot find type `E` in this scope
[00:03:20]   --> libcore/task.rs:60:21
[00:03:20]    |
[00:03:20]    |
[00:03:20] 60 | impl Poll<Result<T, E>> {
[00:03:20]    |                     ^ did you mean `Eq`?
[00:03:20] error[E0412]: cannot find type `T` in this scope
[00:03:20]   --> libcore/task.rs:63:25
[00:03:20]    |
[00:03:20]    |
[00:03:20] 63 |         where F: FnOnce(T) -> U
[00:03:20]    |                         ^ did you mean `F`?
[00:03:20] error[E0412]: cannot find type `E` in this scope
[00:03:20]   --> libcore/task.rs:62:51
[00:03:20]    |
[00:03:20]    |
[00:03:20] 62 |     fn map_ok<U, F>(self, f: F) -> Poll<Result<U, E>>
[00:03:20]    |                                                   ^ did you mean `Eq`?
[00:03:20] error[E0412]: cannot find type `E` in this scope
[00:03:20]   --> libcore/task.rs:74:25
[00:03:20]    |
[00:03:20]    |
[00:03:20] 74 |         where F: FnOnce(E) -> U
[00:03:20]    |                         ^ did you mean `Eq`?
[00:03:20] error[E0412]: cannot find type `T` in this scope
[00:03:20]   --> libcore/task.rs:73:49
[00:03:20]    |
[00:03:20]    |
[00:03:20] 73 |     fn map_err<U, F>(self, f: F) -> Poll<Result<T, U>>
[00:03:20]    |                                                 ^ did you mean `F`?
[00:03:24]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:03:24]    Compiling cmake v0.1.30
[00:03:24]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:03:29]    Compiling std v0.0.0 (file:///checkout/src/libstd)
---
[00:03:32] Caused by:
[00:03:32]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=e9cdce497aae9e81 -C extra-filename=-e9cdce497aae9e81 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:32] warning: build failed, waiting for other jobs to finish...
[00:03:42] error: build failed
[00:03:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:42] expected success, got: exit code: 101
[00:03:42] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:42] travis_fold:end:stage0-std

[00:03:42] travis_time:end:stage0-std:start=1528515254429284768,finish=1528515279341944884,duration=24912660116


[00:03:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:42] Build completed unsuccessfully in 0:00:26
[00:03:42] Makefile:79: recipe for target 'tidy' failed
[00:03:42] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1cb42ddd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
