plain
travis_time:end:05b0c87b:start=1558539975405021652,finish=1558539977726465227,duration=2321443575
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:36] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:37] tidy error: duplicate error code: 727
[00:04:37] tidy error: /checkout/src/librustc_typeck/error_codes.rs:4650: E0727: r##"
[00:04:37] tidy error: /checkout/src/librustc/error_codes.rs:2208:     E0727, // `async` generators are not yet supported
[00:04:42] some tidy checks failed
[00:04:42] 
[00:04:42] 
[00:04:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:42] 
[00:04:42] 
[00:04:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:42] Build completed unsuccessfully in 0:01:12
[00:04:42] Build completed unsuccessfully in 0:01:12
[00:04:42] Makefile:67: recipe for target 'tidy' failed
[00:04:42] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00cd3260
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 22 15:51:10 UTC 2019
---
travis_time:end:05fc766d:start=1558540271246115582,finish=1558540271250803029,duration=4687447
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15d54464
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04ef9049
travis_time:start:04ef9049
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00adf57a
$ dmesg | grep -i kill
