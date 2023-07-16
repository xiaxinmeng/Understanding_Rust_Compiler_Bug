plain
travis_time:end:0467255a:start=1548689060750712466,finish=1548689063994766076,duration=3244053610
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:07:18] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:07:18] tidy error: /checkout/src/librustc_metadata/creader.rs:371: tab character
[00:07:18] tidy error: /checkout/src/librustc_metadata/creader.rs:371: trailing whitespace
[00:07:20] some tidy checks failed
[00:07:20] 
[00:07:20] 
[00:07:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:07:20] 
[00:07:20] 
[00:07:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:07:20] Build completed unsuccessfully in 0:00:45
[00:07:20] Build completed unsuccessfully in 0:00:45
[00:07:20] make: *** [tidy] Error 1
[00:07:20] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b4a82ef
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 28 15:31:55 UTC 2019
---
travis_time:end:0f8b3400:start=1548689516623578081,finish=1548689516628522641,duration=4944560
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:038748c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01673980
travis_time:start:01673980
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00eb39d4
$ dmesg | grep -i kill
