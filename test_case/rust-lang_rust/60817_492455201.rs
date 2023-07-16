plain
travis_time:end:1c24a794:start=1557872641057319656,finish=1557872644671779500,duration=3614459844
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
[01:21:30] 
[01:21:30] running 143 tests
[01:21:33] i..iii.....iii..iiii.....i......................i...i................i......i.........ii.i..i..i.ii. 100/143
[01:21:35] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:21:35] 
[01:21:35]  finished in 4.988
[01:21:35] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:38] 
[01:21:38] running 9 tests
[01:21:38] iiiiiiiii
[01:21:38] 
[01:21:38]  finished in 0.161
[01:21:38] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:53] 
[01:21:53] running 122 tests
[01:22:18] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:22:22] .i.i......iii.i.....ii
[01:22:22] 
[01:22:22]  finished in 29.344
[01:22:22] travis_fold:end:test_debuginfo

---
[01:43:44] travis_fold:end:stage0-linkchecker

[01:43:44] travis_time:end:stage0-linkchecker:start=1557878877600547153,finish=1557878879729506202,duration=2128959049

[01:43:46] std/keyword.mod.html:1: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:46] std/keyword.super.html:1: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:47] std/index.html:295: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:47] std/index.html:304: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:52] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:43:52] 
[01:43:52] 
[01:43:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:43:52] expected success, got: exit code: 101
[01:43:52] expected success, got: exit code: 101
[01:43:52] 
[01:43:52] 
[01:43:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:52] Build completed unsuccessfully in 0:34:15
[01:43:52] Makefile:48: recipe for target 'check' failed
[01:43:52] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0dbfaa88
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 15 00:08:08 UTC 2019
