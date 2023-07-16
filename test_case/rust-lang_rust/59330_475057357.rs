plain
travis_time:end:014fda7c:start=1553120794077718353,finish=1553120879772457804,duration=85694739451
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---

[00:03:16] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:9: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:10: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:199: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:200: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:201: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:212: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:236: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:237: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:277: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:278: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:311: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:312: line longer than 100 chars
[00:03:16] tidy error: /checkout/src/libcore/convert.rs:313: line longer than 100 chars
[00:03:17] some tidy checks failed
[00:03:17] 
[00:03:17] 
[00:03:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:17] 
[00:03:17] 
[00:03:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:17] Build completed unsuccessfully in 0:00:44
[00:03:17] Build completed unsuccessfully in 0:00:44
[00:03:17] Makefile:67: recipe for target 'tidy' failed
[00:03:17] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d4c406a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Mar 20 22:31:26 UTC 2019
---
travis_time:end:2d247e6b:start=1553121086771121895,finish=1553121086776201799,duration=5079904
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1fc13f5e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00fe8140
travis_time:start:00fe8140
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:208c33f2
$ dmesg | grep -i kill
