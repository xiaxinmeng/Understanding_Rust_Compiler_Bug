plain
travis_time:end:2c28b2fa:start=1553116747728380361,finish=1553116832684845043,duration=84956464682
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:tidy
tidy check
[00:03:26] * 570 error codes
[00:03:26] * highest error code: E0725
[00:03:26] tidy error: /checkout/src/libstd/ffi/c_str.rs:374: malformed stability attribute
[00:03:27] some tidy checks failed
[00:03:27] 
[00:03:27] 
[00:03:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:27] 
[00:03:27] 
[00:03:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:27] Build completed unsuccessfully in 0:00:45
[00:03:27] Build completed unsuccessfully in 0:00:45
[00:03:27] Makefile:67: recipe for target 'tidy' failed
[00:03:27] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15b0b390
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 20 21:24:09 UTC 2019
---
travis_time:end:07b4f18e:start=1553117050281334060,finish=1553117050286103434,duration=4769374
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ae81d28
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:076ca21c
travis_time:start:076ca21c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01728810
$ dmesg | grep -i kill
