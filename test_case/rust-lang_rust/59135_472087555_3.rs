ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
[00:03:17] 
[00:03:19] some tidy checks failed
[00:03:19] 
[00:03:19] 
[00:03:19] 
[00:03:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:19] 
[00:03:19] 
[00:03:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:19] Build completed unsuccessfully in 0:00:43
[00:03:19] Build completed unsuccessfully in 0:00:43
[00:03:19] make: *** [tidy] Error 1
[00:03:19] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:253e07c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 12 16:44:33 UTC 2019
---
travis_time:end:047a7719:start=1552409074661128220,finish=1552409074665691577,duration=4563357
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23310e57
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:001962ab
travis_time:start:001962ab
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:005046b0
$ dmesg | grep -i kill
