ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
[00:03:25] 
[00:03:26] some tidy checks failed
[00:03:26] 
[00:03:26] 
[00:03:26] 
[00:03:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:26] 
[00:03:26] 
[00:03:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:26] Build completed unsuccessfully in 0:00:46
[00:03:26] Build completed unsuccessfully in 0:00:46
[00:03:26] make: *** [tidy] Error 1
[00:03:26] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0011f503
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 21 20:35:08 UTC 2019
---
travis_time:end:0624ec66:start=1548102908829143724,finish=1548102908833488527,duration=4344803
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b0094cc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14eda240
travis_time:start:14eda240
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0128c4c6
$ dmesg | grep -i kill
