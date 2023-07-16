plain
travis_time:end:151be792:start=1550339882495244432,finish=1550339958707367624,duration=76212123192
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
[01:10:51] 
[01:10:51] running 119 tests
[01:11:15] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:11:19] i......iii.i.....ii
[01:11:19] 
[01:11:19]  finished in 28.336
[01:11:19] travis_fold:end:test_debuginfo

---
[01:18:11]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[01:18:12] error[E0428]: the name `test_try_fold_unit` is defined multiple times
[01:18:12]     --> src/liballoc/../liballoc/tests/vec_deque.rs:1504:1
[01:18:12]      |
[01:18:12] 1464 | fn test_try_fold_unit() {
[01:18:12]      | ----------------------- previous definition of the value `test_try_fold_unit` here
[01:18:12] 1504 | fn test_try_fold_unit() {
[01:18:12] 1504 | fn test_try_fold_unit() {
[01:18:12]      | ^^^^^^^^^^^^^^^^^^^^^^^ `test_try_fold_unit` redefined here
[01:18:12]      |
[01:18:12]      = note: `test_try_fold_unit` must be defined only once in the value namespace of this module
[01:18:17] error: aborting due to previous error
[01:18:17] 
[01:18:17] For more information about this error, try `rustc --explain E0428`.
[01:18:17] error: Could not compile `alloc`.
[01:18:17] error: Could not compile `alloc`.
[01:18:17] warning: build failed, waiting for other jobs to finish...
[01:18:25] error: build failed
[01:18:25] 
[01:18:25] 
[01:18:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:18:25] 
[01:18:25] 
[01:18:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:18:25] Build completed unsuccessfully in 0:18:57
[01:18:25] Build completed unsuccessfully in 0:18:57
[01:18:25] make: *** [check] Error 1
[01:18:25] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05de4540
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb 16 19:17:53 UTC 2019
