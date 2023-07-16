plain
travis_time:end:008015d0:start=1549362469777406613,finish=1549362474689306942,duration=4911900329
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:11:02] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:11:03] tidy error: /checkout/src/test/ui/resolve/token-error-correct.rs: missing trailing newline
[00:11:04] some tidy checks failed
[00:11:04] 
[00:11:04] 
[00:11:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:11:04] 
[00:11:04] 
[00:11:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:11:04] Build completed unsuccessfully in 0:00:48
[00:11:04] Build completed unsuccessfully in 0:00:48
[00:11:04] make: *** [tidy] Error 1
[00:11:04] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06c8e4d4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 10:39:09 UTC 2019
---
travis_time:end:0f627c70:start=1549363150706931031,finish=1549363150711843954,duration=4912923
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0009fbd0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:094a4276
travis_time:start:094a4276
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:006d7a53
$ dmesg | grep -i kill
