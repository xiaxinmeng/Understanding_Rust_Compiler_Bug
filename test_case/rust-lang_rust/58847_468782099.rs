plain
travis_time:end:147d2dd3:start=1551462432501123491,finish=1551462433402242789,duration=901119298
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
[01:15:32] 
[01:15:32] running 119 tests
[01:15:57] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:16:01] i......iii.i.....ii
[01:16:01] 
[01:16:01]  finished in 29.013
[01:16:01] travis_fold:end:test_debuginfo

---
[01:38:05] 
[01:38:05] error: test failed, to rerun pass '--lib'
[01:38:05] 
[01:38:05] 
[01:38:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:38:05] 
[01:38:05] 
[01:38:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:05] Build completed unsuccessfully in 0:34:33
[01:38:05] Build completed unsuccessfully in 0:34:33
[01:38:05] make: *** [check] Error 1
[01:38:05] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0255482c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar  1 19:25:29 UTC 2019
---
travis_time:end:1598e4db:start=1551468331035927668,finish=1551468331095536332,duration=59608664
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dc6756e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04871f00
$ dmesg | grep -i kill
