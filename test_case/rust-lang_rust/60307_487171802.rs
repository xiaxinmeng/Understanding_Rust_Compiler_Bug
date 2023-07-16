plain
travis_time:end:0a096e33:start=1556305549262096979,finish=1556305550068333792,duration=806236813
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:34] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:35] tidy error: /checkout/src/librustdoc/html/render.rs:4142: line longer than 100 chars
[00:03:36] some tidy checks failed
[00:03:36] 
[00:03:36] 
[00:03:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:36] 
[00:03:36] 
[00:03:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:36] Build completed unsuccessfully in 0:00:48
[00:03:36] Build completed unsuccessfully in 0:00:48
[00:03:36] Makefile:67: recipe for target 'tidy' failed
[00:03:36] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1019e41a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 26 19:09:37 UTC 2019
---
travis_time:end:0ad595b4:start=1556305778124779372,finish=1556305778130119261,duration=5339889
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1e3c8ac9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08980251
travis_time:start:08980251
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09a39cb6
$ dmesg | grep -i kill
