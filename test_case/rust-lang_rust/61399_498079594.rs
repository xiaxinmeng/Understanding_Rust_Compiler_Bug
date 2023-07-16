plain
travis_time:end:2f1974a0:start=1559521258245974729,finish=1559521348320982267,duration=90075007538
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:01] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:01] tidy error: /checkout/src/librustc_mir/interpret/intrinsics/type_name.rs:231: line longer than 100 chars
[00:04:01] tidy error: /checkout/src/librustc_mir/interpret/intrinsics.rs:86: line longer than 100 chars
[00:04:06] some tidy checks failed
[00:04:06] 
[00:04:06] 
[00:04:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:06] 
[00:04:06] 
[00:04:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:06] Build completed unsuccessfully in 0:01:18
---
travis_time:end:1f0365d8:start=1559521604792524264,finish=1559521604797842009,duration=5317745
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2c4171ca
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19b71400
travis_time:start:19b71400
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00451b18
$ dmesg | grep -i kill
