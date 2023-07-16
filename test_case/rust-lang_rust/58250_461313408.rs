plain
travis_time:end:0865c396:start=1549518016251258386,finish=1549518186943784646,duration=170692526260
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
[01:11:39] 
[01:11:39] running 119 tests
[01:12:05] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:12:09] i......iii.i.....ii
[01:12:09] 
[01:12:09]  finished in 29.986
[01:12:09] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc-ui
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:40:41] 
[01:40:41] running 14 tests
[01:40:46] ..F...........
[01:40:46] failures:
[01:40:46] 
[01:40:46] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:40:46] diff of stdout:
[01:40:46] diff of stdout:
[01:40:46] 
[01:40:46] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:40:46] 23 
[01:40:46] - ', src/librustdoc/test.rs:389:17
[01:40:46] + ', src/librustdoc/test.rs:390:17
[01:40:46] 25 
[01:40:46] 25 
[01:40:46] 26 
[01:40:46] 27 failures:
[01:40:46] 
[01:40:46] 
[01:40:46] The actual stdout differed from the expected stdout.
[01:40:46] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:40:46] To update references, rerun the tests and pass the `--bless` flag
[01:40:46] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:40:46] error: 1 errors occurred comparing output.
[01:40:46] status: exit code: 101
[01:40:46] status: exit code: 101
[01:40:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:40:46] ------------------------------------------
[01:40:46] 
[01:40:46] running 2 tests
[01:40:46] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:40:46] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:40:46] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:40:46] failures:
[01:40:46] 
[01:40:46] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:40:46] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:40:46] error[E0425]: cannot find value `no` in this scope
[01:40:46]   |
[01:40:46] 3 | no
[01:40:46]   | ^^ not found in this scope
[01:40:46] 
[01:40:46] 
[01:40:46] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:355:13
[01:40:46] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:40:46] 
[01:40:46] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:40:46] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:40:46] 
[01:40:46] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:40:46] 
[01:40:46] ', src/librustdoc/test.rs:390:17
[01:40:46] 
[01:40:46] 
[01:40:46] 
[01:40:46] failures:
[01:40:46]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:40:46]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:40:46] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:40:46] 
[01:40:46] 
[01:40:46] ------------------------------------------
---
[01:40:46] test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:40:46] 
[01:40:46] 
[01:40:46] 
[01:40:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:40:46] 
[01:40:46] 
[01:40:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:46] Build completed unsuccessfully in 0:40:48
[01:40:46] Build completed unsuccessfully in 0:40:48
[01:40:46] make: *** [check] Error 1
[01:40:46] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:388487ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 07:24:02 UTC 2019
---
travis_time:end:08643a82:start=1549524244408165122,finish=1549524244413081504,duration=4916382
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2bdec07d
$ ln -s . checkout && for CORE in obj/cores/c
