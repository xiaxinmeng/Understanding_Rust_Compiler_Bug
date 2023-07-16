plain
travis_time:end:1f4d7e9e:start=1542570720324934632,finish=1542570721501581896,duration=1176647264
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:03:46] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:47] tidy error: /checkout/src/libstd/net/ip.rs:519: line longer than 100 chars
[00:03:47] tidy error: /checkout/src/libstd/net/ip.rs:1219: line longer than 100 chars
[00:03:47] tidy error: /checkout/src/libstd/net/ip.rs:1271: line longer than 100 chars
[00:03:47] tidy error: /checkout/src/libstd/net/ip.rs:2031: line longer than 100 chars
[00:03:47] tidy error: /checkout/src/libstd/net/ip.rs:2033: line longer than 100 chars
[00:03:47] tidy error: /checkout/src/libstd/net/ip.rs:2052: line longer than 100 chars
[00:03:48] some tidy checks failed
[00:03:48] 
[00:03:48] 
[00:03:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:48] 
[00:03:48] 
[00:03:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:48] Build completed unsuccessfully in 0:00:50
[00:03:48] Build completed unsuccessfully in 0:00:50
[00:03:48] Makefile:79: recipe for target 'tidy' failed
[00:03:48] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bfbc8b8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 18 19:55:59 UTC 2018
---
travis_time:end:1a72b9f5:start=1542570959526905974,finish=1542570959534549926,duration=7643952
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00167052
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09606000
travis_time:start:09606000
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a1b9a25
$ dmesg | grep -i kill
