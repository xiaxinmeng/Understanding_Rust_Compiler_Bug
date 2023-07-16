plain
travis_time:end:14d955fe:start=1549125930567548702,finish=1549125932935744937,duration=2368196235
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
[01:12:19] 
[01:12:19] running 119 tests
[01:12:47] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:12:52] i......iii.i.....ii
[01:12:52] 
[01:12:52]  finished in 32.991
[01:12:52] travis_fold:end:test_debuginfo

---
[01:35:09]     Finished release [optimized] target(s) in 15.59s
[01:35:09]      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_driver-8025f6f53380c851
[01:35:09] 
[01:35:09] running 9 tests
[01:35:09] error: failed to find a `codegen-backends` folder in the sysroot candidates:
[01:35:09] * /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
[01:35:09] 
[01:35:09] thread 'rustc' panicked at 'Once instance has previously been poisoned', src/libstd/sync/once.rs:362:21
[01:35:09] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:35:09] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:35:09] thread 'rustc' panicked at 'Once instance has previously been poisoned', src/libstd/sync/once.rs:362:21
[01:35:09] thread 'rustc' panicked at 'Once instance has previously been poisoned', src/libstd/sync/once.rs:362:21
[01:35:09] thread 'rustc' panicked at 'Once instance has previously been poisoned', src/libstd/sync/once.rs:362:21
[01:35:09] thread 'thread 'rustcrustc' panicked at '' panicked at 'Once instance has previously been poisonedOnce instance has previously been poisoned', ', src/libstd/sync/once.rssrc/libstd/sync/once.rs::362362::2121
[01:35:09] 
[01:35:09] thread 'rustc' panicked at 'thread 'Once instance has previously been poisoned', rustcsrc/libstd/sync/once.rs' panicked at ':Once instance has previously been poisoned', 362src/libstd/sync/once.rs::21362
[01:35:09] FFFFFFFFF
[01:35:09] failures:
[01:35:09] 
[01:35:09] failures:
---
[01:35:09] 
[01:35:09] error: test failed, to rerun pass '--lib'
[01:35:09] 
[01:35:09] 
[01:35:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:35:09] 
[01:35:09] 
[01:35:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:09] Build completed unsuccessfully in 0:35:18
[01:35:09] Build completed unsuccessfully in 0:35:18
[01:35:09] make: *** [check] Error 1
[01:35:09] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1940a1c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  2 18:20:53 UTC 2019
---
travis_time:end:298ab368:start=1549131655389680549,finish=1549131655394830394,duration=5149845
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0db4f6fe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo
