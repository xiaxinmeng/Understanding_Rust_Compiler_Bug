plain
travis_time:end:09abd4fa:start=1557698943521009521,finish=1557698944929801356,duration=1408791835
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:54] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:55] tidy error: /checkout/src/liballoc/raw_vec.rs:323: line longer than 100 chars
[00:04:55] tidy error: /checkout/src/liballoc/alloc.rs:150: trailing whitespace
[00:05:00] some tidy checks failed
[00:05:00] 
[00:05:00] 
[00:05:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:00] 
[00:05:00] 
[00:05:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:00] Build completed unsuccessfully in 0:01:14
[00:05:00] Build completed unsuccessfully in 0:01:14
[00:05:00] make: *** [tidy] Error 1
[00:05:00] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d7349ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun May 12 22:14:16 UTC 2019
---
travis_time:end:151265c8:start=1557699257128049056,finish=1557699257133325065,duration=5276009
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26d00224
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:325e2e28
travis_time:start:325e2e28
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure
