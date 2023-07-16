plain
2019-12-08T10:50:29.9479910Z diff of stdout:
2019-12-08T10:50:29.9480540Z 
2019-12-08T10:50:29.9481190Z 1 
2019-12-08T10:50:29.9491140Z 2 running 2 tests
2019-12-08T10:50:29.9492930Z - test $DIR/failed-doctest-output.rs - OtherStruct (line 21) ... FAILED
2019-12-08T10:50:29.9494260Z 4 test $DIR/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
2019-12-08T10:50:29.9495960Z + test $DIR/failed-doctest-output.rs - OtherStruct (line 21) ... FAILED
2019-12-08T10:50:29.9533040Z 6 failures:
2019-12-08T10:50:29.9533720Z 7 
2019-12-08T10:50:29.9534200Z 
2019-12-08T10:50:29.9534200Z 
2019-12-08T10:50:29.9535560Z - ---- $DIR/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
2019-12-08T10:50:29.9536840Z - error[E0425]: cannot find value `no` in this scope
2019-12-08T10:50:29.9549350Z -    |
2019-12-08T10:50:29.9551350Z - LL | no
2019-12-08T10:50:29.9552680Z -    | ^^ not found in this scope
2019-12-08T10:50:29.9553880Z - 
2019-12-08T10:50:29.9553880Z - 
2019-12-08T10:50:29.9556150Z - error: aborting due to previous error
2019-12-08T10:50:29.9557440Z - 
2019-12-08T10:50:29.9558720Z - For more information about this error, try `rustc --explain E0425`.
2019-12-08T10:50:29.9559500Z - Couldn't compile the test.
2019-12-08T10:50:29.9560330Z 19 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
2019-12-08T10:50:29.9560510Z 20 Test executable failed (exit code 101).
2019-12-08T10:50:29.9561610Z 
2019-12-08T10:50:29.9562310Z 30 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-08T10:50:29.9563300Z 31 
2019-12-08T10:50:29.9564180Z 32 
2019-12-08T10:50:29.9564180Z 32 
2019-12-08T10:50:29.9569030Z + ---- $DIR/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
2019-12-08T10:50:29.9570540Z + error[E0425]: cannot find value `no` in this scope
2019-12-08T10:50:29.9572520Z +   --> $DIR/failed-doctest-output.rs:22:1
2019-12-08T10:50:29.9578110Z + LL | no
2019-12-08T10:50:29.9579260Z +    | ^^ not found in this scope
2019-12-08T10:50:29.9580120Z + 
2019-12-08T10:50:29.9580890Z + error: aborting due to previous error
---
2019-12-08T10:50:29.9641230Z 35     $DIR/failed-doctest-output.rs - OtherStruct (line 21)
2019-12-08T10:50:29.9641410Z 
2019-12-08T10:50:29.9641460Z 
2019-12-08T10:50:29.9641550Z The actual stdout differed from the expected stdout.
2019-12-08T10:50:29.9642420Z Actual stdout saved to /Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
2019-12-08T10:50:29.9643220Z To update references, rerun the tests and pass the `--bless` flag
2019-12-08T10:50:29.9643980Z To only update this specific test, also pass `--test-args failed-doctest-output.rs`
2019-12-08T10:50:29.9644180Z error: 1 errors occurred comparing output.
2019-12-08T10:50:29.9644280Z status: exit code: 101
2019-12-08T10:50:29.9644280Z status: exit code: 101
2019-12-08T10:50:29.9645780Z command: "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustdoc" "/Users/runner/runners/2.160.1/work/1/s/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-o" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/rustdoc-ui/failed-doctest-output" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--test" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/rustdoc-ui/failed-doctest-output/auxiliary"
2019-12-08T10:50:29.9647110Z ------------------------------------------
2019-12-08T10:50:29.9647230Z 
2019-12-08T10:50:29.9647340Z running 2 tests
2019-12-08T10:50:29.9648110Z test src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
---
2019-12-08T10:50:29.9650680Z 
2019-12-08T10:50:29.9650750Z stderr:
2019-12-08T10:50:29.9650830Z stderr 1
2019-12-08T10:50:29.9650900Z stderr 2
2019-12-08T10:50:29.9651660Z thread 'main' panicked at 'oh no', src/test/rustdoc-ui/failed-doctest-output.rs:7:1
2019-12-08T10:50:29.9651920Z 
2019-12-08T10:50:29.9651960Z 
2019-12-08T10:50:29.9652690Z ---- src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
2019-12-08T10:50:29.9652830Z error[E0425]: cannot find value `no` in this scope
---
2019-12-08T10:50:29.9662170Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-08T10:50:29.9662320Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-08T10:50:29.9662390Z 
2019-12-08T10:50:29.9662450Z 
2019-12-08T10:50:29.9665780Z command did not execute successfully: "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/runner/runners/2.160.1/work/1/s/src/test/rustdoc-ui" "--build-base" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/test/rustdoc-ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-08T10:50:29.9667020Z 
2019-12-08T10:50:29.9667070Z 
2019-12-08T10:50:29.9667180Z failed to run: /Users/runner/runners/2.160.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-08T10:50:29.9667280Z Build completed unsuccessfully in 1:37:39
2019-12-08T10:50:29.9667280Z Build completed unsuccessfully in 1:37:39
2019-12-08T10:50:29.9667380Z == clock drift check ==
2019-12-08T10:50:29.9667460Z   local time: Sun Dec  8 10:50:29 UTC 2019
2019-12-08T10:50:30.0404410Z   network time: Sun, 08 Dec 2019 10:50:30 GMT
2019-12-08T10:50:30.0406160Z == end clock drift check ==
2019-12-08T10:50:30.0463870Z 
2019-12-08T10:50:30.0632690Z ##[error]Bash exited with code '1'.
2019-12-08T10:50:30.0697710Z ##[section]Starting: Checkout
2019-12-08T10:50:30.0706910Z ==============================================================================
2019-12-08T10:50:30.0707160Z Task         : Get sources
2019-12-08T10:50:30.0707350Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
