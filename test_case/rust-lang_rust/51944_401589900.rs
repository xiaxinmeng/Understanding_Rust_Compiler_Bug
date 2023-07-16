plain
[00:03:27]    Compiling cc v1.0.15
[00:03:27]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:03:27]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:03:27]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:03:29] error[E0405]: cannot find trait `UnsafeFutureObj` in this scope
[00:03:29]     --> libcore/mem.rs:1232:48
[00:03:29]      |
[00:03:29] 1232 | unsafe impl<'a, T, F: Future<Output = T> + 'a> UnsafeFutureObj<'a, T> for PinMut<'a, F> {
[00:03:29] help: possible candidate is found in another module, you can import it into scope
[00:03:29]      |
[00:03:29]      |
[00:03:29] 18   | use future::future_obj::UnsafeFutureObj;
[00:03:29] 
[00:03:29] 
[00:03:29] error[E0405]: cannot find trait `Future` in this scope
[00:03:29]     --> libcore/mem.rs:1232:23
[00:03:29]      |
[00:03:29] 1232 | unsafe impl<'a, T, F: Future<Output = T> + 'a> UnsafeFutureObj<'a, T> for PinMut<'a, F> {
[00:03:29] help: possible candidate is found in another module, you can import it into scope
[00:03:29]      |
[00:03:29]      |
[00:03:29] 18   | use future::future::Future;
[00:03:29] 
[00:03:29] error[E0412]: cannot find type `Context` in this scope
[00:03:29]     --> libcore/mem.rs:1237:43
[00:03:29]      |
[00:03:29]      |
[00:03:29] 1237 |     unsafe fn poll(ptr: *mut (), cx: &mut Context) -> Poll<T> {
[00:03:29] help: possible candidate is found in another module, you can import it into scope
[00:03:29]      |
[00:03:29] 18   | use task::context::Context;
[00:03:29]      |
[00:03:29]      |
[00:03:29] 
[00:03:29] error[E0412]: cannot find type `Poll` in this scope
[00:03:29]     --> libcore/mem.rs:1237:55
[00:03:29]      |
[00:03:29] 1237 |     unsafe fn poll(ptr: *mut (), cx: &mut Context) -> Poll<T> {
[00:03:29] help: possible candidate is found in another module, you can import it into scope
[00:03:29]      |
[00:03:29] 18   | use task::poll::Poll;
[00:03:29]      |
---
[00:03:30] Caused by:
[00:03:30]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=c8f7a210a5d40dd6 -C extra-filename=-c8f7a210a5d40dd6 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:30] warning: build failed, waiting for other jobs to finish...
[00:03:31] error: build failed
[00:03:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:31] expected success, got: exit code: 101
[00:03:31] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:31] travis_fold:end:stage0-std

[00:03:31] travis_time:end:stage0-std:start=1530430443868978546,finish=1530430448871414077,duration=5002435531


[00:03:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:31] Build completed unsuccessfully in 0:00:06
[00:03:31] make: *** [tidy] Error 1
[00:03:31] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06dec062
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:12ad5af3:start=1530430449356663655,finish=1530430449363300232,duration=6636577
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:060795bc
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:002603ab
$ dmesg | grep -i kill
