plain
[00:03:46]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:03:50] error[E0544]: multiple stability levels
[00:03:50]   --> libcore/time.rs:42:1
[00:03:50]    |
[00:03:50] 42 | #[unstable(feature = "time_units", issue = "0")]
[00:03:50] 
[00:03:52]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:03:52]    Compiling cmake v0.1.31
[00:03:52]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
---
[00:04:00] warning: build failed, waiting for other jobs to finish...
[00:04:09] error: build failed
[00:04:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:09] expected success, got: exit code: 101
[00:04:09] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:04:09] travis_fold:end:stage0-std

[00:04:09] travis_time:end:stage0-std:start=1532047805421941556,finish=1532047828482829589,duration=23060888033


[00:04:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:09] Build completed unsuccessfully in 0:00:24
[00:04:09] Makefile:28: recipe for target 'all' failed
[00:04:09] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1a399f84
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2578e501:start=1532047828950354602,finish=1532047828956191404,duration=5836802
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c1ea860
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bb205d8
travis_time:start:1bb205d8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02ba2d0c
$ dmesg | grep -i kill
