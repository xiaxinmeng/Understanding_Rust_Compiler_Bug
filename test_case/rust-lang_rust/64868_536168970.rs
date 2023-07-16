plain
2019-09-28T09:01:16.7872210Z test spec::test_json_encode_decode::aarch64_unknown_linux_gnu ... ok
2019-09-28T09:01:16.7872660Z test spec::test_json_encode_decode::aarch64_unknown_linux_musl ... ok
2019-09-28T09:01:16.7872920Z test spec::test_json_encode_decode::aarch64_unknown_netbsd ... ok
2019-09-28T09:01:16.7873260Z test spec::test_json_encode_decode::aarch64_unknown_none ... ok
2019-09-28T09:01:16.7875230Z test spec::test_json_encode_decode::aarch64_unknown_none_softfloat ... ok
2019-09-28T09:01:16.7884160Z test spec::test_json_encode_decode::aarch64_unknown_redox ... ok
2019-09-28T09:01:16.7889980Z test spec::test_json_encode_decode::aarch64_uwp_windows_msvc ... ok
2019-09-28T09:01:16.7900560Z test spec::test_json_encode_decode::aarch64_wrs_vxworks ... ok
2019-09-28T09:01:16.7901240Z test spec::test_json_encode_decode::arm_linux_androideabi ... ok
---
2019-09-28T09:13:53.8768680Z diff of stdout:
2019-09-28T09:13:53.8768800Z 
2019-09-28T09:13:53.8768920Z 1 
2019-09-28T09:13:53.8769020Z 2 running 2 tests
2019-09-28T09:13:53.8770150Z - test $DIR/failed-doctest-output.rs - OtherStruct (line 21) ... FAILED
2019-09-28T09:13:53.8771230Z 4 test $DIR/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
2019-09-28T09:13:53.8772290Z + test $DIR/failed-doctest-output.rs - OtherStruct (line 21) ... FAILED
2019-09-28T09:13:53.8773040Z 6 failures:
2019-09-28T09:13:53.8773190Z 7 
2019-09-28T09:13:53.8773280Z 
2019-09-28T09:13:53.8773280Z 
2019-09-28T09:13:53.8774020Z - ---- $DIR/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
2019-09-28T09:13:53.8775010Z - error[E0425]: cannot find value `no` in this scope
2019-09-28T09:13:53.8776290Z -   |
2019-09-28T09:13:53.8776860Z - 3 | no
2019-09-28T09:13:53.8777810Z -   | ^^ not found in this scope
2019-09-28T09:13:53.8778710Z - 
2019-09-28T09:13:53.8778710Z - 
2019-09-28T09:13:53.8779650Z - error: aborting due to previous error
2019-09-28T09:13:53.8780750Z - 
2019-09-28T09:13:53.8781930Z - For more information about this error, try `rustc --explain E0425`.
2019-09-28T09:13:53.8783080Z - Couldn't compile the test.
2019-09-28T09:13:53.8784090Z 19 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
2019-09-28T09:13:53.8784550Z 20 Test executable failed (exit code 101).
2019-09-28T09:13:53.8784950Z 
2019-09-28T09:13:53.8785100Z 30 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-28T09:13:53.8785990Z 31 
2019-09-28T09:13:53.8786150Z 32 
2019-09-28T09:13:53.8786150Z 32 
2019-09-28T09:13:53.8786950Z + ---- $DIR/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
2019-09-28T09:13:53.8787410Z + error[E0425]: cannot find value `no` in this scope
2019-09-28T09:13:53.8788720Z +   |
2019-09-28T09:13:53.8788870Z + 3 | no
2019-09-28T09:13:53.8788980Z +   | ^^ not found in this scope
2019-09-28T09:13:53.8789110Z + 
---
2019-09-28T09:13:53.8792310Z 35     $DIR/failed-doctest-output.rs - OtherStruct (line 21)
2019-09-28T09:13:53.8792660Z 
2019-09-28T09:13:53.8793030Z 
2019-09-28T09:13:53.8793460Z The actual stdout differed from the expected stdout.
2019-09-28T09:13:53.8794360Z Actual stdout saved to /Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
2019-09-28T09:13:53.8795410Z To update references, rerun the tests and pass the `--bless` flag
2019-09-28T09:13:53.8796440Z To only update this specific test, also pass `--test-args failed-doctest-output.rs`
2019-09-28T09:13:53.8796960Z error: 1 errors occurred comparing output.
2019-09-28T09:13:53.8797100Z status: exit code: 101
2019-09-28T09:13:53.8797100Z status: exit code: 101
2019-09-28T09:13:53.8798570Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustdoc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-o" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-ui/failed-doctest-output" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--test" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-ui/failed-doctest-output/auxiliary"
2019-09-28T09:13:53.8800100Z ------------------------------------------
2019-09-28T09:13:53.8800510Z 
2019-09-28T09:13:53.8800680Z running 2 tests
2019-09-28T09:13:53.8801490Z test src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
2019-09-28T09:13:53.8801490Z test src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
2019-09-28T09:13:53.8802670Z test src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 21) ... FAILED
2019-09-28T09:13:53.8803190Z 
2019-09-28T09:13:53.8803380Z failures:
2019-09-28T09:13:53.8803430Z 
2019-09-28T09:13:53.8804150Z ---- src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
2019-09-28T09:13:53.8804270Z Test executable failed (exit code 101).
2019-09-28T09:13:53.8804410Z stdout:
2019-09-28T09:13:53.8804500Z stdout 1
2019-09-28T09:13:53.8804570Z stdout 2
2019-09-28T09:13:53.8804620Z 
2019-09-28T09:13:53.8804620Z 
2019-09-28T09:13:53.8804690Z stderr:
2019-09-28T09:13:53.8804770Z stderr 1
2019-09-28T09:13:53.8804840Z stderr 2
2019-09-28T09:13:53.8805540Z thread 'main' panicked at 'oh no', src/test/rustdoc-ui/failed-doctest-output.rs:7:1
2019-09-28T09:13:53.8805750Z 
2019-09-28T09:13:53.8806140Z 
2019-09-28T09:13:53.8806940Z ---- src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
2019-09-28T09:13:53.8807370Z error[E0425]: cannot find value `no` in this scope
---
2019-09-28T09:13:53.8822000Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-28T09:13:53.8822470Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-28T09:13:53.8822600Z 
2019-09-28T09:13:53.8822670Z 
2019-09-28T09:13:53.8825830Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/vsts/agent/2.155.1/work/1/s/src/test/rustdoc-ui" "--build-base" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-28T09:13:53.8827210Z 
2019-09-28T09:13:53.8827290Z 
2019-09-28T09:13:53.8827430Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-09-28T09:13:53.8827570Z Build completed unsuccessfully in 1:36:27
2019-09-28T09:13:53.8827570Z Build completed unsuccessfully in 1:36:27
2019-09-28T09:13:53.8867920Z == clock drift check ==
2019-09-28T09:13:53.8935030Z   local time: Sat Sep 28 09:13:53 UTC 2019
2019-09-28T09:13:53.9821880Z   network time: Sat, 28 Sep 2019 09:13:53 GMT
2019-09-28T09:13:53.9829460Z == end clock drift check ==
2019-09-28T09:13:54.0040010Z ##[error]Bash exited with code '1'.
2019-09-28T09:13:54.0107680Z ##[section]Starting: Upload CPU usage statistics
2019-09-28T09:13:54.0130790Z ==============================================================================
2019-09-28T09:13:54.0130930Z Task         : Bash
2019-09-28T09:13:54.0131030Z Description  : Run a Bash script on macOS, Linux, or Windows
