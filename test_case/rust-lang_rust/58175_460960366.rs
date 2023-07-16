plain
travis_time:end:03626e7a:start=1549440373640777592,finish=1549440448082992966,duration=74442215374
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
[01:10:55] 
[01:10:55] running 119 tests
[01:11:21] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:11:26] i......iii.i.....ii
[01:11:26] 
[01:11:26]  finished in 30.393
[01:11:26] travis_fold:end:test_debuginfo

---
[01:36:42] travis_fold:end:stage0-linkchecker

[01:36:42] travis_time:end:stage0-linkchecker:start=1549446257710709606,finish=1549446259623088878,duration=1912379272

[01:36:43] alloc/sync/atomic/index.html:13: broken link - alloc/marker/trait.Sync.html
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:098bced4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 09:44:27 UTC 2019
---
travis_time:end:0ff3da40:start=1549446269199554678,finish=1549446269256814024,duration=57259346
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03cc1174
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04b140d7
$ dmesg | grep -i kill
