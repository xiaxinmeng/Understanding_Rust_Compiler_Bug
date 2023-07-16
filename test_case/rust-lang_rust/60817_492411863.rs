plain
travis_time:end:15a82df0:start=1557862127582414182,finish=1557862129782117609,duration=2199703427
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
[01:21:07] 
[01:21:07] running 143 tests
[01:21:10] i..iii.....iii..iiii.....i......................i...i................i.....i..........ii.i..i..i.ii. 100/143
[01:21:12] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:21:12] 
[01:21:12]  finished in 4.666
[01:21:12] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:14] 
[01:21:14] running 9 tests
[01:21:14] iiiiiiiii
[01:21:14] 
[01:21:14]  finished in 0.160
[01:21:14] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:30] 
[01:21:30] running 122 tests
[01:21:55] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:22:01] .i.i......iii.i.....ii
[01:22:01] 
[01:22:01]  finished in 30.903
[01:22:01] travis_fold:end:test_debuginfo

---
[01:43:41] travis_fold:end:stage0-linkchecker

[01:43:41] travis_time:end:stage0-linkchecker:start=1557868360863827443,finish=1557868362967510108,duration=2103682665

[01:43:43] std/keyword.mod.html:1: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:43] std/keyword.super.html:1: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:44] std/index.html:295: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:44] std/index.html:304: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/nightly/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[01:43:49] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:43:49] 
[01:43:49] 
[01:43:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:43:49] expected success, got: exit code: 101
[01:43:49] expected success, got: exit code: 101
[01:43:49] 
[01:43:49] 
[01:43:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:49] Build completed unsuccessfully in 0:34:42
[01:43:49] make: *** [check] Error 1
[01:43:49] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:058a62ba
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 14 21:12:51 UTC 2019
