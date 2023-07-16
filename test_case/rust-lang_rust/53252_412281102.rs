plain
[02:45:48] fatal: Could not parse object '5e11c4852fe4aa086b0e4fe5885822fbe57ba928'.
[02:47:55] From https://github.com/Aaronepower/tokei
[02:47:55]  * branch            master     -> FETCH_HEAD
[02:47:55] fatal: Could not parse object '5e11c4852fe4aa086b0e4fe5885822fbe57ba928'.
[02:51:32] fatal: unable to access 'https://github.com/Aaronepower/tokei/': Failed to connect to github.com port 443: Connection timed out
[02:51:32] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[02:51:32] 
[02:51:32] 
[02:51:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[02:51:32] expected success, got: exit code: 101
[02:51:32] expected success, got: exit code: 101
[02:51:32] 
[02:51:32] 
[02:51:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[02:51:32] Build completed unsuccessfully in 1:35:25
[02:51:32] Makefile:60: recipe for target 'check-aux' failed
[02:51:32] make: *** [check-aux] Error 1
Checking DNS for travis-ci/travis-ci#9696! --------------------------
[02:51:32] travis_fold:start:resolv-conf
cat /etc/resolv.conf --------------------------------------------
[02:51:32] search c.travis-ci-prod-4.internal google.internal
---
travis_time:end:1b8e27e3:start=1534000114986913808,finish=1534000115003435015,duration=16521207
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:015c2170
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03177372
travis_time:start:03177372
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:34c92423
$ dmesg | grep -i kill
