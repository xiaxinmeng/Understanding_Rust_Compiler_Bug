plain

[00:04:59] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:59] tidy error: /checkout/src/librustc/ty/query/plumbing.rs:663: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:107: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:115: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:145: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:173: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:192: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:195: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:204: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:206: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:218: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:223: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:228: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:244: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:252: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:260: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:269: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:270: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:277: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:278: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:291: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:292: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:293: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:294: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:321: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:339: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:357: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:365: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:371: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:386: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:399: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/ty/query/mod.rs:457: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/util/profiling.rs:96: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/util/profiling.rs:112: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/util/profiling.rs:113: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/util/profiling.rs:129: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/util/profiling.rs:212: trailing whitespace
[00:04:59] tidy error: /checkout/src/librustc/util/profiling.rs:214: trailing whitespace
[00:04:59] tidy error: /checkout/src/librustc/util/profiling.rs:226: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/util/profiling.rs:243: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/util/profiling.rs:248: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/util/profiling.rs:264: line longer than 100 chars
[00:05:01] some tidy checks failed
[00:05:01] 
[00:05:01] 
[00:05:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:01] 
[00:05:01] 
[00:05:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:01] Build completed unsuccessfully in 0:01:51
[00:05:01] Build completed unsuccessfully in 0:01:51
[00:05:01] Makefile:79: recipe for target 'tidy' failed
[00:05:01] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02e0b590
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2336de62:start=1529497634384776948,finish=1529497634390273095,duration=5496147
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13f31e20
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00007842
$ dmesg | grep -i kill
