plain
[00:18:06]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:18:07] error[E0425]: cannot find value `e` in this scope
[00:18:07]    --> librustc_lint/unused.rs:275:59
[00:18:07]     |
[00:18:07] 275 |                         if let ast::ExprKind::Break(..) = e {
[00:18:07]     |                                                           ^ not found in this scope
[00:18:08] error: aborting due to previous error
[00:18:08] 
[00:18:08] For more information about this error, try `rustc --explain E0425`.
[00:18:08] error: Could not compile `rustc_lint`.
[00:18:08] error: Could not compile `rustc_lint`.
[00:18:08] warning: build failed, waiting for other jobs to finish...
[00:18:39] error: build failed
[00:18:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:18:39] expected success, got: exit code: 101
[00:18:39] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:18:39] travis_fold:end:stage0-rustc

[00:18:39] travis_time:end:stage0-rustc:start=1539081717909378202,finish=1539082505648526554,duration=787739148352


[00:18:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:39] Build completed unsuccessfully in 0:14:09
[00:18:39] make: *** [all] Error 1
[00:18:39] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b05e9c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
151412 ./src/tools/clang
149120 ./src/llvm-emscripten/test
145036 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
135996 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113
135992 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113/s-f5k4at9773-1e1b1l1-1akhp35ixzhyj
107656 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lld
