plain
[02:39:13] fatal: Could not parse object '21c7dae29c3c214c08533c2a55ac649b418f2fe3'.
[02:39:14] From https://github.com/iron/iron
[02:39:14]  * branch            master     -> FETCH_HEAD
[02:39:14] fatal: Could not parse object '21c7dae29c3c214c08533c2a55ac649b418f2fe3'.
[02:42:47] fatal: unable to access 'https://github.com/iron/iron/': Failed to connect to github.com port 443: Connection timed out
[02:42:47] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[02:42:47] 
[02:42:47] 
[02:42:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[02:42:47] expected success, got: exit code: 101
[02:42:47] expected success, got: exit code: 101
[02:42:47] 
[02:42:47] 
[02:42:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[02:42:47] Build completed unsuccessfully in 1:17:50
[02:42:47] Makefile:60: recipe for target 'check-aux' failed
[02:42:47] make: *** [check-aux] Error 1
Checking DNS for travis-ci/travis-ci#9696! --------------------------
[02:42:47] travis_fold:start:resolv-conf
cat /etc/resolv.conf --------------------------------------------
[02:42:47] search c.travis-ci-prod-4.internal google.internal
---
travis_time:end:18eb5ba0:start=1533981868849396638,finish=1533981868865837433,duration=16440795
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12af9b5a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1af16c98
travis_time:start:1af16c98
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:014e1e26
$ dmesg | grep -i kill
