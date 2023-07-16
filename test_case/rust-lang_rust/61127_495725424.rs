plain
travis_time:end:0758c495:start=1558718523751343877,finish=1558718641223458793,duration=117472114916
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:04] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:05] tidy error: /checkout/src/librustc_mir/interpret/place.rs:624: line longer than 100 chars
[00:04:10] some tidy checks failed
[00:04:10] 
[00:04:10] 
[00:04:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:10] 
[00:04:10] 
[00:04:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:10] Build completed unsuccessfully in 0:01:15
[00:04:10] Build completed unsuccessfully in 0:01:15
[00:04:10] make: *** [tidy] Error 1
[00:04:10] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08d6cb0a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 24 17:28:21 UTC 2019
---
travis_time:end:248fa9ab:start=1558718902010591003,finish=1558718902015807885,duration=5216882
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03888abc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:083998ee
travis_time:start:083998ee
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1db7c688
$ dmesg | grep -i kill
