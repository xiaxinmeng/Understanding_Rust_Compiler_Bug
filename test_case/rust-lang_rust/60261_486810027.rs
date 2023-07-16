plain
travis_time:end:10f74b48:start=1556220154917281636,finish=1556220157524918567,duration=2607636931
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:38] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:38] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:1456: line longer than 100 chars
[00:03:38] tidy error: /checkout/src/libsyntax/parse/unescape_error_reporting.rs:10: line longer than 100 chars
[00:03:38] tidy error: /checkout/src/libsyntax/parse/unescape_error_reporting.rs:57: TODO is deprecated; use FIXME
[00:03:40] some tidy checks failed
[00:03:40] 
[00:03:40] 
[00:03:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:40] 
[00:03:40] 
[00:03:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:40] Build completed unsuccessfully in 0:00:44
[00:03:40] Build completed unsuccessfully in 0:00:44
[00:03:40] Makefile:67: recipe for target 'tidy' failed
[00:03:40] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1600b5d4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 25 19:26:28 UTC 2019
---
travis_time:end:0c392e75:start=1556220389503326639,finish=1556220389507923003,duration=4596364
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23cccc3a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04596812
travis_time:start:04596812
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2d50cf26
$ dmesg | grep -i kill
