plain
travis_time:end:0089e00e:start=1549211764138734719,finish=1549211847032886163,duration=82894151444
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:31] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:32] tidy error: /checkout/src/librustdoc/html/render.rs:2519: line longer than 100 chars
[00:03:32] tidy error: /checkout/src/librustdoc/clean/auto_trait.rs:223: line longer than 100 chars
[00:03:32] tidy error: /checkout/src/librustdoc/clean/mod.rs:3384: line longer than 100 chars
[00:03:32] tidy error: /checkout/src/librustdoc/clean/mod.rs:3395: line longer than 100 chars
[00:03:32] tidy error: /checkout/src/librustdoc/clean/mod.rs:3957: line longer than 100 chars
[00:03:32] tidy error: /checkout/src/librustdoc/clean/inline.rs:38: line longer than 100 chars
[00:03:32] tidy error: /checkout/src/librustdoc/clean/inline.rs:387: line longer than 100 chars
[00:03:33] some tidy checks failed
[00:03:33] 
[00:03:33] 
[00:03:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:33] 
[00:03:33] 
[00:03:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:33] Build completed unsuccessfully in 0:00:46
[00:03:33] Build completed unsuccessfully in 0:00:46
[00:03:33] Makefile:68: recipe for target 'tidy' failed
[00:03:33] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2051b140
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 16:41:16 UTC 2019
---
travis_time:end:0e21dd15:start=1549212076985733491,finish=1549212076990731950,duration=4998459
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02e954a4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23af1ae8
travis_time:start:23af1ae8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04936b83
$ dmesg | grep -i kill
