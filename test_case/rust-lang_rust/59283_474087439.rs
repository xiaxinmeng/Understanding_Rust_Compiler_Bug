plain
travis_time:end:1610fe8c:start=1552939996582639211,finish=1552939997546939179,duration=964299968
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:05:09] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:09] tidy error: /checkout/src/libcore/benches/ascii_case.rs:148: line longer than 100 chars
[00:05:09] tidy error: /checkout/src/libcore/benches/ascii_case.rs:150: line longer than 100 chars
[00:05:11] some tidy checks failed
[00:05:11] 
[00:05:11] 
[00:05:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:11] 
[00:05:11] 
[00:05:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:11] Build completed unsuccessfully in 0:00:43
[00:05:11] Build completed unsuccessfully in 0:00:43
[00:05:11] make: *** [tidy] Error 1
[00:05:11] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f6f1c5c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 18 20:18:40 UTC 2019
---
travis_time:end:000a3bdf:start=1552940321027716542,finish=1552940321032773778,duration=5057236
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:050a3420
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1fd11106
travis_time:start:1fd11106
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01ef92e5
$ dmesg | grep -i kill
