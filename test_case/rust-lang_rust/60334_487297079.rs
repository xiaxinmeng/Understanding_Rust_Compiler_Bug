plain
travis_time:end:1714cc53:start=1556379477220789431,finish=1556379478075675524,duration=854886093
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:03] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:03] tidy error: /checkout/src/libstd/io/impls.rs:2: line longer than 100 chars
[00:04:03] tidy error: /checkout/src/libstd/io/buffered.rs:8: line longer than 100 chars
[00:04:03] tidy error: /checkout/src/libstd/io/cursor.rs:458: line longer than 100 chars
[00:04:03] tidy error: /checkout/src/libstd/io/cursor.rs:585: line longer than 100 chars
[00:04:03] tidy error: /checkout/src/libstd/io/cursor.rs:636: line longer than 100 chars
[00:04:03] tidy error: /checkout/src/libstd/io/cursor.rs:695: line longer than 100 chars
[00:04:05] some tidy checks failed
[00:04:05] 
[00:04:05] 
[00:04:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:05] 
[00:04:05] 
[00:04:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:05] Build completed unsuccessfully in 0:00:45
[00:04:05] Build completed unsuccessfully in 0:00:45
[00:04:05] make: *** [tidy] Error 1
[00:04:05] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12e23a64
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 27 15:42:15 UTC 2019
---
travis_time:end:0737a1ec:start=1556379737254543678,finish=1556379737259266627,duration=4722949
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18800070
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bd06f2c
travis_time:start:1bd06f2c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2d165cf4
$ dmesg | grep -i kill
