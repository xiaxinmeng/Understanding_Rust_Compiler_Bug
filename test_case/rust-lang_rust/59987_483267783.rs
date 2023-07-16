plain
travis_time:end:18021cbb:start=1555336490298707472,finish=1555336602194786422,duration=111896078950
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:23] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:23] tidy error: /checkout/src/librustc_mir/borrow_check/nll/explain_borrow/mod.rs:584: line longer than 100 chars
[00:03:23] tidy error: /checkout/src/librustc_mir/monomorphize/collector.rs:532: line longer than 100 chars
[00:03:25] some tidy checks failed
[00:03:25] 
[00:03:25] 
[00:03:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:25] 
[00:03:25] 
[00:03:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:25] Build completed unsuccessfully in 0:00:46
[00:03:25] Build completed unsuccessfully in 0:00:46
[00:03:25] Makefile:67: recipe for target 'tidy' failed
[00:03:25] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01968ed0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 15 14:00:16 UTC 2019
---
travis_time:end:01027adc:start=1555336817305012439,finish=1555336817309712140,duration=4699701
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:140757dc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07477398
travis_time:start:07477398
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a6037b5
$ dmesg | grep -i kill
