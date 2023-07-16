plain

[00:04:01] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:01] tidy error: /checkout/src/test/run-pass/impl-trait/xcrate-issue-50865.rs: missing trailing newline
[00:04:01] tidy error: /checkout/src/test/run-pass/impl-trait/auxiliary/xcrate.rs: missing trailing newline
[00:04:01] tidy error: /checkout/src/test/ui/impl-trait/issue-52128.rs: missing trailing newline
[00:04:02] some tidy checks failed
[00:04:02] 
[00:04:02] 
[00:04:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:02] 
[00:04:02] 
[00:04:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:02] Build completed unsuccessfully in 0:00:49
[00:04:02] Build completed unsuccessfully in 0:00:49
[00:04:02] Makefile:79: recipe for target 'tidy' failed
[00:04:02] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06479a06
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:031cb24c:start=1531492702666551123,finish=1531492702675265237,duration=8714114
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02958537
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03bc514f
$ dmesg | grep -i kill
