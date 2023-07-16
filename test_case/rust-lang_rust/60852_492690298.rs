plain
travis_time:end:04d66049:start=1557931359719744091,finish=1557931452933773470,duration=93214029379
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
tidy check
[00:04:04] * 573 error codes
[00:04:04] * highest error code: E0728
[00:04:06] * 255 features
[00:04:06] tidy error: /checkout/src/libstd/sys_common/backtrace.rs:193: platform-specific cfg: cfg(windows)
[00:04:06] tidy error: /checkout/src/libstd/sys_common/backtrace.rs:196: platform-specific cfg: cfg(unix)
[00:04:06] tidy error: /checkout/src/libstd/sys_common/backtrace.rs:201: platform-specific cfg: cfg(not(unix))
[00:04:06] tidy error: /checkout/src/libstd/sys_common/backtrace.rs:205: platform-specific cfg: cfg(windows)
[00:04:06] tidy error: /checkout/src/libstd/sys_common/backtrace.rs:211: platform-specific cfg: cfg(not(windows))
[00:04:09] Dependencies not on the whitelist:
[00:04:09] * autocfg 
[00:04:09] invalid source: "git+https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9533bbdc27f299b1675191ec587ce05ed"
[00:04:09] invalid source: "git+https://github.com/alexcrichton/backtrace-rs?branch=include-in-std#d6d2d7a9533bbdc27f299b1675191ec587ce05ed"
[00:04:09] some tidy checks failed
[00:04:09] 
[00:04:09] 
[00:04:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:09] 
[00:04:09] 
[00:04:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:09] Build completed unsuccessfully in 0:01:13
[00:04:09] Build completed unsuccessfully in 0:01:13
[00:04:09] make: *** [tidy] Error 1
[00:04:09] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2ac14bb6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 15 14:48:31 UTC 2019
---
travis_time:end:04a172b4:start=1557931712836621474,finish=1557931712841378622,duration=4757148
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19a8edb6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0093e8b0
travis_time:start:0093e8b0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:026901cc
$ dmesg | grep -i kill
