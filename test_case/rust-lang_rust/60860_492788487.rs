plain
travis_time:end:18742968:start=1557946922106419963,finish=1557946925734907099,duration=3628487136
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:04:25] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:26] tidy error: /checkout/src/libsyntax/attr/builtin.rs:348: TODO is deprecated; use FIXME
[00:04:26] tidy error: /checkout/src/libsyntax/attr/builtin.rs:360: TODO is deprecated; use FIXME
[00:04:26] tidy error: /checkout/src/libsyntax/error_codes.rs:435: TODO is deprecated; use FIXME
[00:04:31] some tidy checks failed
[00:04:31] 
[00:04:31] 
[00:04:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:31] 
[00:04:31] 
[00:04:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:31] Build completed unsuccessfully in 0:01:10
[00:04:31] Build completed unsuccessfully in 0:01:10
[00:04:31] Makefile:67: recipe for target 'tidy' failed
[00:04:31] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:022f81aa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 15 19:06:47 UTC 2019
---
travis_time:end:00694e00:start=1557947207892396626,finish=1557947207896841279,duration=4444653
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:083cf84a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2430b2e3
travis_time:start:2430b2e3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0097edde
$ dmesg | grep -i kill
