plain
[01:47:50] 
[01:47:50] testing https://github.com/BurntSushi/xsv
[01:47:51] Initialized empty Git repository in /checkout/obj/build/ct/xsv/.git/
[01:47:51] fatal: Could not parse object '66956b6bfd62d6ac767a6b6499c982eae20a2c9f'.
[01:48:11] fatal: unable to access 'https://github.com/BurntSushi/xsv/': Could not resolve host: github.com
[01:48:11] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[01:48:11] 
[01:48:11] 
[01:48:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:48:11] expected success, got: exit code: 101
[01:48:11] expected success, got: exit code: 101
[01:48:11] 
[01:48:11] 
[01:48:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:48:11] Build completed unsuccessfully in 0:45:19
[01:48:11] make: *** [check-aux] Error 1
[01:48:11] Makefile:60: recipe for target 'check-aux' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00497e4e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1a5fffba:start=1534574918091265952,finish=1534574918097774891,duration=6508939
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0743de88
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02594940
travis_time:start:02594940
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14a46a05
$ dmesg | grep -i kill
