plain

[00:04:52] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:52] tidy error: /checkout/src/librustc_typeck/check/method/probe.rs:261: XXX is deprecated; use FIXME
[00:04:52] tidy error: /checkout/src/librustc_typeck/check/method/probe.rs:265: XXX is deprecated; use FIXME
[00:04:52] tidy error: /checkout/src/librustc_typeck/check/method/probe.rs:268: line longer than 100 chars
[00:04:52] tidy error: /checkout/src/librustc_typeck/check/method/probe.rs:327: line longer than 100 chars
[00:04:52] tidy error: /checkout/src/librustc_typeck/check/method/probe.rs:422: line longer than 100 chars
[00:04:52] tidy error: /checkout/src/librustc_typeck/check/method/probe.rs:519: line longer than 100 chars
[00:04:52] tidy error: /checkout/src/librustc_typeck/check/method/probe.rs:696: line longer than 100 chars
[00:04:52] tidy error: /checkout/src/librustc_typeck/check/method/probe.rs:1020: line longer than 100 chars
[00:04:53] some tidy checks failed
[00:04:53] 
[00:04:53] 
[00:04:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:53] 
[00:04:53] 
[00:04:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:53] Build completed unsuccessfully in 0:00:54
[00:04:53] Build completed unsuccessfully in 0:00:54
[00:04:53] Makefile:79: recipe for target 'tidy' failed
[00:04:53] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27d13364
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:004b1b06:start=1537132315822987460,finish=1537132315827473271,duration=4485811
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f80f856
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07800a78
travis_time:start:07800a78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10abce7c
$ dmesg | grep -i kill
