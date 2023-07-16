plain

[00:04:45] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:45] tidy error: /checkout/src/librustdoc/test.rs:751: TODO is deprecated; use FIXME
[00:04:47] some tidy checks failed
[00:04:47] 
[00:04:47] 
[00:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:47] 
[00:04:47] 
[00:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:47] Build completed unsuccessfully in 0:00:50
[00:04:47] Build completed unsuccessfully in 0:00:50
[00:04:47] make: *** [tidy] Error 1
[00:04:47] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00215ca4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1f8cfc20:start=1537547888764672862,finish=1537547888770798903,duration=6126041
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00112e2c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:027ae768
travis_time:start:027ae768
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01e611e5
$ dmesg | grep -i kill
