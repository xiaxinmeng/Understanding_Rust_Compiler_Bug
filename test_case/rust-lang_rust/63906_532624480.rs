plain
2019-09-18T10:26:29.1423640Z diff of stdout:
2019-09-18T10:26:29.1423690Z 
2019-09-18T10:26:29.1423770Z 1 
2019-09-18T10:26:29.1423840Z 2 running 2 tests
2019-09-18T10:26:29.1424590Z - test $DIR/failed-doctest-output.rs - OtherStruct (line 21) ... FAILED
2019-09-18T10:26:29.1425320Z 4 test $DIR/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
2019-09-18T10:26:29.1426110Z + test $DIR/failed-doctest-output.rs - OtherStruct (line 21) ... FAILED
2019-09-18T10:26:29.1437090Z 6 failures:
2019-09-18T10:26:29.1437590Z 7 
2019-09-18T10:26:29.1437660Z 
2019-09-18T10:26:29.1437660Z 
2019-09-18T10:26:29.1438470Z - ---- $DIR/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
2019-09-18T10:26:29.1439300Z - error[E0425]: cannot find value `no` in this scope
2019-09-18T10:26:29.1440930Z -   |
2019-09-18T10:26:29.1441540Z - 3 | no
2019-09-18T10:26:29.1442180Z -   | ^^ not found in this scope
2019-09-18T10:26:29.1442780Z - 
2019-09-18T10:26:29.1442780Z - 
2019-09-18T10:26:29.1443430Z - error: aborting due to previous error
2019-09-18T10:26:29.1444020Z - 
2019-09-18T10:26:29.1444710Z - For more information about this error, try `rustc --explain E0425`.
2019-09-18T10:26:29.1445350Z - Couldn't compile the test.
2019-09-18T10:26:29.1446050Z 19 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
2019-09-18T10:26:29.1446260Z 20 Test executable failed (exit code 101).
2019-09-18T10:26:29.1446640Z 
2019-09-18T10:26:29.1446810Z 30 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-18T10:26:29.1446910Z 31 
2019-09-18T10:26:29.1446990Z 32 
2019-09-18T10:26:29.1446990Z 32 
2019-09-18T10:26:29.1447720Z + ---- $DIR/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
2019-09-18T10:26:29.1447860Z + error[E0425]: cannot find value `no` in this scope
2019-09-18T10:26:29.1448610Z +   |
2019-09-18T10:26:29.1448700Z + 3 | no
2019-09-18T10:26:29.1448770Z +   | ^^ not found in this scope
2019-09-18T10:26:29.1448850Z + 
---
2019-09-18T10:26:29.1451180Z 35     $DIR/failed-doctest-output.rs - OtherStruct (line 21)
2019-09-18T10:26:29.1451280Z 
2019-09-18T10:26:29.1451330Z 
2019-09-18T10:26:29.1451410Z The actual stdout differed from the expected stdout.
2019-09-18T10:26:29.1452210Z Actual stdout saved to /Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
2019-09-18T10:26:29.1452940Z To update references, rerun the tests and pass the `--bless` flag
2019-09-18T10:26:29.1453650Z To only update this specific test, also pass `--test-args failed-doctest-output.rs`
2019-09-18T10:26:29.1453830Z error: 1 errors occurred comparing output.
2019-09-18T10:26:29.1453910Z status: exit code: 101
2019-09-18T10:26:29.1453910Z status: exit code: 101
2019-09-18T10:26:29.1455340Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustdoc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-o" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-ui/failed-doctest-output" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--test" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-ui/failed-doctest-output/auxiliary"
2019-09-18T10:26:29.1456320Z ------------------------------------------
2019-09-18T10:26:29.1456390Z 
2019-09-18T10:26:29.1456470Z running 2 tests
2019-09-18T10:26:29.1457150Z test src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
2019-09-18T10:26:29.1457150Z test src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
2019-09-18T10:26:29.1457860Z test src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 21) ... FAILED
2019-09-18T10:26:29.1457970Z 
2019-09-18T10:26:29.1458040Z failures:
2019-09-18T10:26:29.1458090Z 
2019-09-18T10:26:29.1458780Z ---- src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
2019-09-18T10:26:29.1458910Z Test executable failed (exit code 101).
2019-09-18T10:26:29.1459050Z stdout:
2019-09-18T10:26:29.1459400Z stdout 1
2019-09-18T10:26:29.1459470Z stdout 2
2019-09-18T10:26:29.1459520Z 
2019-09-18T10:26:29.1459520Z 
2019-09-18T10:26:29.1459590Z stderr:
2019-09-18T10:26:29.1459660Z stderr 1
2019-09-18T10:26:29.1459720Z stderr 2
2019-09-18T10:26:29.1460460Z thread 'main' panicked at 'oh no', src/test/rustdoc-ui/failed-doctest-output.rs:7:1
2019-09-18T10:26:29.1460650Z 
2019-09-18T10:26:29.1460710Z 
2019-09-18T10:26:29.1461380Z ---- src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
2019-09-18T10:26:29.1461520Z error[E0425]: cannot find value `no` in this scope
---
2019-09-18T10:26:29.1470690Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-18T10:26:29.1470820Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-18T10:26:29.1470900Z 
2019-09-18T10:26:29.1470940Z 
2019-09-18T10:26:29.1473920Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/vsts/agent/2.155.1/work/1/s/src/test/rustdoc-ui" "--build-base" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/test/rustdoc-ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-18T10:26:29.1475080Z 
2019-09-18T10:26:29.1475130Z 
2019-09-18T10:26:29.1475230Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-09-18T10:26:29.1475320Z Build completed unsuccessfully in 1:39:14
2019-09-18T10:26:29.1475320Z Build completed unsuccessfully in 1:39:14
2019-09-18T10:26:29.1523060Z == clock drift check ==
2019-09-18T10:26:29.1589310Z   local time: Wed Sep 18 10:26:29 UTC 2019
2019-09-18T10:26:29.3833400Z   network time: Wed, 18 Sep 2019 10:26:29 GMT
2019-09-18T10:26:29.3835950Z == end clock drift check ==
2019-09-18T10:26:29.4050410Z ##[error]Bash exited with code '1'.
2019-09-18T10:26:29.4136230Z ##[section]Starting: Upload CPU usage statistics
2019-09-18T10:26:29.4169560Z ==============================================================================
2019-09-18T10:26:29.4169830Z Task         : Bash
2019-09-18T10:26:29.4170050Z Description  : Run a Bash script on macOS, Linux, or Windows
