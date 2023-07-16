plain

[00:03:58] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:58] tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/mod.rs:1040: line longer than 100 chars
[00:03:59] some tidy checks failed
[00:03:59] 
[00:03:59] 
[00:03:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:59] 
[00:03:59] 
[00:03:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:59] Build completed unsuccessfully in 0:00:47
[00:03:59] Build completed unsuccessfully in 0:00:47
[00:03:59] make: *** [tidy] Error 1
[00:03:59] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:123090be
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2334fd00:start=1539804237265387512,finish=1539804237270303787,duration=4916275
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:017cf0df
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04f27be2
travis_time:start:04f27be2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12102b22
$ dmesg | grep -i kill
