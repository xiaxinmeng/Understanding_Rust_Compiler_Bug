plain
travis_time:end:156c2314:start=1557849552930384025,finish=1557849555038429287,duration=2108045262
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:10:33] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:10:33] tidy error: /checkout/src/libsyntax/parse/parser.rs:2631: line longer than 100 chars
[00:10:38] some tidy checks failed
[00:10:38] 
[00:10:38] 
[00:10:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:10:38] 
[00:10:38] 
[00:10:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:10:38] Build completed unsuccessfully in 0:01:15
[00:10:38] Build completed unsuccessfully in 0:01:15
[00:10:38] Makefile:67: recipe for target 'tidy' failed
[00:10:38] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d41291b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 14 16:10:04 UTC 2019
---
travis_time:end:03b0489b:start=1557850205817302229,finish=1557850205822784929,duration=5482700
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0925ba08
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:024d40e6
travis_time:start:024d40e6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0af3dc00
$ dmesg | grep -i kill
