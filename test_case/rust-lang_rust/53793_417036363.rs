plain
tidy check
[00:04:47] * 555 error codes
[00:04:47] * highest error code: E0712
[00:04:47] * 225 features
[00:04:48] tidy error: The Unstable Book has a 'language feature' section 'infer-outlives-requirements' which doesn't correspond to an unstable language feature
[00:04:48] some tidy checks failed
[00:04:48] 
[00:04:48] 
[00:04:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:48] 
[00:04:48] 
[00:04:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:48] Build completed unsuccessfully in 0:00:50
[00:04:48] Build completed unsuccessfully in 0:00:50
[00:04:48] make: *** [tidy] Error 1
[00:04:48] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:056ffba8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:050487c4:start=1535563088658206566,finish=1535563088665401042,duration=7194476
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18fbc19e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e493ae2
travis_time:start:1e493ae2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:19414edb
$ dmesg | grep -i kill
