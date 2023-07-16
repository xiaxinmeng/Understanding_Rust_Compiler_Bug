plain

[00:03:49] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:49] tidy error: /checkout/src/test/ui/const-eval/issue-52442.rs: missing trailing newline
[00:03:49] tidy error: /checkout/src/test/ui/const-eval/issue-52443.rs: missing trailing newline
[00:03:51] some tidy checks failed
[00:03:51] 
[00:03:51] 
[00:03:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:51] 
[00:03:51] 
[00:03:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:51] Build completed unsuccessfully in 0:00:50
[00:03:51] Build completed unsuccessfully in 0:00:50
[00:03:51] make: *** [tidy] Error 1
[00:03:51] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f670bfa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:38dfd2b4:start=1532095644131302959,finish=1532095644137210548,duration=5907589
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a29630e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0277b504
travis_time:start:0277b504
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05b8c381
$ dmesg | grep -i kill
