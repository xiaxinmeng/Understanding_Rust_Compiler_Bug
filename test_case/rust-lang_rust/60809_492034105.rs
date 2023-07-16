plain
travis_time:end:0f4baccb:start=1557792985941449991,finish=1557792988027273576,duration=2085823585
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:23] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:24] tidy error: duplicate error code: 724
[00:04:24] tidy error: /checkout/src/librustc_mir/error_codes.rs:2427: E0724: r##"
[00:04:24] tidy error: /checkout/src/librustc_typeck/error_codes.rs:4736:     E0724, // `#[ffi_returns_twice]` is only allowed in foreign functions
[00:04:28] some tidy checks failed
[00:04:28] 
[00:04:28] 
[00:04:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:28] 
[00:04:28] 
[00:04:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:28] Build completed unsuccessfully in 0:01:11
[00:04:28] Build completed unsuccessfully in 0:01:11
[00:04:28] make: *** [tidy] Error 1
[00:04:28] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:121b8388
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 14 00:21:08 UTC 2019
---
travis_time:end:03a5252a:start=1557793269352101740,finish=1557793269356789202,duration=4687462
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b27c0a6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10d81a04
travis_time:start:10d81a04
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d9f5a6e
$ dmesg | grep -i kill
