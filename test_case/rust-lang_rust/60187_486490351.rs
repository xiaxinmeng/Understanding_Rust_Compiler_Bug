plain
travis_time:end:1f485cf0:start=1556156481484382630,finish=1556156482401242044,duration=916859414
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:11] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:11] tidy error: /checkout/src/librustc_mir/transform/generator.rs:561: TODO is deprecated; use FIXME
[00:04:13] some tidy checks failed
[00:04:13] 
[00:04:13] 
[00:04:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:13] 
[00:04:13] 
[00:04:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:13] Build completed unsuccessfully in 0:00:45
[00:04:13] Build completed unsuccessfully in 0:00:45
[00:04:13] make: *** [tidy] Error 1
[00:04:13] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:201bfb6e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 25 01:45:47 UTC 2019
---
travis_time:end:02be76d9:start=1556156747911994846,finish=1556156747916916594,duration=4921748
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e908098
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:084779d0
travis_time:start:084779d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f7238cd
$ dmesg | grep -i kill
