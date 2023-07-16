plain

[00:05:05] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:06] tidy error: /checkout/src/librustc/traits/query/type_op/outlives.rs:39: line longer than 100 chars
[00:05:06] tidy error: /checkout/src/librustc/traits/query/type_op/subtype.rs:36: line longer than 100 chars
[00:05:06] tidy error: /checkout/src/librustc/traits/query/type_op/normalize.rs:39: line longer than 100 chars
[00:05:06] tidy error: /checkout/src/librustc/traits/query/type_op/eq.rs:32: line longer than 100 chars
[00:05:06] tidy error: /checkout/src/librustc/traits/query/type_op/prove_predicate.rs:34: line longer than 100 chars
[00:05:06] tidy error: /checkout/src/librustc/traits/query/type_op/mod.rs:48: line longer than 100 chars
[00:05:07] some tidy checks failed
[00:05:07] 
[00:05:07] 
[00:05:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:07] 
[00:05:07] 
[00:05:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:07] Build completed unsuccessfully in 0:01:54
[00:05:07] Build completed unsuccessfully in 0:01:54
[00:05:07] Makefile:79: recipe for target 'tidy' failed
[00:05:07] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0002b818
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0c8d2a91:start=1530124523588195390,finish=1530124523593770163,duration=5574773
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:042229ee
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1c149576
$ dmesg | grep -i kill
