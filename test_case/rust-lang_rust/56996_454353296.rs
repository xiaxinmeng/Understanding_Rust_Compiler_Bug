plain
travis_time:end:06c5070f:start=1547544036668993722,finish=1547544110427254251,duration=73758260529
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:14] 
[01:12:14] running 118 tests
[01:12:39] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:12:44] ......iii.i.....ii
[01:12:44] 
[01:12:44]  finished in 29.867
[01:12:44] travis_fold:end:test_debuginfo

---
[01:39:52] travis_fold:end:stage0-linkchecker

[01:39:52] travis_time:end:stage0-linkchecker:start=1547550110119868230,finish=1547550112081692327,duration=1961824097

[01:39:56] std/hint/fn.spin_loop.html:3: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/thread/fn.yield_now.html
[01:39:59] core/hint/fn.spin_loop.html:3: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/thread/fn.yield_now.html
[01:40:00] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:39:9
[01:40:00] 
[01:40:00] 
[01:40:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:40:00] expected success, got: exit code: 101
[01:40:00] expected success, got: exit code: 101
[01:40:00] 
[01:40:00] 
[01:40:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:00] Build completed unsuccessfully in 0:39:22
[01:40:00] make: *** [check] Error 1
[01:40:00] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08ffd828
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 15 11:02:00 UTC 2019
---
travis_time:end:0c504482:start=1547550121979173460,finish=1547550121984730642,duration=5557182
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02d2d84c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!chetravis_time:end:02d2d84c:start=1547550121989581918,finish=1547550122049724122,duration=60142204
travis_fold:start:after_failure.5
travis_time:start:10859e80
travis_time:start:10859e80
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13fd3388
$ dmesg | grep -i kill
