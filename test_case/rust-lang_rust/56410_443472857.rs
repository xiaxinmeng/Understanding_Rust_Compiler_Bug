plain
travis_time:end:13ee08a4:start=1543709531669510421,finish=1543709533977542973,duration=2308032552
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
[00:59:22] 
[00:59:22] running 119 tests
[00:59:25] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[00:59:25] .ii.i.....iiii.....
[00:59:25] 
[00:59:25]  finished in 3.668
[00:59:25] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:41] 
[00:59:41] running 118 tests
[01:00:07] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:00:11] ......iii.i.....ii
[01:00:11] 
[01:00:11]  finished in 30.123
[01:00:11] travis_fold:end:test_debuginfo

---
[01:17:03] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:03]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:17:06] error[E0432]: unresolved imports `Condvar`, `Mutex`
[01:17:06]    --> src/libstd/sys_common/parking_lot/condvar.rs:399:10
[01:17:06]     |
[01:17:06] 399 |     use {Condvar, Mutex};
[01:17:06]     |          ^^^^^^^  ^^^^^ no `Mutex` in the root
[01:17:06]     |          no `Condvar` in the root
[01:17:06] 
[01:17:19] error: aborting due to previous error
[01:17:19] 
[01:17:19] 
[01:17:19] For more information about this error, try `rustc --explain E0432`.
[01:17:19] error: Could not compile `std`.
[01:17:19] 
[01:17:19] To learn more, run the command again with --verbose.
[01:17:19] 
[01:17:19] 
[01:17:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:17:19] 
[01:17:19] 
[01:17:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:19] Build completed unsuccessfully in 0:29:07
[01:17:19] Build completed unsuccessfully in 0:29:07
[01:17:19] make: *** [check] Error 1
[01:17:19] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00ae4529
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec  2 01:29:43 UTC 2018
