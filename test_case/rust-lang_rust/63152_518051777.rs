plain
2019-08-05T00:55:47.3910920Z 
2019-08-05T00:55:47.3911750Z ---- [ui] ui/consts/issue-55878.rs stdout ----
2019-08-05T00:55:47.3911880Z diff of stderr:
2019-08-05T00:55:47.3911930Z 
2019-08-05T00:55:47.3912630Z - error[E0080]: the type `[u8; 18446744073709551615]` is too big for the current architecture
2019-08-05T00:55:47.3912780Z + error[E0080]: the type `[u8; 4294967295]` is too big for the current architecture
2019-08-05T00:55:47.3913420Z 2   --> $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-08-05T00:55:47.3913560Z 3    |
2019-08-05T00:55:47.3913620Z 4 LL |     intrinsics::size_of::<T>()
2019-08-05T00:55:47.3913730Z 
2019-08-05T00:55:47.3914080Z The actual stderr differed from the expected stderr.
2019-08-05T00:55:47.3915100Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/consts/issue-55878/issue-55878.stderr
2019-08-05T00:55:47.3915100Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/consts/issue-55878/issue-55878.stderr
2019-08-05T00:55:47.3915820Z To update references, rerun the tests and pass the `--bless` flag
2019-08-05T00:55:47.3916520Z To only update this specific test, also pass `--test-args consts/issue-55878.rs`
2019-08-05T00:55:47.3916700Z error: 1 errors occurred comparing output.
2019-08-05T00:55:47.3916790Z status: exit code: 1
2019-08-05T00:55:47.3916790Z status: exit code: 1
2019-08-05T00:55:47.3918200Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/consts/issue-55878.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/consts/issue-55878" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/consts/issue-55878/auxiliary" "-A" "unused"
2019-08-05T00:55:47.3919190Z ------------------------------------------
2019-08-05T00:55:47.3919270Z 
2019-08-05T00:55:47.3920010Z ------------------------------------------
2019-08-05T00:55:47.3920110Z stderr:
2019-08-05T00:55:47.3920110Z stderr:
2019-08-05T00:55:47.3920730Z ------------------------------------------
2019-08-05T00:55:47.3920840Z error[E0080]: the type `[u8; 4294967295]` is too big for the current architecture
2019-08-05T00:55:47.3921530Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/libcore/mem/mod.rs:243:5
2019-08-05T00:55:47.3921640Z    |
2019-08-05T00:55:47.3921730Z LL |     intrinsics::size_of::<T>()
2019-08-05T00:55:47.3921900Z    | 
2019-08-05T00:55:47.3922580Z   ::: /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/consts/issue-55878.rs:3:26
2019-08-05T00:55:47.3922720Z    |
2019-08-05T00:55:47.3922720Z    |
2019-08-05T00:55:47.3922800Z LL |     println!("Size: {}", std::mem::size_of::<[u8; std::u64::MAX as usize]>());
2019-08-05T00:55:47.3923590Z 
2019-08-05T00:55:47.3923680Z error: aborting due to previous error
2019-08-05T00:55:47.3923730Z 
2019-08-05T00:55:47.3924380Z For more information about this error, try `rustc --explain E0080`.
2019-08-05T00:55:47.3924380Z For more information about this error, try `rustc --explain E0080`.
2019-08-05T00:55:47.3924460Z 
2019-08-05T00:55:47.3925060Z ------------------------------------------
2019-08-05T00:55:47.3925150Z 
2019-08-05T00:55:47.3925200Z 
2019-08-05T00:55:47.3925850Z ---- [ui] ui/huge-array-simple.rs stdout ----
2019-08-05T00:55:47.3925980Z diff of stderr:
2019-08-05T00:55:47.3926020Z 
2019-08-05T00:55:47.3926110Z 1 error: the type `[u8; N]` is too big for the current architecture
2019-08-05T00:55:47.3926740Z -   --> $DIR/huge-array-simple.rs:12:9
2019-08-05T00:55:47.3927410Z +   --> $DIR/huge-array-simple.rs:18:9
2019-08-05T00:55:47.3927510Z 3    |
2019-08-05T00:55:47.3928160Z - LL |     let _fat : [u8; (1<<61)+(1<<31)] =
2019-08-05T00:55:47.3928270Z + LL |     let _fat : [u8; (1<<31)+(1<<15)] =
2019-08-05T00:55:47.3928430Z 6 
2019-08-05T00:55:47.3928520Z 7 error: aborting due to previous error
2019-08-05T00:55:47.3928570Z 
2019-08-05T00:55:47.3928610Z 
2019-08-05T00:55:47.3928610Z 
2019-08-05T00:55:47.3928690Z The actual stderr differed from the expected stderr.
2019-08-05T00:55:47.3929450Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/huge-array-simple/huge-array-simple.stderr
2019-08-05T00:55:47.3930160Z To update references, rerun the tests and pass the `--bless` flag
2019-08-05T00:55:47.3930970Z To only update this specific test, also pass `--test-args huge-array-simple.rs`
2019-08-05T00:55:47.3931130Z error: 1 errors occurred comparing output.
2019-08-05T00:55:47.3931220Z status: exit code: 1
2019-08-05T00:55:47.3931220Z status: exit code: 1
2019-08-05T00:55:47.3932850Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/huge-array-simple.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/huge-array-simple" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/huge-array-simple/auxiliary" "-A" "unused"
2019-08-05T00:55:47.3934040Z ------------------------------------------
2019-08-05T00:55:47.3934110Z 
2019-08-05T00:55:47.3934750Z ------------------------------------------
2019-08-05T00:55:47.3934850Z stderr:
2019-08-05T00:55:47.3934850Z stderr:
2019-08-05T00:55:47.3935470Z ------------------------------------------
2019-08-05T00:55:47.3935590Z error: the type `[u8; 2147516416]` is too big for the current architecture
2019-08-05T00:55:47.3936420Z    |
2019-08-05T00:55:47.3936420Z    |
2019-08-05T00:55:47.3936510Z LL |     let _fat : [u8; (1<<31)+(1<<15)] =
2019-08-05T00:55:47.3936660Z 
2019-08-05T00:55:47.3936720Z error: aborting due to previous error
2019-08-05T00:55:47.3936790Z 
2019-08-05T00:55:47.3936830Z 
2019-08-05T00:55:47.3936830Z 
2019-08-05T00:55:47.3937450Z ------------------------------------------
2019-08-05T00:55:47.3937550Z 
2019-08-05T00:55:47.3937590Z 
2019-08-05T00:55:47.3938200Z ---- [ui] ui/issues/issue-15919.rs stdout ----
2019-08-05T00:55:47.3938320Z diff of stderr:
2019-08-05T00:55:47.3938370Z 
2019-08-05T00:55:47.3938460Z 1 error: the type `[usize; N]` is too big for the current architecture
2019-08-05T00:55:47.3939080Z -   --> $DIR/issue-15919.rs:15:9
2019-08-05T00:55:47.3939720Z +   --> $DIR/issue-15919.rs:10:9
2019-08-05T00:55:47.3939820Z 3    |
2019-08-05T00:55:47.3940470Z - LL |     let x = [0usize; 0xffff_ffff_ffff_ffff];
2019-08-05T00:55:47.3940590Z + LL |     let x = [0usize; 0xffff_ffff];
2019-08-05T00:55:47.3940750Z 6 
2019-08-05T00:55:47.3940830Z 7 error: aborting due to previous error
2019-08-05T00:55:47.3940880Z 
2019-08-05T00:55:47.3940920Z 
2019-08-05T00:55:47.3940920Z 
2019-08-05T00:55:47.3941000Z The actual stderr differed from the expected stderr.
2019-08-05T00:55:47.3941770Z Actual stderr saved to /Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/issues/issue-15919/issue-15919.stderr
2019-08-05T00:55:47.3942480Z To update references, rerun the tests and pass the `--bless` flag
2019-08-05T00:55:47.3943170Z To only update this specific test, also pass `--test-args issues/issue-15919.rs`
2019-08-05T00:55:47.3943350Z error: 1 errors occurred comparing output.
2019-08-05T00:55:47.3943440Z status: exit code: 1
2019-08-05T00:55:47.3943440Z status: exit code: 1
2019-08-05T00:55:47.3944840Z command: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui/issues/issue-15919.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/issues/issue-15919" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui/issues/issue-15919/auxiliary" "-A" "unused"
2019-08-05T00:55:47.3945810Z ------------------------------------------
2019-08-05T00:55:47.3945880Z 
2019-08-05T00:55:47.3946510Z ------------------------------------------
2019-08-05T00:55:47.3946610Z stderr:
2019-08-05T00:55:47.3946610Z stderr:
2019-08-05T00:55:47.3947230Z ------------------------------------------
2019-08-05T00:55:47.3947340Z error: the type `[usize; 4294967295]` is too big for the current architecture
2019-08-05T00:55:47.3948240Z   --> /Users/vsts/agent/2.155.1/work/1/s/src/test/ui/issues/issue-15919.rs:10:9
2019-08-05T00:55:47.3948530Z    |
2019-08-05T00:55:47.3948630Z LL |     let x = [0usize; 0xffff_ffff];
2019-08-05T00:55:47.3948760Z 
2019-08-05T00:55:47.3948830Z error: aborting due to previous error
2019-08-05T00:55:47.3948880Z 
2019-08-05T00:55:47.3948930Z 
---
2019-08-05T00:55:47.4013770Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-05T00:55:47.4013960Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-05T00:55:47.4030720Z 
2019-08-05T00:55:47.4030830Z 
2019-08-05T00:55:47.4034500Z command did not execute successfully: "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.155.1/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.155.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-05T00:55:47.4035370Z 
2019-08-05T00:55:47.4035420Z 
2019-08-05T00:55:47.4045800Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-08-05T00:55:47.4045920Z Build completed unsuccessfully in 1:04:12
2019-08-05T00:55:47.4045920Z Build completed unsuccessfully in 1:04:12
2019-08-05T00:55:47.4283560Z ##[error]Bash exited with code '1'.
2019-08-05T00:55:47.4348220Z ##[section]Starting: Upload CPU usage statistics
2019-08-05T00:55:47.4353130Z ==============================================================================
2019-08-05T00:55:47.4353250Z Task         : Bash
2019-08-05T00:55:47.4353330Z Description  : Run a Bash script on macOS, Linux, or Windows
