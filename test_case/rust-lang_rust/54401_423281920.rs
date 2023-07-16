plain

[00:05:52] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:53] tidy error: /checkout/src/librustc/traits/select.rs:319: line longer than 100 chars
[00:05:54] some tidy checks failed
[00:05:54] 
[00:05:54] 
[00:05:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:54] 
[00:05:54] 
[00:05:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:54] Build completed unsuccessfully in 0:00:55
[00:05:54] Build completed unsuccessfully in 0:00:55
[00:05:54] Makefile:79: recipe for target 'tidy' failed
[00:05:54] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c40e0b5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0185ecc8:start=1537466793949573596,finish=1537466793953632315,duration=4058719
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0deeab0c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05c59895
travis_time:start:05c59895
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11891b20
$ dmesg | grep -i kill
