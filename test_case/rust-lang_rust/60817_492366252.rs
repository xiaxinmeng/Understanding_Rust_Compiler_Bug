plain
travis_time:end:11421b7b:start=1557854123005599575,finish=1557854125028189446,duration=2022589871
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
[01:21:43] 
[01:21:43] running 143 tests
[01:21:45] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:21:47] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:21:47] 
[01:21:47]  finished in 4.469
[01:21:47] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:49] 
[01:21:49] running 9 tests
[01:21:49] iiiiiiiii
[01:21:49] 
[01:21:49]  finished in 0.145
[01:21:49] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:04] 
[01:22:04] running 122 tests
[01:22:27] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:22:32] .i.i......iii.i.....ii
[01:22:32] 
[01:22:32]  finished in 28.270
[01:22:32] travis_fold:end:test_debuginfo

---
[01:43:26] travis_fold:end:stage0-linkchecker

[01:43:26] travis_time:end:stage0-linkchecker:start=1557860339911112823,finish=1557860341993826775,duration=2082713952

[01:43:28] std/keyword.mod.html:1: absolute path - /nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:28] std/keyword.super.html:1: absolute path - /nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:28] std/index.html:295: absolute path - /nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:28] std/index.html:304: absolute path - /nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:34] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:43:34] 
[01:43:34] 
[01:43:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:43:34] expected success, got: exit code: 101
[01:43:34] expected success, got: exit code: 101
[01:43:34] 
[01:43:34] 
[01:43:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:34] Build completed unsuccessfully in 0:33:18
[01:43:34] Makefile:48: recipe for target 'check' failed
[01:43:34] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c61f24f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 14 18:59:10 UTC 2019
