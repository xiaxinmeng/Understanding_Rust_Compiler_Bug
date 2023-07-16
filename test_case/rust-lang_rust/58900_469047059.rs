plain
travis_time:end:3014fc89:start=1551634956566190579,finish=1551634957417205320,duration=851014741
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:04:00] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:00] tidy error: /checkout/src/bootstrap/config.rs:658: line longer than 100 chars
[00:04:02] some tidy checks failed
[00:04:02] 
[00:04:02] 
[00:04:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:02] 
[00:04:02] 
[00:04:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:02] Build completed unsuccessfully in 0:00:47
[00:04:02] Build completed unsuccessfully in 0:00:47
[00:04:02] Makefile:68: recipe for target 'tidy' failed
[00:04:02] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00eb4e50
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar  3 17:46:50 UTC 2019
---
travis_time:end:0705dc00:start=1551635211476902897,finish=1551635211482467540,duration=5564643
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:21fac980
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ada93cc
travis_time:start:1ada93cc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03c23c64
$ dmesg | grep -i kill
