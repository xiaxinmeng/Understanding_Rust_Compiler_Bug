plain
travis_time:end:1e6997b4:start=1556649926925201123,finish=1556650043983212622,duration=117058011499
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:56] 
[01:20:56] running 9 tests
[01:20:56] iiiiiiiii
[01:20:56] 
[01:20:56]  finished in 0.152
[01:20:56] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:12] 
[01:21:12] running 121 tests
[01:21:38] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:21:43] i.i......iii.i.....ii
[01:21:43] 
[01:21:43]  finished in 31.227
[01:21:43] travis_fold:end:test_debuginfo

---
[01:45:45] 
[01:45:45] running 25 tests
[01:45:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:45:58] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:45:58] ............F............
[01:45:58] 
[01:45:58] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:45:58] diff of stdout:
[01:45:58] 
[01:45:58] 
[01:45:58] 15 error: aborting due to previous error
[01:45:58] 16 
[01:45:58] 17 For more information about this error, try `rustc --explain E0425`.
[01:45:58] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:45:58] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:317:13
[01:45:58] 20 
[01:45:58] 20 
[01:45:58] 21 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:45:58] 
[01:45:58] 24 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:45:58] 26 
[01:45:58] - ', src/librustdoc/test.rs:332:17
[01:45:58] + ', src/librustdoc/test.rs:346:17
[01:45:58] 28 
[01:45:58] 28 
[01:45:58] 29 
[01:45:58] 30 failures:
[01:45:58] 
[01:45:58] 
[01:45:58] The actual stdout differed from the expected stdout.
[01:45:58] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:45:58] To update references, rerun the tests and pass the `--bless` flag
[01:45:58] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:45:58] error: 1 errors occurred comparing output.
[01:45:58] status: exit code: 101
[01:45:58] status: exit code: 101
[01:45:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:45:58] ------------------------------------------
[01:45:58] 
[01:45:58] running 2 tests
[01:45:58] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:45:58] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:45:58] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:45:58] failures:
[01:45:58] 
[01:45:58] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:45:58] error[E0425]: cannot find value `no` in this scope
---
[01:45:58] For more information about this error, try `rustc --explain E0425`.
[01:45:58] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:317:13
[01:45:58] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:45:58] 
[01:45:58] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:45:58] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:45:58] 
[01:45:58] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:45:58] 
[01:45:58] ', src/librustdoc/test.rs:346:17
[01:45:58] 
[01:45:58] 
[01:45:58] 
[01:45:58] failures:
[01:45:58]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:45:58]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:45:58] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:45:58] 
[01:45:58] 
[01:45:58] ------------------------------------------
---
[01:45:58] 
[01:45:51:45:58] 
[01:45:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:45:58] Build completed unsuccessfully in 0:36:50
[01:45:58] Makefile:48: recipe for target 'check' failed
[01:45:58] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:32677e6f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 20:33:31 UTC 2019
