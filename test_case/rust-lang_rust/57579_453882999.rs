plain
travis_time:end:37219e82:start=1547423227885386447,finish=1547423298566798116,duration=70681411669
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
[01:12:44] 
[01:12:44] running 118 tests
[01:13:09] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:13:14] ......iii.i.....ii
[01:13:14] 
[01:13:14]  finished in 29.595
[01:13:14] travis_fold:end:test_debuginfo

---
[01:24:33] 
[01:24:33] To learn more, run the command again with --verbose.
[01:24:33] 
[01:24:33] 
[01:24:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:24:33] 
[01:24:33] 
[01:24:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:33] Build completed unsuccessfully in 0:23:39
[01:24:33] Build completed unsuccessfully in 0:23:39
[01:24:33] Makefile:48: recipe for target 'check' failed
[01:24:33] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02bdac46
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 14 01:13:01 UTC 2019
---
travis_time:end:011801a0:start=1547428383417756850,finish=1547428383487746872,duration=69990022
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:26d1a2be
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12b19d44
$ dmesg | grep -i kill
