plain
travis_time:end:026e003a:start=1549669934240811867,finish=1549670008746836906,duration=74506025039
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
[01:10:27] 
[01:10:27] running 119 tests
[01:10:52] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:10:56] i......iii.i.....ii
[01:10:56] 
[01:10:56]  finished in 29.312
[01:10:56] travis_fold:end:test_debuginfo

---
[01:35:57] travis_fold:end:stage0-linkchecker

[01:35:57] travis_time:end:stage0-linkchecker:start=1549675772549704369,finish=1549675774626307748,duration=2076603379

[01:35:58] alloc/sync/atomic/index.html:13: broken link - alloc/marker/trait.Sync.html
[01:36:05] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:36:05] 
[01:36:05] 
[01:36:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:36:05] expected success, got: exit code: 101
[01:36:05] expected success, got: exit code: 101
[01:36:05] 
[01:36:05] 
[01:36:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:05] Build completed unsuccessfully in 0:37:12
[01:36:05] make: *** [check] Error 1
[01:36:05] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:060fb820
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  9 01:29:43 UTC 2019
