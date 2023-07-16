plain
travis_time:end:087ab5dc:start=1557885636902207577,finish=1557885725018605259,duration=88116397682
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
[01:18:49] 
[01:18:49] running 143 tests
[01:18:52] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:18:54] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:18:54] 
[01:18:54]  finished in 4.594
[01:18:54] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:56] 
[01:18:56] running 9 tests
[01:18:56] iiiiiiiii
[01:18:56] 
[01:18:56]  finished in 0.154
[01:18:56] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:11] 
[01:19:11] running 122 tests
[01:19:36] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:19:41] .i.i......iii.i.....ii
[01:19:41] 
[01:19:41]  finished in 29.659
[01:19:41] travis_fold:end:test_debuginfo

---
[01:41:04] travis_fold:end:stage0-linkchecker

[01:41:04] travis_time:end:stage0-linkchecker:start=1557891796973844216,finish=1557891798993551479,duration=2019707263

[01:41:07] std/keyword.union.html:1: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/reference/items/unions.html
[01:41:07] std/index.html:309: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/reference/items/unions.html
[01:41:12] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:41:12] 
[01:41:12] 
[01:41:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:41:12] expected success, got: exit code: 101
[01:41:12] expected success, got: exit code: 101
[01:41:12] 
[01:41:12] 
[01:41:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:41:12] Build completed unsuccessfully in 0:34:01
[01:41:12] Makefile:48: recipe for target 'check' failed
[01:41:12] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00078169
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 15 03:43:27 UTC 2019
