plain
travis_time:end:36405690:start=1559030288828577675,finish=1559030291031888498,duration=2203310823
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:05:00] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:00] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:1322: line longer than 100 chars
[00:05:05] some tidy checks failed
[00:05:05] 
[00:05:05] 
[00:05:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:05] 
[00:05:05] 
[00:05:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:05] Build completed unsuccessfully in 0:01:16
[00:05:05] Build completed unsuccessfully in 0:01:16
[00:05:05] Makefile:67: recipe for target 'tidy' failed
[00:05:05] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0012c9a5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 28 08:03:27 UTC 2019
---
travis_time:end:00c28791:start=1559030608193513475,finish=1559030608198934983,duration=5421508
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00bc8180
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:186483a9
travis_time:start:186483a9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17d1532a
$ dmesg | grep -i kill
