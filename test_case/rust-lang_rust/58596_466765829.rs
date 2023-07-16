plain
travis_time:end:26258363:start=1551002030495889378,finish=1551002032872750424,duration=2376861046
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
[01:13:41] 
[01:13:41] running 119 tests
[01:14:07] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:14:12] i......iii.i.....ii
[01:14:12] 
[01:14:12]  finished in 30.502
[01:14:12] travis_fold:end:test_debuginfo

---
[01:40:45] travis_fold:end:stage0-linkchecker

[01:40:45] travis_time:end:stage0-linkchecker:start=1551008086699973458,finish=1551008088722490149,duration=2022516691

[01:40:47] std/primitive.str.html:1142: absolute path - /checkout/src/libcore/std/primitive.char.html
[01:40:47] std/primitive.str.html:1165: absolute path - /checkout/src/libcore/std/primitive.char.html
[01:40:47] std/primitive.str.html:1186: absolute path - /checkout/src/libcore/std/primitive.char.html
[01:40:53] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:40:53] 
[01:40:53] 
[01:40:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:40:53] expected success, got: exit code: 101
[01:40:53] expected success, got: exit code: 101
[01:40:53] 
[01:40:53] 
[01:40:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:53] Build completed unsuccessfully in 0:39:04
[01:40:53] make: *** [check] Error 1
[01:40:53] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21a6f60a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 24 11:34:56 UTC 2019
---
travis_time:end:0db00ba5:start=1551008098748373489,finish=1551008098803739531,duration=55366042
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f0f5137
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:068cd678
$ dmesg | grep -i kill
