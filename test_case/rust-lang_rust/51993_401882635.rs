plain
[00:18:39]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:18:39]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:18:40]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:18:40]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:19:08] error[E0597]: borrowed value does not live long enough
[00:19:08]   --> libcore/panic.rs:58:31
[00:19:08]    |
[00:19:08] 58 |         PanicInfo { payload: &NoPayload, location, message }
[00:19:08]    |                               ^^^^^^^^^ temporary value does not live long enough
[00:19:08] 59 |     }
[00:19:08]    |     - temporary value only lives until here
[00:19:08]    |
[00:19:08] note: borrowed value must be valid for the lifetime 'a as defined on the impl at 47:6...
[00:19:08]   --> libcore/panic.rs:47:6
[00:19:08] 47 | impl<'a> PanicInfo<'a> {
[00:19:08]    |      ^^
[00:19:08] 
[00:19:13] error: aborting due to previous error
[00:19:13] error: aborting due to previous error
[00:19:13] 
[00:19:13] For more information about this error, try `rustc --explain E0597`.
[00:19:13] error: Could not compile `core`.
[00:19:13] 
[00:19:13] Caused by:
[00:19:13]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=9d37d95f0c4f2954 -C extra-filename=-9d37d95f0c4f2954 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:19:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:19:13] expected success, got: exit code: 101
[00:19:13] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:19:13] travis_fold:end:stage1-std

[00:19:13] travis_time:end:stage1-std:start=1530553676908148709,finish=1530553721731750493,duration=44823601784


[00:19:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:13] Build completed unsuccessfully in 0:14:41
[00:19:13] Makefile:28: recipe for target 'all' failed
[00:19:13] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:031c1e80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jul  2 17:48:42 UTC 2018
/modules
158412 ./.git/modules/src
149120 ./src/llvm-emscripten/test
144408 ./obj/build/bootstrap/debug/incremental
129896 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9
129892 ./obj/build/bootstrap/debug/incremental/bootstrap-146vjsckowoo9/s-f2ja76iejg-hbe8rr-3ah8kbogwyk4h
89812 ./src/llvm/test/CodeGen
88916 ./obj/build/x86_64-unknown-linux-gnu/stage1
88892 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
76232 ./.git/modules/src/tools
