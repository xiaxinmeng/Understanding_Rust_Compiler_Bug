plain
travis_time:end:0bfebb18:start=1559412660660794362,finish=1559412750288733176,duration=89627938814
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:55] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:56] tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:24: line longer than 100 chars
[00:03:56] tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:519: line longer than 100 chars
[00:03:56] tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:533: line longer than 100 chars
[00:03:56] tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:545: line longer than 100 chars
[00:03:56] tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:566: line longer than 100 chars
[00:03:56] tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:568: line longer than 100 chars
[00:04:00] some tidy checks failed
[00:04:00] 
[00:04:00] 
[00:04:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:00] 
[00:04:00] 
[00:04:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:00] Build completed unsuccessfully in 0:01:15
---
travis_time:end:010f3ab0:start=1559413001601943813,finish=1559413001607038958,duration=5095145
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e9f83b4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11dbe41a
travis_time:start:11dbe41a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11e9aa4c
$ dmesg | grep -i kill
