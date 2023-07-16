plain

[00:05:19] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:19] tidy error: /checkout/src/test/ui/nll/enum-drop-access.rs: missing trailing newline
[00:05:20] some tidy checks failed
[00:05:20] 
[00:05:20] 
[00:05:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:20] 
[00:05:20] 
[00:05:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:20] Build completed unsuccessfully in 0:00:55
[00:05:20] Build completed unsuccessfully in 0:00:55
[00:05:20] make: *** [tidy] Error 1
[00:05:20] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1802539e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:23d0eee0:start=1537717195640387573,finish=1537717195645207774,duration=4820201
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00434937
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:164677b0
travis_time:start:164677b0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1153b28b
$ dmesg | grep -i kill
