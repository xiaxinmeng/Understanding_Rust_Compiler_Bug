ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
[00:03:15] 
[00:03:16] some tidy checks failed
[00:03:16] 
[00:03:16] 
[00:03:16] 
[00:03:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:16] 
[00:03:16] 
[00:03:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:16] Build completed unsuccessfully in 0:00:46
[00:03:16] Build completed unsuccessfully in 0:00:46
[00:03:16] make: *** [tidy] Error 1
[00:03:16] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02650359
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 19 19:32:33 UTC 2019
---
travis_time:end:01126e6c:start=1550604754520141307,finish=1550604754524534932,duration=4393625
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08ddc0d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1af8b3d8
travis_time:start:1af8b3d8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01ce34fb
$ dmesg | grep -i kill
