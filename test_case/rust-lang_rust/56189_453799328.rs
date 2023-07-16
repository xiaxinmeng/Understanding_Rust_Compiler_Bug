plain
travis_time:end:24a6e5e8:start=1547344477534995725,finish=1547344479663251464,duration=2128255739
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
[01:13:30] 
[01:13:30] running 118 tests
[01:13:55] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:13:59] ......iii.i.....ii
[01:13:59] 
[01:13:59]  finished in 29.445
[01:13:59] travis_fold:end:test_debuginfo

---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:45:00] 
[01:45:00] running 14 tests
[01:45:06] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:45:06] ...F..........
[01:45:06] 
[01:45:06] ---- [ui] rustdoc-ui/failed-doctest-output.rs stdout ----
[01:45:06] diff of stdout:
[01:45:06] 
[01:45:06] 
[01:45:06] 12 3 | no
[01:45:06] 13   | ^^ not found in this scope
[01:45:06] 14 
[01:45:06] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:45:06] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:356:13
[01:45:06] 17 
[01:45:06] 17 
[01:45:06] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:45:06] 
[01:45:06] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:45:06] 23 
[01:45:06] - ', src/librustdoc/test.rs:386:17
[01:45:06] + ', src/librustdoc/test.rs:391:17
[01:45:06] 25 
[01:45:06] 25 
[01:45:06] 26 
[01:45:06] 27 failures:
[01:45:06] 
[01:45:06] 
[01:45:06] The actual stdout differed from the expected stdout.
[01:45:06] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:45:06] To update references, rerun the tests and pass the `--bless` flag
[01:45:06] To only update this specific test, also pass `--test-args failed-doctest-output.rs`
[01:45:06] error: 1 errors occurred comparing output.
[01:45:06] status: exit code: 101
[01:45:06] status: exit code: 101
[01:45:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
[01:45:06] ------------------------------------------
[01:45:06] 
[01:45:06] running 2 tests
[01:45:06] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:45:06] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) ... FAILED
[01:45:06] test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
[01:45:06] failures:
[01:45:06] 
[01:45:06] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:45:06] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17) stdout ----
[01:45:06] error[E0425]: cannot find value `no` in this scope
[01:45:06]   |
[01:45:06] 3 | no
[01:45:06]   | ^^ not found in this scope
[01:45:06] 
[01:45:06] 
[01:45:06] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:356:13
[01:45:06] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:45:06] 
[01:45:06] ---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:45:06] thread '/checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)' panicked at 'test executable failed:
[01:45:06] 
[01:45:06] thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:3:1
[01:45:06] 
[01:45:06] ', src/librustdoc/test.rs:391:17
[01:45:06] 
[01:45:06] 
[01:45:06] 
[01:45:06] failures:
[01:45:06]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 17)
[01:45:06]     /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11)
[01:45:06] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:45:06] 
[01:45:06] 
[01:45:06] ------------------------------------------
---
[01:45:06] test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:45:06] 
[01:45:06] 
[01:45:06] 
[01:45:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:45:06] 
[01:45:06] 
[01:45:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:45:06] Build completed unsuccessfully in 0:43:29
[01:45:06] Build completed unsuccessfully in 0:43:29
[01:45:06] Makefile:48: recipe for target 'check' failed
[01:45:06] make: *** [check] Error 1
30696 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release/build
28344 ./obj/build/x86_64-unknown-linux-gnu/doc/book
27044 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
27040 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
