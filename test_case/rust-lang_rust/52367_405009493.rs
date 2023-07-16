plain

[00:03:35] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:35] tidy error: /checkout/src/test/compile-fail-fulldeps/proc-macro/issue-52329.rs: missing trailing newline
[00:03:37] some tidy checks failed
[00:03:37] 
[00:03:37] 
[00:03:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:37] 
[00:03:37] 
[00:03:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:37] Build completed unsuccessfully in 0:00:46
[00:03:37] Build completed unsuccessfully in 0:00:46
[00:03:37] Makefile:79: recipe for target 'tidy' failed
[00:03:37] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17b37166
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:22f7f41f:start=1531557635921704885,finish=1531557635928571408,duration=6866523
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01a3865c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ba1ace2
travis_time:start:0ba1ace2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:032a079e
$ dmesg | grep -i kill
