plain
travis_time:end:0d18d2ba:start=1556383305947931674,finish=1556383391429402047,duration=85481470373
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:20] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:20] tidy error: /checkout/src/librustc/traits/error_reporting.rs:719: line longer than 100 chars
[00:03:20] tidy error: /checkout/src/librustc/ich/impls_ty.rs:59: line longer than 100 chars
[00:03:20] tidy error: /checkout/src/librustc/ty/instance.rs:462: line longer than 100 chars
[00:03:20] tidy error: /checkout/src/librustc/ty/context.rs:575: line longer than 100 chars
[00:03:22] some tidy checks failed
[00:03:22] 
[00:03:22] 
[00:03:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:22] 
[00:03:22] 
[00:03:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:22] Build completed unsuccessfully in 0:00:47
[00:03:22] Build completed unsuccessfully in 0:00:47
[00:03:22] make: *** [tidy] Error 1
[00:03:22] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d4580e4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 27 16:46:43 UTC 2019
---
travis_time:end:0a130e57:start=1556383604256340974,finish=1556383604261698985,duration=5358011
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2e835b98
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:023dcdaa
travis_time:start:023dcdaa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12c6fe4e
$ dmesg | grep -i kill
