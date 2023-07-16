plain
tidy check
[00:04:34] * 555 error codes
[00:04:34] * highest error code: E0712
[00:04:35] * 226 features
[00:04:35] tidy error: The Unstable Book has a 'language feature' section 'infer-outlives-requirements' which doesn't correspond to an unstable language feature
[00:04:35] some tidy checks failed
[00:04:35] 
[00:04:35] 
[00:04:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:35] 
[00:04:35] 
[00:04:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:35] Build completed unsuccessfully in 0:00:49
[00:04:35] Build completed unsuccessfully in 0:00:49
[00:04:35] make: *** [tidy] Error 1
[00:04:35] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1424ceed
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2e76080c:start=1535684828421418062,finish=1535684828429210744,duration=7792682
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07d2ecb0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1dd6bbd2
travis_time:start:1dd6bbd2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00186e5f
$ dmesg | grep -i kill
