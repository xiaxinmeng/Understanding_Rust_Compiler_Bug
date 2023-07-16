plain
travis_time:end:116f4e4f:start=1554282517681176085,finish=1554282628385919550,duration=110704743465
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:58] 
[01:22:58] running 9 tests
[01:22:58] iiiiiiiii
[01:22:58] 
[01:22:58]  finished in 0.156
[01:22:58] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:14] 
[01:23:14] running 121 tests
[01:23:40] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:23:45] i.i......iii.i.....ii
[01:23:45] 
[01:23:45]  finished in 30.807
[01:23:45] travis_fold:end:test_debuginfo

---
[01:51:26] travis_fold:end:stage0-linkchecker

[01:51:26] travis_time:end:stage0-linkchecker:start=1554289321744213817,finish=1554289323800123346,duration=2055909529

[01:51:29] std/hint/fn.spin_loop.html:4: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/thread/fn.yield_now.html
[01:51:29] std/hint/fn.spin_loop.html:14: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/thread/fn.yield_now.html
[01:51:33] core/hint/fn.spin_loop.html:4: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/thread/fn.yield_now.html
[01:51:33] core/hint/fn.spin_loop.html:14: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/thread/fn.yield_now.html
[01:51:33] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:51:33] 
[01:51:33] 
[01:51:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:51:33] expected success, got: exit code: 101
[01:51:33] expected success, got: exit code: 101
[01:51:33] 
[01:51:33] 
[01:51:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:51:33] Build completed unsuccessfully in 0:40:41
[01:51:33] Makefile:48: recipe for target 'check' failed
[01:51:33] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:299bb582
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr  3 11:02:11 UTC 2019
