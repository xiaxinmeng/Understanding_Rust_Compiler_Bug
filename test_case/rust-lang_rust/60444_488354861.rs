plain
travis_time:end:07b81db5:start=1556731637715198881,finish=1556731638482573283,duration=767374402
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:02] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:02] tidy error: /checkout/src/librustc/traits/select.rs:874: line longer than 100 chars
[00:04:02] tidy error: /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:18: line longer than 100 chars
[00:04:02] tidy error: /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:24: line longer than 100 chars
[00:04:02] tidy error: /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:26: line longer than 100 chars
[00:04:02] tidy error: /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:27: line longer than 100 chars
[00:04:02] tidy error: /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:30: line longer than 100 chars
[00:04:04] some tidy checks failed
[00:04:04] 
[00:04:04] 
[00:04:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:04] 
[00:04:04] 
[00:04:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:04] Build completed unsuccessfully in 0:00:45
[00:04:04] Build completed unsuccessfully in 0:00:45
[00:04:04] make: *** [tidy] Error 1
[00:04:04] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1bafcee4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  1 17:31:34 UTC 2019
---
travis_time:end:1de6d40e:start=1556731895489899076,finish=1556731895494582882,duration=4683806
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13e8ac13
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04a67700
travis_time:start:04a67700
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:234bbfc2
$ dmesg | grep -i kill
