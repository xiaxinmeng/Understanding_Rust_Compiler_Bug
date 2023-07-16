plain
Unpacking objects:  65% (29/44)   
Unpacking objects:  68% (30/44)   
Untravis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
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
[01:24:25] 
[01:24:25] running 119 tests
[01:24:52] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:24:56] i......iii.i.....ii
[01:24:56] 
[01:24:56]  finished in 30.711
[01:24:56] travis_fold:end:test_debuginfo

---
[01:35:55] 
[01:35:55]    Doc-tests core
[01:36:01] 
[01:36:01] running 2277 tests
[01:36:12] ....iiiii........................................................................................... 100/2277
[01:36:23] .....................................................................ii.....F....................... 200/2277
[01:36:53] .................................................................................................... 400/2277
[01:37:05] .....................i..i........................................................................... 500/2277
[01:37:17] .................................................................................................... 600/2277
[01:37:30] .................................................................................................... 700/2277
---
[01:40:59] 
[01:40:59] error: test failed, to rerun pass '--doc'
[01:40:59] 
[01:40:59] 
[01:40:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:40:59] 
[01:40:59] 
[01:40:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:59] Build completed unsuccessfully in 0:28:54
[01:40:59] Build completed unsuccessfully in 0:28:54
[01:40:59] Makefile:48: recipe for target 'check' failed
[01:40:59] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06d3e3f6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 27 17:56:09 UTC 2019
---
travis_time:end:0068bc35:start=1551290171507995076,finish=1551290171560713661,duration=52718585
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:29a250fd
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:003edbd6
$ dmesg | grep -i kill
