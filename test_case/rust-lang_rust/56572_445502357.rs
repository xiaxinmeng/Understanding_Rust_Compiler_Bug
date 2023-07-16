plain
travis_time:end:17d98bb9:start=1544317390948378159,finish=1544317445314993435,duration=54366615276
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:09] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:09] tidy error: /checkout/src/librustc_resolve/lib.rs:3199: trailing whitespace
[00:03:09] tidy error: /checkout/src/librustc_resolve/lib.rs:3205: trailing whitespace
[00:03:09] tidy error: /checkout/src/librustc_resolve/lib.rs:3206: line longer than 100 chars
[00:03:09] tidy error: /checkout/src/librustc_resolve/lib.rs:3207: line longer than 100 chars
[00:03:09] tidy error: /checkout/src/librustc_resolve/lib.rs:3208: line longer than 100 chars
[00:03:09] tidy error: /checkout/src/librustc_resolve/lib.rs:3209: line longer than 100 chars
[00:03:09] tidy error: /checkout/src/librustc_resolve/lib.rs:3210: trailing whitespace
[00:03:11] some tidy checks failed
[00:03:11] 
[00:03:11] 
[00:03:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:11] 
[00:03:11] 
[00:03:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:11] Build completed unsuccessfully in 0:00:53
[00:03:11] Build completed unsuccessfully in 0:00:53
[00:03:11] make: *** [tidy] Error 1
[00:03:11] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1035f456
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec  9 01:07:25 UTC 2018
---
travis_time:end:181121f4:start=1544317645780872921,finish=1544317645785414302,duration=4541381
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1c873459
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:178c3167
travis_time:start:178c3167
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00257db2
$ dmesg | grep -i kill
