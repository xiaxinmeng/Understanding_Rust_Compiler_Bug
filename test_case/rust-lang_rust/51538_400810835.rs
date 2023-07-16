plain

[00:05:23] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:24] tidy error: /checkout/src/librustc/traits/query/type_op/eq.rs:30: line longer than 100 chars
[00:05:24] tidy error: /checkout/src/librustc/traits/query/type_op/prove_predicate.rs:31: line longer than 100 chars
[00:05:25] some tidy checks failed
[00:05:25] 
[00:05:25] 
[00:05:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:25] 
[00:05:25] 
[00:05:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:25] Build completed unsuccessfully in 0:02:00
[00:05:25] Build completed unsuccessfully in 0:02:00
[00:05:25] make: *** [tidy] Error 1
[00:05:25] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:052e86ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:004d2128:start=1530129231175073385,finish=1530129231181769909,duration=6696524
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00b9b42b
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16e5b99c
$ dmesg | grep -i kill
