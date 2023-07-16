plain

[00:04:46] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:47] tidy error: /checkout/src/librustc_mir/interpret/place.rs:547: line longer than 100 chars
[00:04:47] tidy error: /checkout/src/librustc_mir/interpret/place.rs:779: line longer than 100 chars
[00:04:47] tidy error: /checkout/src/librustc_mir/interpret/eval_context.rs:234: line longer than 100 chars
[00:04:47] tidy error: /checkout/src/librustc_mir/interpret/eval_context.rs:518: line longer than 100 chars
[00:04:48] some tidy checks failed
[00:04:48] 
[00:04:48] 
[00:04:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:48] 
[00:04:48] 
[00:04:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:48] Build completed unsuccessfully in 0:00:52
[00:04:48] Build completed unsuccessfully in 0:00:52
[00:04:48] Makefile:79: recipe for target 'tidy' failed
[00:04:48] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03117e61
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00b5cf7e:start=1537613176987574104,finish=1537613176993261326,duration=5687222
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24bd0074
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03ccfc2c
travis_time:start:03ccfc2c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00c811a8
$ dmesg | grep -i kill
