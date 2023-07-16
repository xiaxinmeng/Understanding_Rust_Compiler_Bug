plain
2019-08-06T00:16:29.8445180Z 
2019-08-06T00:16:29.8445840Z ---- [ui] ui/huge-array-simple-64.rs stdout ----
2019-08-06T00:16:29.8445960Z diff of stderr:
2019-08-06T00:16:29.8446010Z 
2019-08-06T00:16:29.8446110Z 1 error: the type `[u8; 2147516416]` is too big for the current architecture
2019-08-06T00:16:29.8447170Z -   --> $DIR/huge-array-simple-32.rs:9:9
2019-08-06T00:16:29.8447810Z +   --> $DIR/huge-array-simple-64.rs:9:9
2019-08-06T00:16:29.8447920Z 3    |
2019-08-06T00:16:29.8448020Z 4 LL |     let _fat: [u8; (1<<31)+(1<<15)] =
2019-08-06T00:16:29.8448150Z 
2019-08-06T00:16:29.8448200Z 
2019-08-06T00:16:29.8448280Z The actual stderr differed from the expected stderr.
2019-08-06T00:16:29.8448280Z The actual stderr differed from the expected stderr.
2019-08-06T00:16:29.8449060Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/huge-array-simple-64/huge-array-simple-64.stderr
2019-08-06T00:16:29.8449760Z To update references, rerun the tests and pass the `--bless` flag
2019-08-06T00:16:29.8450500Z To only update this specific test, also pass `--test-args huge-array-simple-64.rs`
2019-08-06T00:16:29.8450680Z error: 1 errors occurred comparing output.
2019-08-06T00:16:29.8450760Z status: exit code: 1
2019-08-06T00:16:29.8450760Z status: exit code: 1
2019-08-06T00:16:29.8452380Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/huge-array-simple-64.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/huge-array-simple-64" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/huge-array-simple-64/auxiliary" "-A" "unused"
2019-08-06T00:16:29.8453430Z ------------------------------------------
2019-08-06T00:16:29.8453520Z 
2019-08-06T00:16:29.8454130Z ------------------------------------------
2019-08-06T00:16:29.8454260Z stderr:
2019-08-06T00:16:29.8454260Z stderr:
2019-08-06T00:16:29.8454840Z ------------------------------------------
2019-08-06T00:16:29.8454980Z error: the type `[u8; 2147516416]` is too big for the current architecture
2019-08-06T00:16:29.8455670Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/huge-array-simple-64.rs:9:9
2019-08-06T00:16:29.8456650Z    |
2019-08-06T00:16:29.8456770Z LL |     let _fat: [u8; (1<<31)+(1<<15)] = //~ ERROR too big for the current architecture
2019-08-06T00:16:29.8457050Z 
2019-08-06T00:16:29.8457180Z error: aborting due to previous error
2019-08-06T00:16:29.8457260Z 
2019-08-06T00:16:29.8457450Z 
2019-08-06T00:16:29.8457450Z 
2019-08-06T00:16:29.8458190Z ------------------------------------------
2019-08-06T00:16:29.8458560Z 
2019-08-06T00:16:29.8458710Z 
2019-08-06T00:16:29.8459430Z ---- [ui] ui/issues/issue-15919-32.rs stdout ----
2019-08-06T00:16:29.8459870Z diff of stderr:
2019-08-06T00:16:29.8459980Z 
2019-08-06T00:16:29.8460130Z 1 error: the type `[usize; 4294967295]` is too big for the current architecture
2019-08-06T00:16:29.8460850Z -   --> $DIR/issue-15919-64.rs:8:9
2019-08-06T00:16:29.8461820Z +   --> $DIR/issue-15919-32.rs:8:9
2019-08-06T00:16:29.8462250Z 3    |
2019-08-06T00:16:29.8462420Z 4 LL |     let x = [0usize; 0xffff_ffff];
2019-08-06T00:16:29.8462630Z 
2019-08-06T00:16:29.8462700Z 
2019-08-06T00:16:29.8462850Z The actual stderr differed from the expected stderr.
2019-08-06T00:16:29.8463720Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/issues/issue-15919-32/issue-15919-32.stderr
2019-08-06T00:16:29.8463720Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/issues/issue-15919-32/issue-15919-32.stderr
2019-08-06T00:16:29.8464820Z To update references, rerun the tests and pass the `--bless` flag
2019-08-06T00:16:29.8465890Z To only update this specific test, also pass `--test-args issues/issue-15919-32.rs`
2019-08-06T00:16:29.8466450Z error: 1 errors occurred comparing output.
2019-08-06T00:16:29.8466560Z status: exit code: 1
2019-08-06T00:16:29.8466560Z status: exit code: 1
2019-08-06T00:16:29.8468080Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/issues/issue-15919-32.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/issues/issue-15919-32" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/issues/issue-15919-32/auxiliary" "-A" "unused"
2019-08-06T00:16:29.8469740Z ------------------------------------------
2019-08-06T00:16:29.8470120Z 
2019-08-06T00:16:29.8470900Z ------------------------------------------
2019-08-06T00:16:29.8471330Z stderr:
2019-08-06T00:16:29.8471330Z stderr:
2019-08-06T00:16:29.8472050Z ------------------------------------------
2019-08-06T00:16:29.8472500Z error: the type `[usize; 4294967295]` is too big for the current architecture
2019-08-06T00:16:29.8473320Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/issues/issue-15919-32.rs:8:9
2019-08-06T00:16:29.8473740Z    |
2019-08-06T00:16:29.8473930Z LL |     let x = [0usize; 0xffff_ffff]; //~ ERROR too big
2019-08-06T00:16:29.8474140Z 
2019-08-06T00:16:29.8474470Z error: aborting due to previous error
2019-08-06T00:16:29.8474980Z 
2019-08-06T00:16:29.8475100Z 
---
2019-08-06T00:16:29.8528710Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-06T00:16:29.8528900Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-06T00:16:29.8544340Z 
2019-08-06T00:16:29.8544890Z 
2019-08-06T00:16:29.8548500Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-06T00:16:29.8550330Z 
2019-08-06T00:16:29.8550450Z 
2019-08-06T00:16:29.8561230Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-08-06T00:16:29.8561410Z Build completed unsuccessfully in 1:06:46
2019-08-06T00:16:29.8561410Z Build completed unsuccessfully in 1:06:46
2019-08-06T00:16:29.8809060Z ##[error]Bash exited with code '1'.
2019-08-06T00:16:29.8856920Z ##[section]Starting: Upload CPU usage statistics
2019-08-06T00:16:29.8861910Z ==============================================================================
2019-08-06T00:16:29.8862020Z Task         : Bash
2019-08-06T00:16:29.8862110Z Description  : Run a Bash script on macOS, Linux, or Windows
