plain
travis_time:end:02400c06:start=1556199436669093120,finish=1556199437433886678,duration=764793558
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
[00:03:55] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:1456: line longer than 100 chars
[00:03:55] tidy error: /checkout/src/libsyntax/parse/unescape_error_reporting.rs:10: line longer than 100 chars
[00:03:55] tidy error: /checkout/src/libsyntax/parse/unescape_error_reporting.rs:57: TODO is deprecated; use FIXME
[00:03:57] some tidy checks failed
[00:03:57] 
[00:03:57] 
[00:03:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:57] 
[00:03:57] 
[00:03:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:57] Build completed unsuccessfully in 0:00:48
[00:03:57] Build completed unsuccessfully in 0:00:48
[00:03:57] Makefile:67: recipe for target 'tidy' failed
[00:03:57] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b5480ef
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 25 13:41:26 UTC 2019
---
travis_time:end:033982d0:start=1556199687689453400,finish=1556199687694864454,duration=5411054
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04cfeb68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06b813f7
travis_time:start:06b813f7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0401c513
$ dmesg | grep -i kill
