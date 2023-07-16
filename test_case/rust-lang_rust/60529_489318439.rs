plain
travis_time:end:04662d46:start=1556968104256625746,finish=1556968188989629572,duration=84733003826
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:56] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:57] tidy error: /checkout/src/test/ui/rfc-2008-non-exhaustive/uninhabited/indirect_match_with_exhaustive_patterns_same_crate.rs: leading newline
[00:04:01] some tidy checks failed
[00:04:01] 
[00:04:01] 
[00:04:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:01] 
[00:04:01] 
[00:04:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:01] Build completed unsuccessfully in 0:01:04
[00:04:01] Build completed unsuccessfully in 0:01:04
[00:04:01] Makefile:67: recipe for target 'tidy' failed
[00:04:01] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10105968
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May  4 11:13:59 UTC 2019
---
travis_time:end:03c55834:start=1556968440252678166,finish=1556968440257504463,duration=4826297
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d5d466c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17243bf4
travis_time:start:17243bf4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05f2cfa0
$ dmesg | grep -i kill
