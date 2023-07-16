plain
travis_time:end:05783928:start=1554326837989462115,finish=1554326838988987464,duration=999525349
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
[01:23:56] 
[01:23:56] running 9 tests
[01:23:56] iiiiiiiii
[01:23:56] 
[01:23:56]  finished in 0.157
[01:23:56] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:12] 
[01:24:12] running 121 tests
[01:24:39] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:24:44] i.i......iii.i.....ii
[01:24:44] 
[01:24:44]  finished in 31.632
[01:24:44] travis_fold:end:test_debuginfo

---
[01:53:16] travis_fold:end:stage0-linkchecker

[01:53:16] travis_time:end:stage0-linkchecker:start=1554333643750532476,finish=1554333646023327726,duration=2272795250

[01:53:16] cargo/print.html:13033: directory link - edition-guide
[01:53:16] cargo/appendix/glossary.html:149: directory link - edition-guide
[01:53:23] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:53:23] 
[01:53:23] 
[01:53:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:53:23] expected success, got: exit code: 101
[01:53:23] expected success, got: exit code: 101
[01:53:23] 
[01:53:23] 
[01:53:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:53:23] Build completed unsuccessfully in 0:41:40
[01:53:23] Makefile:48: recipe for target 'check' failed
[01:53:23] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07e247c5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr  3 23:20:54 UTC 2019
