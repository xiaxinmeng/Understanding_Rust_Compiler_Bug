plain
travis_time:end:1bf50d9f:start=1554229678433293926,finish=1554229680936511331,duration=2503217405
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:49] 
[01:19:49] running 9 tests
[01:19:49] iiiiiiiii
[01:19:49] 
[01:19:49]  finished in 0.149
[01:19:49] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:05] 
[01:20:05] running 121 tests
[01:20:29] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:20:33] i.i......iii.i.....ii
[01:20:33] 
[01:20:33]  finished in 28.693
[01:20:33] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc-ui
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:47:56] 
[01:47:56] running 24 tests
[01:48:08] ...........F............
[01:48:08] failures:
[01:48:08] 
[01:48:08] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:48:08] diff of stdout:
[01:48:08] diff of stdout:
[01:48:08] 
[01:48:08] 6 failures:
[01:48:08] 7 
[01:48:08] 8 ---- $DIR/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:48:08] - error[E0425]: cannot find value `no` in this scope
[01:48:08] -   |
[01:48:08] - 3 | no
[01:48:08] -   | ^^ not found in this scope
[01:48:08] -   | ^^ not found in this scope
[01:48:08] + error[E0425]: cannot find value `no` in this scope
[01:48:08] +   |
[01:48:08] + 3 | no
[01:48:08] +   | ^^ not found in this scope
[01:48:08] 14 
[01:48:08] 14 
[01:48:08] - error: aborting due to previous error
[01:48:08] + error: aborting due to previous error
[01:48:08] 16 
[01:48:08] - For more information about this error, try `rustc --explain E0425`.
[01:48:08] + For more information about this error, try `rustc --explain E0425`.
[01:48:08] 18 thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:48:08] 20 
[01:48:08] 
[01:48:08] 
[01:48:08] The actual stdout differed from the expected stdout.
[01:48:08] The actual stdout differed from the expected stdout.
[01:48:08] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:48:08] To update references, rerun the tests and pass the `--bless` flag
[01:48:08] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:48:08] error: 1 errors occurred comparing output.
[01:48:08] status: exit code: 101
[01:48:08] status: exit code: 101
[01:48:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:48:08] ------------------------------------------
[01:48:08] 
[01:48:08] ------------------------------------------
[01:48:08] stderr:
---
[01:48:08] test result: FAILED. 23 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:48:08] 
[01:48:08] 
[01:48:08] 
[01:48:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:48:08] 
[01:48:08] 
[01:48:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:48:08] Build completed unsuccessfully in 0:39:42
[01:48:08] Build completed unsuccessfully in 0:39:42
[01:48:08] make: *** [check] Error 1
[01:48:08] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1eb7134a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr  2 20:16:19 UTC 2019
