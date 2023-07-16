plain
[00:04:52]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:53]    Compiling alloc_system v0.0.0 (/checkout/src/liballoc_system)
[00:04:53]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:57]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:59] error[E0548]: incorrect stability attribute type
[00:04:59]      |
[00:04:59] 1043 |     #[rustc_const_unstable]
[00:04:59]      |     ^^^^^^^^^^^^^^^^^^^^^^^
[00:04:59] 
---
[00:05:01] 
[00:05:01] To learn more, run the command again with --verbose.
[00:05:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:05:01] expected success, got: exit code: 101
[00:05:01] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:05:01] travis_fold:end:stage0-std

[00:05:01] travis_time:end:stage0-std:start=1538479266326394735,finish=1538479303466363965,duration=37139969230


[00:05:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:01] Build completed unsuccessfully in 0:00:38
[00:05:01] make: *** [all] Error 1
[00:05:01] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08ccbdf5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:26a7a514:start=1538479304164944651,finish=1538479304170228523,duration=5283872
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:072432a7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03aefab2
travis_time:start:03aefab2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0965d8e0
$ dmesg | grep -i kill
