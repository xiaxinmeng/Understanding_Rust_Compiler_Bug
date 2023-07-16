plain
travis_time:end:03420d2c:start=1545163607887865163,finish=1545163687189380861,duration=79301515698
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
[00:53:12] 
[00:53:12] running 119 tests
[00:53:35] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:53:39] i......iii.i.....ii
[00:53:39] 
[00:53:39]  finished in 26.713
[00:53:39] travis_fold:end:test_debuginfo

---
[01:18:10] travis_fold:end:stage0-linkchecker

[01:18:10] travis_time:end:stage0-linkchecker:start=1545168384630372367,finish=1545168386735049222,duration=2104676855

[01:18:11] std/pin/index.html:23: broken link - std/pin/trait.Unpin.html
[01:18:13] core/pin/index.html:23: broken link - core/pin/trait.Unpin.html
[01:18:15] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:49:9
[01:18:15] 
[01:18:15] 
[01:18:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:18:15] expected success, got: exit code: 101
[01:18:15] expected success, got: exit code: 101
[01:18:15] 
[01:18:15] 
[01:18:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:15] Build completed unsuccessfully in 0:35:32
[01:18:15] Makefile:58: recipe for target 'check' failed
[01:18:15] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0883cf00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec 18 21:26:32 UTC 2018
