plain

[00:03:40] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:40] tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/liveness.rs:79: line longer than 100 chars
[00:03:40] tidy error: /checkout/src/librustc_mir/util/liveness.rs:372: line longer than 100 chars
[00:03:41] some tidy checks failed
[00:03:41] 
[00:03:41] 
[00:03:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:41] 
[00:03:41] 
[00:03:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:41] Build completed unsuccessfully in 0:00:45
[00:03:41] Build completed unsuccessfully in 0:00:45
[00:03:41] Makefile:79: recipe for target 'tidy' failed
[00:03:41] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ca7cfcd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0bb57731:start=1531480643727623078,finish=1531480643734350747,duration=6727669
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ca8d216
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:193a7a90
$ dmesg | grep -i kill
