plain
travis_time:end:2186c604:start=1543833366901124773,finish=1543833368046465512,duration=1145340739
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:25] 
[00:54:25] running 119 tests
[00:54:28] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[00:54:29] .ii.i.....iiii.....
[00:54:29] 
[00:54:29]  finished in 3.367
[00:54:29] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:43] 
[00:54:43] running 118 tests
[00:55:06] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:55:10] ......iii.i.....ii
[00:55:10] 
[00:55:10]  finished in 27.202
[00:55:10] travis_fold:end:test_debuginfo

---
[01:20:11] travis_fold:end:stage0-linkchecker

[01:20:11] travis_time:end:stage0-linkchecker:start=1543838186664567473,finish=1543838189096460201,duration=2431892728

[01:20:11] std/primitive.str.html:585: broken link - iter/trait.DoubleEndedIterator.html
[01:20:11] std/primitive.str.html:663: broken link - iter/trait.DoubleEndedIterator.html
[01:20:11] std/primitive.str.html:695: broken link - iter/trait.DoubleEndedIterator.html
[01:20:11] std/primitive.str.html:796: broken link - iter/trait.DoubleEndedIterator.html
[01:20:11] std/primitive.str.html:816: broken link - iter/trait.DoubleEndedIterator.html
[01:20:11] std/primitive.str.html:835: broken link - iter/trait.DoubleEndedIterator.html
[01:20:11] std/primitive.str.html:860: broken link - iter/trait.DoubleEndedIterator.html
[01:20:16] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:49:9
[01:20:16] 
[01:20:16] 
[01:20:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:20:16] expected success, got: exit code: 101
[01:20:16] expected success, got: exit code: 101
[01:20:16] 
[01:20:16] 
[01:20:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:16] Build completed unsuccessfully in 0:36:15
[01:20:16] make: *** [check] Error 1
[01:20:16] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0005d1cb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec  3 11:56:35 UTC 2018
---
travis_time:end:26c68928:start=1543838197544871556,finish=1543838197643107783,duration=98236227
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05ae65b0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:053a44b8
$ dmesg | grep -i kill
