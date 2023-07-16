plain
[00:04:35]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:36]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:37]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:38]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:42] error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
[00:04:42]      |
[00:04:42]      |
[00:04:42] 1044 |         MaybeUninit { value: unsafe { intrinsics::init() } }
[00:04:42] 
[00:04:45] error: aborting due to previous error
[00:04:45] 
[00:04:45] For more information about this error, try `rustc --explain E0015`.
[00:04:45] For more information about this error, try `rustc --explain E0015`.
[00:04:45] error: Could not compile `core`.
[00:04:45] warning: build failed, waiting for other jobs to finish...
[00:04:47] error: build failed
[00:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:47] expected success, got: exit code: 101
[00:04:47] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:04:47] travis_fold:end:stage0-std

[00:04:47] travis_time:end:stage0-std:start=1538752784643223586,finish=1538752809104880801,duration=24461657215


[00:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:47] Build completed unsuccessfully in 0:00:25
[00:04:47] Makefile:28: recipe for target 'all' failed
[00:04:47] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:058ec358
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:08a77faf:start=1538752809713581766,finish=1538752809720478920,duration=6897154
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:205e952e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11f26c0c
travis_time:start:11f26c0c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e62d467
$ dmesg | grep -i kill
