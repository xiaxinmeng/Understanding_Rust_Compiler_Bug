plain
travis_time:end:04ee9a5e:start=1546978089151273248,finish=1546978090321928223,duration=1170654975
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:05:08] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:08] tidy error: /checkout/src/librustc_typeck/check/demand.rs:195: line longer than 100 chars
[00:05:10] some tidy checks failed
[00:05:10] 
[00:05:10] 
[00:05:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:10] 
[00:05:10] 
[00:05:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:10] Build completed unsuccessfully in 0:00:45
[00:05:10] Build completed unsuccessfully in 0:00:45
[00:05:10] Makefile:69: recipe for target 'tidy' failed
[00:05:10] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:094e3067
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan  8 20:13:40 UTC 2019
---
travis_time:end:0148d452:start=1546978421677496844,finish=1546978421682105515,duration=4608671
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03c82578
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f41bc7b
travis_time:start:2f41bc7b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0247d91f
$ dmesg | grep -i kill
