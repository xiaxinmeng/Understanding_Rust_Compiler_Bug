plain
travis_time:end:01bcf512:start=1550142738220470889,finish=1550142739101137347,duration=880666458
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
[01:14:27] 
[01:14:27] running 119 tests
[01:14:52] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:14:56] i......iii.i.....ii
[01:14:56] 
[01:14:56]  finished in 28.721
[01:14:56] travis_fold:end:test_debuginfo

---
[01:32:26] .................................................................................................... 400/763
[01:32:27] thread '<unnamed>' panicked at 'attempted to use a condition variable with two mutexes', src/libstd/sync/condvar.rs:578:18
[01:32:27] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:32:27]   left: `16`,
[01:32:27]  right: `0`', src/libstd/sys/unix/mutex.rs:72:9
[01:32:27] thread panicked while panicking. aborting.
[01:32:27] error: process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-5a5109574d7166f0 --quiet` (signal: 4, SIGILL: illegal instruction)
[01:32:27] 
[01:32:27] 
[01:32:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:32:27] 
[01:32:27] 
[01:32:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:27] Build completed unsuccessfully in 0:29:32
[01:32:27] Build completed unsuccessfully in 0:29:32
[01:32:27] make: *** [check] Error 1
[01:32:27] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:003c3f7b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 14 12:44:55 UTC 2019
