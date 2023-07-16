plain

[00:06:50] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:06:50] tidy error: /checkout/src/liballoc/boxed.rs:754: trailing whitespace
[00:06:52] some tidy checks failed
[00:06:52] 
[00:06:52] 
[00:06:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:52] 
[00:06:52] 
[00:06:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:52] Build completed unsuccessfully in 0:00:51
[00:06:52] Build completed unsuccessfully in 0:00:51
[00:06:52] make: *** [tidy] Error 1
[00:06:52] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:090e55ea
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:14559ba0:start=1536184653303742405,finish=1536184653312308198,duration=8565793
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16a90cbb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:383c0220
travis_time:start:383c0220
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2f34c2f1
$ dmesg | grep -i kill
