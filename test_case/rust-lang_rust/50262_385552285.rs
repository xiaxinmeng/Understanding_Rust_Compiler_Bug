plain
[00:21:42]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:21:45] error: unreachable expression
[00:21:45]    --> libstd/error.rs:312:9
[00:21:45]     |
[00:21:45] 312 |         match *self {}
[00:21:45]     |
[00:21:45]     = note: `-D unreachable-code` implied by `-D warnings`
[00:21:45] 
[00:21:46] error: unreachable expression
[00:21:46] error: unreachable expression
[00:21:46]     --> libstd/path.rs:1457:9
[00:21:46]      |
[00:21:46] 1457 |         match *self {}
[00:21:46] 
[00:21:46] error: unreachable expression
[00:21:46]     --> libstd/process.rs:1480:28
[00:21:46]      |
[00:21:46]      |
[00:21:46] 1480 |     fn report(self) -> i32 { self }
[00:21:46] 
iler_builtins-8e2608424e362418/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/native/jemalloc/lib` (exit code: 101)
[00:21:47] travis_fold:end:stage1-std


[00:21:47] travis_time:end:stage1-std:start=1525128672317608722,finish=1525128742031870149,duration=69714261427

[00:21:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:47] expected success, got: exit code: 101
[00:21:47] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:21:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:47] Build completed unsuccessfully in 0:16:06
[00:21:47] Build completed unsuccessfully in 0:16:06
[00:21:47] make: *** [all] Error 1
[00:21:47] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08e71afd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
