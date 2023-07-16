plain
travis_time:end:145d5110:start=1550435248266383567,finish=1550435249044308162,duration=777924595
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
[01:11:36] 
[01:11:36] running 119 tests
[01:12:01] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:12:05] i......iii.i.....ii
[01:12:05] 
[01:12:05]  finished in 28.585
[01:12:05] travis_fold:end:test_debuginfo

---
[01:37:51] travis_fold:end:stage0-linkchecker

[01:37:51] travis_time:end:stage0-linkchecker:start=1550441129215788670,finish=1550441131230971896,duration=2015183226

[01:37:51] alloc/sync/atomic/index.html:5: broken link - alloc/std/sync/atomic/index.html
[01:37:58] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:37:58] 
[01:37:58] 
[01:37:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:37:58] expected success, got: exit code: 101
[01:37:58] expected success, got: exit code: 101
[01:37:58] 
[01:37:58] 
[01:37:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:37:58] Build completed unsuccessfully in 0:37:51
[01:37:58] Makefile:48: recipe for target 'check' failed
[01:37:58] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07a27fe2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 17 22:05:39 UTC 2019
