plain
travis_time:end:231cb2a2:start=1554048269416135751,finish=1554048270381977278,duration=965841527
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:04:20] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:21] tidy error: /checkout/src/librustc/ty/query/mod.rs:152: line longer than 100 chars
[00:04:22] some tidy checks failed
[00:04:22] 
[00:04:22] 
[00:04:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:22] 
[00:04:22] 
[00:04:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:22] Build completed unsuccessfully in 0:00:45
[00:04:22] Build completed unsuccessfully in 0:00:45
[00:04:22] Makefile:67: recipe for target 'tidy' failed
[00:04:22] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11b02e88
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 31 16:09:04 UTC 2019
---
travis_time:end:0dbf7fc4:start=1554048544721569252,finish=1554048544726293930,duration=4724678
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0374c117
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00463022
travis_time:start:00463022
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:038ab0f8
$ dmesg | grep -i kill
