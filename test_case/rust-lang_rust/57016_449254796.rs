plain
travis_time:end:21a7df5e:start=1545368610868652551,finish=1545368613711224017,duration=2842571466
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:02:52] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:02:52] tidy error: /checkout/src/bootstrap/test.rs:1997: line longer than 100 chars
[00:02:53] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:574: line longer than 100 chars
[00:02:53] tidy error: /checkout/src/tools/tidy/src/features.rs:351: line longer than 100 chars
[00:02:53] tidy error: /checkout/src/librustc/ty/query/job.rs:48: line longer than 100 chars
[00:02:54] some tidy checks failed
[00:02:54] 
[00:02:54] 
[00:02:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:02:54] 
[00:02:54] 
[00:02:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:02:54] Build completed unsuccessfully in 0:00:46
[00:02:54] Build completed unsuccessfully in 0:00:46
[00:02:54] Makefile:79: recipe for target 'tidy' failed
[00:02:54] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00328433
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 21 05:06:36 UTC 2018
---
travis_time:end:032d15e2:start=1545368797340628518,finish=1545368797346757630,duration=6129112
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f5b8161
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:043f5a7a
travis_time:start:043f5a7a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:053b1be4
$ dmesg | grep -i kill
