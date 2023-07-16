ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
[00:03:37] 
[00:03:38] some tidy checks failed
[00:03:38] 
[00:03:38] 
[00:03:38] 
[00:03:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:38] 
[00:03:38] 
[00:03:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:38] Build completed unsuccessfully in 0:00:49
[00:03:38] Build completed unsuccessfully in 0:00:49
[00:03:38] Makefile:68: recipe for target 'tidy' failed
[00:03:38] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04f7e0d8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb  4 10:18:10 UTC 2019
---
travis_time:end:04814e5a:start=1549275490930779311,finish=1549275490935881652,duration=5102341
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06f351b7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c4dcb82
travis_time:start:1c4dcb82
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:048eec80
$ dmesg | grep -i kill
