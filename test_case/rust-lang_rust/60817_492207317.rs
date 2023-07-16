plain
travis_time:end:179a9e56:start=1557829011799816242,finish=1557829012537681079,duration=737864837
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:33] 
[01:19:33] running 143 tests
[01:19:36] i..iii.....iii..iiii.....i......................i...i................i.....i..........ii.i..i..i.ii. 100/143
[01:19:37] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:19:37] 
[01:19:37]  finished in 4.608
[01:19:37] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:39] 
[01:19:39] running 9 tests
[01:19:39] iiiiiiiii
[01:19:39] 
[01:19:39]  finished in 0.148
[01:19:39] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:55] 
[01:19:55] running 122 tests
[01:20:20] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:20:25] .i.i......iii.i.....ii
[01:20:25] 
[01:20:25]  finished in 30.530
[01:20:25] travis_fold:end:test_debuginfo

---
[01:41:43] travis_fold:end:stage0-linkchecker

[01:41:43] travis_time:end:stage0-linkchecker:start=1557835124511247893,finish=1557835126474664071,duration=1963416178

[01:41:44] std/keyword.mod.html:1: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:41:45] std/keyword.super.html:1: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:41:45] std/index.html:295: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:41:45] std/index.html:304: broken link - book/ch07-02-modules-and-use-to-control-scope-and-privacy.html
[01:41:51] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:41:51] 
[01:41:51] 
[01:41:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:41:51] expected success, got: exit code: 101
[01:41:51] expected success, got: exit code: 101
[01:41:51] 
[01:41:51] 
[01:41:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:41:51] Build completed unsuccessfully in 0:34:00
[01:41:51] Makefile:48: recipe for target 'check' failed
[01:41:51] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1775d9fe
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 14 11:58:55 UTC 2019
