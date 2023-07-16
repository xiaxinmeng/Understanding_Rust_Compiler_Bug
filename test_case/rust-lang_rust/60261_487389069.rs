plain
travis_time:end:065c394c:start=1556464050685635777,finish=1556464144389422280,duration=93703786503
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:32] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:32] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:1181: line longer than 100 chars
[00:03:32] tidy error: /checkout/src/libsyntax/parse/unescape_error_reporting.rs:10: line longer than 100 chars
[00:03:32] tidy error: /checkout/src/libsyntax/parse/unescape_error_reporting.rs:57: TODO is deprecated; use FIXME
[00:03:34] some tidy checks failed
[00:03:34] 
[00:03:34] 
[00:03:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:34] 
[00:03:34] 
[00:03:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:34] Build completed unsuccessfully in 0:00:44
[00:03:34] Build completed unsuccessfully in 0:00:44
[00:03:34] make: *** [tidy] Error 1
[00:03:34] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:075162db
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 28 15:12:48 UTC 2019
---
travis_time:end:27d9f214:start=1556464368987353964,finish=1556464368991966173,duration=4612209
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05d3e0d4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19a1c122
travis_time:start:19a1c122
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2260a790
$ dmesg | grep -i kill
