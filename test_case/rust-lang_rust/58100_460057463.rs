plain
travis_time:end:094aedaa:start=1549202892805446078,finish=1549202893773528562,duration=968082484
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:11] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:11] tidy error: /checkout/src/librustdoc/html/render.rs:2519: line longer than 100 chars
[00:03:11] tidy error: /checkout/src/librustdoc/clean/auto_trait.rs:223: line longer than 100 chars
[00:03:11] tidy error: /checkout/src/librustdoc/clean/mod.rs:3384: line longer than 100 chars
[00:03:11] tidy error: /checkout/src/librustdoc/clean/mod.rs:3395: line longer than 100 chars
[00:03:11] tidy error: /checkout/src/librustdoc/clean/mod.rs:3957: line longer than 100 chars
[00:03:11] tidy error: /checkout/src/librustdoc/clean/inline.rs:38: line longer than 100 chars
[00:03:11] tidy error: /checkout/src/librustdoc/clean/inline.rs:387: line longer than 100 chars
[00:03:13] some tidy checks failed
[00:03:13] 
[00:03:13] 
[00:03:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:13] 
[00:03:13] 
[00:03:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:13] Build completed unsuccessfully in 0:00:45
[00:03:13] Build completed unsuccessfully in 0:00:45
[00:03:13] make: *** [tidy] Error 1
[00:03:13] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bc542a6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 14:11:37 UTC 2019
---
travis_time:end:0722d400:start=1549203098534417559,finish=1549203098539134475,duration=4716916
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2c8917ed
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04b3b334
travis_time:start:04b3b334
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2776be0c
$ dmesg | grep -i kill
