plain
[00:58:00] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:00]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:58:03] error: unused import: `std::iter`
[00:58:03]   --> libcore/../libcore/tests/num/dec2flt/parse.rs:11:5
[00:58:03] 11 | use std::iter;
[00:58:03]    |     ^^^^^^^^^
[00:58:03]    |
[00:58:03]    = note: `-D unused-imports` implied by `-D warnings`
---
[00:58:14] 
[00:58:14] To learn more, run the command again with --verbose.
[00:58:14] 
[00:58:14] 
[00:58:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[00:58:14] 
[00:58:14] 
[00:58:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:14] Build completed unsuccessfully in 0:16:54
[00:58:14] Build completed unsuccessfully in 0:16:54
[00:58:14] make: *** [check] Error 1
[00:58:14] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0760c78a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:06cee71c:start=1532699005827470738,finish=1532699005836909340,duration=9438602
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c41f6be
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0005f07c
travis_time:start:0005f07c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0151841a
$ dmesg | grep -i kill
