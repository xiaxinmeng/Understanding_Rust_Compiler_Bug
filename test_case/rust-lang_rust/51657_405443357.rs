plain

[00:04:10] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:11] tidy error: /checkout/src/librustc/ty/query/plumbing.rs:652: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/ty/query/mod.rs:133: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/ty/query/mod.rs:277: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/ty/query/mod.rs:353: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/util/profiling.rs:96: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/util/profiling.rs:112: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/util/profiling.rs:113: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/util/profiling.rs:129: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/util/profiling.rs:212: trailing whitespace
[00:04:11] tidy error: /checkout/src/librustc/util/profiling.rs:214: trailing whitespace
[00:04:11] tidy error: /checkout/src/librustc/util/profiling.rs:226: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/util/profiling.rs:243: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/util/profiling.rs:248: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/librustc/util/profiling.rs:264: line longer than 100 chars
[00:04:12] some tidy checks failed
[00:04:12] 
[00:04:12] 
[00:04:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:12] 
[00:04:12] 
[00:04:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:12] Build completed unsuccessfully in 0:00:55
[00:04:12] Build completed unsuccessfully in 0:00:55
[00:04:12] Makefile:79: recipe for target 'tidy' failed
[00:04:12] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1cc76eaf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1b0f422e:start=1531795613959748469,finish=1531795613966714832,duration=6966363
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0522ebe0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d692a7a
travis_time:start:0d692a7a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00fb8440
$ dmesg | grep -i kill
