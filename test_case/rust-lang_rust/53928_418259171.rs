plain
[01:33:50] 
[01:33:50] testing https://github.com/servo/servo
[01:33:50] Initialized empty Git repository in /checkout/obj/build/ct/servo/.git/
[01:33:50] fatal: Could not parse object '17e97b9320fdb7cdb33bbc5f4d0fde0653bbf2e4'.
[01:34:10] fatal: unable to access 'https://github.com/servo/servo/': Could not resolve host: github.com
[01:34:10] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[01:34:10] 
[01:34:10] 
[01:34:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:34:10] expected success, got: exit code: 101
[01:34:10] expected success, got: exit code: 101
[01:34:10] 
[01:34:10] 
[01:34:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:34:10] Build completed unsuccessfully in 0:35:49
[01:34:10] make: *** [check-aux] Error 1
[01:34:10] Makefile:60: recipe for target 'check-aux' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07732863
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00e10dc8:start=1536043573788894490,finish=1536043573795916053,duration=7021563
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00881aee
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15e09cce
travis_time:start:15e09cce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07db1f26
$ dmesg | grep -i kill
