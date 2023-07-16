plain
travis_time:end:0fe6e128:start=1549942434637732500,finish=1549942508737713414,duration=74099980914
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
[01:07:12] 
[01:07:12] running 119 tests
[01:07:36] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:07:40] i......iii.i.....ii
[01:07:40] 
[01:07:40]  finished in 28.435
[01:07:40] travis_fold:end:test_debuginfo

---
[01:32:55] travis_fold:end:stage0-linkchecker

[01:32:55] travis_time:end:stage0-linkchecker:start=1549948090938071448,finish=1549948093129082597,duration=2191011149

[01:32:57] std/io/struct.IoVec.html:393: broken link fragment `#method.sort_by_key` pointing to `std/io/struct.IoVec.html`
[01:32:57] std/io/struct.IoVec.html:505: broken link fragment `#method.make_ascii_uppercase` pointing to `std/io/struct.IoVec.html`
[01:32:57] std/io/struct.IoVec.html:510: broken link fragment `#method.make_ascii_lowercase` pointing to `std/io/struct.IoVec.html`
[01:33:03] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:33:03] 
[01:33:03] 
[01:33:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:33:03] expected success, got: exit code: 101
[01:33:03] expected success, got: exit code: 101
[01:33:03] 
[01:33:03] 
[01:33:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:03] Build completed unsuccessfully in 0:37:12
[01:33:03] Makefile:48: recipe for target 'check' failed
[01:33:03] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0852ae14
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 05:08:21 UTC 2019
---
travis_time:end:1303481a:start=1549948103091812517,finish=1549948103146636412,duration=54823895
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0074ab6a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:121e15f4
$ dmesg | grep -i kill
