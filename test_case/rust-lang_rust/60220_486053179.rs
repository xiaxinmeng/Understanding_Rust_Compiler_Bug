plain
travis_time:end:0d869606:start=1556070157981840191,finish=1556070243991411815,duration=86009571624
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
[01:13:28] 
[01:13:28] running 9 tests
[01:13:28] iiiiiiiii
[01:13:28] 
[01:13:28]  finished in 0.149
[01:13:28] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:44] 
[01:13:44] running 121 tests
[01:14:09] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:14:13] i.i......iii.i.....ii
[01:14:13] 
[01:14:13]  finished in 29.605
[01:14:13] travis_fold:end:test_debuginfo

---
[01:43:46] 
[01:43:46] running 25 tests
[01:43:58] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:43:58] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:43:58] ....................F....
[01:43:58] 
[01:43:58] ---- [ui] rustdoc-ui/unparseable-doc-test.rs stdout ----
[01:43:58] diff of stdout:
[01:43:58] 
[01:43:58] 
[01:43:58] 1 
[01:43:58] 2 running 2 tests
[01:43:58] - test $DIR/unparseable-doc-test.rs - a (line 6) ... FAILED
[01:43:58] 4 test $DIR/unparseable-doc-test.rs - b (line 12) ... FAILED
[01:43:58] + test $DIR/unparseable-doc-test.rs - a (line 6) ... FAILED
[01:43:58] 6 failures:
[01:43:58] 7 
[01:43:58] 
[01:43:58] 
[01:43:58] + ---- $DIR/unparseable-doc-test.rs - b (line 12) stdout ----
[01:43:58] + error: unterminated double quote string
[01:43:58] +  --> $DIR/unparseable-doc-test.rs:14:1
[01:43:58] + 2 | "unterminated
[01:43:58] +   | ^^^^^^^^^^^^^
[01:43:58] + 
[01:43:58] + error: aborting due to previous error
[01:43:58] + error: aborting due to previous error
[01:43:58] + 
[01:43:58] + thread '$DIR/unparseable-doc-test.rs - b (line 12)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
[01:43:58] + note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:43:58] + 
[01:43:58] 8 ---- $DIR/unparseable-doc-test.rs - a (line 6) stdout ----
[01:43:58] 10  --> $DIR/unparseable-doc-test.rs:8:1
[01:43:58] 
[01:43:58] 19 error: aborting due to previous error
[01:43:58] 20 
[01:43:58] 20 
[01:43:58] 21 thread '$DIR/unparseable-doc-test.rs - a (line 6)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
[01:43:58] - note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:43:58] - 
[01:43:58] - ---- $DIR/unparseable-doc-test.rs - b (line 12) stdout ----
[01:43:58] - error: unterminated double quote string
[01:43:58] -  --> $DIR/unparseable-doc-test.rs:14:1
[01:43:58] - 2 | "unterminated
[01:43:58] -   | ^^^^^^^^^^^^^
[01:43:58] - 
[01:43:58] - error: aborting due to previous error
[01:43:58] - error: aborting due to previous error
[01:43:58] - 
[01:43:58] - thread '$DIR/unparseable-doc-test.rs - b (line 12)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
[01:43:58] 35 
[01:43:58] 36 failures:
[01:43:58] 
[01:43:58] 
[01:43:58] 
[01:43:58] The actual stdout differed from the expected stdout.
[01:43:58] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unparseable-doc-test/unparseable-doc-test.stdout
[01:43:58] To update references, rerun the tests and pass the `--bless` flag
[01:43:58] To only update this specific test, also pass `--test-args unparseable-doc-test.rs`
[01:43:58] error: 1 errors occurred comparing output.
[01:43:58] status: exit code: 101
[01:43:58] status: exit code: 101
[01:43:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/unparseable-doc-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unparseable-doc-test/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unparseable-doc-test/auxiliary"
[01:43:58] ------------------------------------------
[01:43:58] 
[01:43:58] running 2 tests
[01:43:58] running 2 tests
[01:43:58] test /checkout/src/test/rustdoc-ui/unparseable-doc-test.rs - b (line 12) ... FAILED
[01:43:58] test /checkout/src/test/rustdoc-ui/unparseable-doc-test.rs - a (line 6) ... FAILED
[01:43:58] failures:
[01:43:58] 
[01:43:58] 
[01:43:58] ---- /checkout/src/test/rustdoc-ui/unparseable-doc-test.rs - b (line 12) stdout ----
[01:43:58] error: unterminated double quote string
[01:43:58]  --> /checkout/src/test/rustdoc-ui/unparseable-doc-test.rs:14:1
[01:43:58] 2 | "unterminated
[01:43:58]   | ^^^^^^^^^^^^^
[01:43:58] 
[01:43:58] error: aborting due to previous error
[01:43:58] error: aborting due to previous error
[01:43:58] 
[01:43:58] thread '/checkout/src/test/rustdoc-ui/unparseable-doc-test.rs - b (line 12)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
[01:43:58] 
[01:43:58] 
[01:43:58] ---- /checkout/src/test/rustdoc-ui/unparseable-doc-test.rs - a (line 6) stdout ----
[01:43:58]  --> /checkout/src/test/rustdoc-ui/unparseable-doc-test.rs:8:1
[01:43:58]   |
[01:43:58] 2 | `
[01:43:58]   | ^
[01:43:58]   | ^
[01:43:58] help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
[01:43:58]   |
[01:43:58] 2 | '
[01:43:58]   |
[01:43:58] 
[01:43:58] error: aborting due to previous error
[01:43:58] 
[01:43:58] thread '/checkout/src/test/rustdoc-ui/unparseable-doc-test.rs - a (line 6)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
[01:43:58] 
[01:43:58] failures:
[01:43:58] failures:
[01:43:58]     /checkout/src/test/rustdoc-ui/unparseable-doc-test.rs - a (line 6)
[01:43:58]     /checkout/src/test/rustdoc-ui/unparseable-doc-test.rs - b (line 12)
[01:43:58] test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:43:58] 
[01:43:58] 
[01:43:58] ------------------------------------------
---
[01:43:58] test result: FAILED. 24 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:43:58] 
[01:43:58] 
[01:43:58] 
[01:43:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:43:58] 
[01:43:58] 
[01:43:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:58] Build completed unsuccessfully in 0:41:56
[01:43:58] Build completed unsuccessfully in 0:41:56
[01:43:58] Makefile:48: recipe for target 'check' failed
[01:43:58] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01fa3a71
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 24 03:28:12 UTC 2019
---
travis_time:end:0db0924c:start=1556076493883255987,finish=1556076493940466172,duration=57210185
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:26722600
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07b0e198
$ dmesg | grep -i kill
