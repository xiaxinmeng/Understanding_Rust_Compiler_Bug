plain

[00:04:23] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:23] tidy error: /checkout/src/libstd/env.rs:515: line longer than 100 chars
[00:04:23] tidy error: /checkout/src/libstd/env.rs:516: line longer than 100 chars
[00:04:23] tidy error: /checkout/src/libstd/env.rs:517: line longer than 100 chars
[00:04:23] tidy error: /checkout/src/libstd/env.rs:522: line longer than 100 chars
[00:04:23] tidy error: /checkout/src/libstd/env.rs:523: line longer than 100 chars
[00:04:23] tidy error: /checkout/src/libstd/env.rs:524: line longer than 100 chars
[00:04:23] tidy error: /checkout/src/libstd/env.rs:539: line longer than 100 chars
[00:04:25] some tidy checks failed
[00:04:25] 
[00:04:25] 
[00:04:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:25] 
[00:04:25] 
[00:04:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:25] Build completed unsuccessfully in 0:01:30
[00:04:25] Build completed unsuccessfully in 0:01:30
[00:04:25] make: *** [tidy] Error 1
[00:04:25] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2787f61e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0362a9e0:start=1530806593995927365,finish=1530806594003376588,duration=7449223
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04e0f1c9
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03991a78
$ dmesg | grep -i kill
