plain

[00:04:15] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:16] tidy error: /checkout/src/librustc_mir/interpret/memory.rs:316: line longer than 100 chars
[00:04:17] some tidy checks failed
[00:04:17] 
[00:04:17] 
[00:04:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:17] 
[00:04:17] 
[00:04:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:17] Build completed unsuccessfully in 0:00:46
[00:04:17] Build completed unsuccessfully in 0:00:46
[00:04:17] Makefile:79: recipe for target 'tidy' failed
[00:04:17] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ee7cef4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0765e580:start=1539160718047358045,finish=1539160718051869745,duration=4511700
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:013b5388
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2e92c71a
travis_time:start:2e92c71a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:021d74fd
$ dmesg | grep -i kill
