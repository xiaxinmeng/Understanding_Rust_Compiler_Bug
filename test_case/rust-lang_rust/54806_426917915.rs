plain
[00:04:54]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:00] warning: unused `sync::mutex::MutexGuard` which must be used
[00:05:00]     --> libstd/thread/mod.rs:1093:9
[00:05:00]      |
[00:05:00] 1093 |         self.inner.lock.lock().unwrap();
[00:05:00]      |
[00:05:00]      = note: #[warn(unused_must_use)] on by default
[00:05:00]      = note: #[warn(unused_must_use)] on by default
[00:05:00]      = note: if unused the Mutex will immediately unlock
[00:05:07]     Finished release [optimized] target(s) in 44.15s
[00:05:07] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:05:07] travis_fold:end:stage0-std

---
[00:20:15]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:20:22] error: unused `sync::mutex::MutexGuard` which must be used
[00:20:22]     --> libstd/thread/mod.rs:1093:9
[00:20:22]      |
[00:20:22] 1093 |         self.inner.lock.lock().unwrap();
[00:20:22]      |
[00:20:22]      = note: `-D unused-must-use` implied by `-D warnings`
[00:20:22]      = note: `-D unused-must-use` implied by `-D warnings`
[00:20:22]      = note: if unused the Mutex will immediately unlock
[00:20:23] error: aborting due to previous error
[00:20:23] 
[00:20:23] error: Could not compile `std`.
[00:20:23] 
[00:20:23] 
[00:20:23] To learn more, run the command again with --verbose.
[00:20:23] travis_fold:end:stage1-std

[00:20:23] travis_time:end:stage1-std:start=1538639012400258770,finish=1538639070907210955,duration=58506952185

[00:20:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:23] expected success, got: exit code: 101
[00:20:23] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:20:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:23] Build completed unsuccessfully in 0:16:00
[00:20:23] Makefile:28: recipe for target 'all' failed
[00:20:23] Makefile:28: recipe for target 'all' failed
[00:20:23] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04a05cb0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
21968 ./.git/modules/src/tools/cargo/objects/pack
21140 ./src/llvm-emscripten/test/Transforms
20960 ./.git/modules/src/liblibc
travis_time:end:14851dfa:start=1538639071241900019,finish=1538639071694095525,duration=452195506
travis_fold:end:after_failurative/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:094cc080
$ dmesg | grep -i kill
