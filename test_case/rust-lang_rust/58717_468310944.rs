plain
travis_time:end:27dbf610:start=1551361726383829793,finish=1551361884356879772,duration=157973049979
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
[01:15:00] 
[01:15:00] running 119 tests
[01:15:25] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:15:29] i......iii.i.....ii
[01:15:29] 
[01:15:29]  finished in 29.047
[01:15:29] travis_fold:end:test_debuginfo

---
[01:24:50] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:50]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:25:09] error[E0451]: field `kind` of struct `std::num::ParseIntError` is private
[01:25:09]    --> src/libcore/../libcore/tests/nonzero.rs:135:13
[01:25:09] 135 |             kind: IntErrorKind::Zero
[01:25:09]     |             ^^^^^^^^^^^^^^^^^^^^^^^^ field `kind` is private
[01:25:09] 
[01:25:09] 
[01:25:09] error[E0451]: field `kind` of struct `std::num::ParseIntError` is private
[01:25:09]    --> src/libcore/../libcore/tests/nonzero.rs:141:13
[01:25:09] 141 |             kind: IntErrorKind::Underflow
[01:25:09]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `kind` is private
[01:25:09] 
[01:25:09] 
[01:25:09] error[E0451]: field `kind` of struct `std::num::ParseIntError` is private
[01:25:09]    --> src/libcore/../libcore/tests/nonzero.rs:147:13
[01:25:09] 147 |             kind: IntErrorKind::Overflow
[01:25:09]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `kind` is private
[01:25:09] 
[01:25:10] error: aborting due to 3 previous errors
[01:25:10] error: aborting due to 3 previous errors
[01:25:10] 
[01:25:10] For more information about this error, try `rustc --explain E0451`.
[01:25:10] error: Could not compile `core`.
[01:25:10] 
[01:25:10] To learn more, run the command again with --verbose.
[01:25:10] 
[01:25:10] 
[01:25:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:25:10] 
[01:25:10] 
[01:25:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:10] Build completed unsuccessfully in 0:21:38
[01:25:10] Build completed unsuccessfully in 0:21:38
[01:25:10] make: *** [check] Error 1
[01:25:10] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0017aaca
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 28 15:16:43 UTC 2019
---
travis_time:end:000834e0:start=1551367005579610717,finish=1551367005585463281,duration=5852564
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:203d394e
$ ln -s . checkout && for CORE 
