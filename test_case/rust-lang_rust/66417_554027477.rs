plain
2019-11-14T18:50:12.1037510Z ---- [ui] ui/consts/issue-66342.rs stdout ----
2019-11-14T18:50:12.1037680Z 
2019-11-14T18:50:12.1038470Z error: test compilation failed although it shouldn't!
2019-11-14T18:50:12.1038640Z status: exit code: 1
2019-11-14T18:50:12.1040140Z command: "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.0/work/1/s/src/test/ui/consts/issue-66342.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/test/ui/consts/issue-66342" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/test/ui/consts/issue-66342/auxiliary" "-A" "unused"
2019-11-14T18:50:12.1041180Z ------------------------------------------
2019-11-14T18:50:12.1041250Z 
2019-11-14T18:50:12.1041880Z ------------------------------------------
2019-11-14T18:50:12.1041980Z stderr:
2019-11-14T18:50:12.1041980Z stderr:
2019-11-14T18:50:12.1042860Z ------------------------------------------
2019-11-14T18:50:12.1043010Z error[E0080]: evaluation of constant value failed
2019-11-14T18:50:12.1043770Z   --> /Users/runner/runners/2.160.0/work/1/s/src/test/ui/consts/issue-66342.rs:5:18
2019-11-14T18:50:12.1043920Z    |
2019-11-14T18:50:12.1044570Z LL | fn foo() -> [u8; 4 * 1024 * 1024 * 1024 * 1024] {
2019-11-14T18:50:12.1044720Z    |                  ^^^^^^^^^^^^^^^^^^^^^^ attempt to multiply with overflow
2019-11-14T18:50:12.1044870Z error: aborting due to previous error
2019-11-14T18:50:12.1044920Z 
2019-11-14T18:50:12.1045570Z For more information about this error, try `rustc --explain E0080`.
2019-11-14T18:50:12.1045680Z 
2019-11-14T18:50:12.1045680Z 
2019-11-14T18:50:12.1046440Z ------------------------------------------
2019-11-14T18:50:12.1046530Z 
2019-11-14T18:50:12.1046580Z 
2019-11-14T18:50:12.1047230Z ---- [ui] ui/consts/issue-66397.rs stdout ----
2019-11-14T18:50:12.1047310Z 
2019-11-14T18:50:12.1047940Z error: test compilation failed although it shouldn't!
2019-11-14T18:50:12.1048090Z status: exit code: 1
2019-11-14T18:50:12.1049560Z command: "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.0/work/1/s/src/test/ui/consts/issue-66397.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/test/ui/consts/issue-66397" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/test/ui/consts/issue-66397/auxiliary" "-A" "unused"
2019-11-14T18:50:12.1050880Z ------------------------------------------
2019-11-14T18:50:12.1050980Z 
2019-11-14T18:50:12.1051610Z ------------------------------------------
2019-11-14T18:50:12.1051730Z stderr:
2019-11-14T18:50:12.1051730Z stderr:
2019-11-14T18:50:12.1052370Z ------------------------------------------
2019-11-14T18:50:12.1052510Z error[E0080]: evaluation of constant value failed
2019-11-14T18:50:12.1053210Z   --> /Users/runner/runners/2.160.0/work/1/s/src/test/ui/consts/issue-66397.rs:6:9
2019-11-14T18:50:12.1053360Z    |
2019-11-14T18:50:12.1053450Z LL |     [0; 4 * 1024 * 1024 * 1024 * 1024];
2019-11-14T18:50:12.1053560Z    |         ^^^^^^^^^^^^^^^^^^^^^^ attempt to multiply with overflow
2019-11-14T18:50:12.1053710Z error: aborting due to previous error
2019-11-14T18:50:12.1053760Z 
2019-11-14T18:50:12.1054450Z For more information about this error, try `rustc --explain E0080`.
2019-11-14T18:50:12.1054540Z 
---
2019-11-14T18:50:12.1143260Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-14T18:50:12.1143500Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-14T18:50:12.1160830Z 
2019-11-14T18:50:12.1161150Z 
2019-11-14T18:50:12.1165070Z command did not execute successfully: "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.160.0/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-14T18:50:12.1166170Z 
2019-11-14T18:50:12.1166230Z 
2019-11-14T18:50:12.1176780Z failed to run: /Users/runner/runners/2.160.0/work/1/s/build/bootstrap/debug/bootstrap test
2019-11-14T18:50:12.1177210Z Build completed unsuccessfully in 0:59:52
2019-11-14T18:50:12.1177210Z Build completed unsuccessfully in 0:59:52
2019-11-14T18:50:12.1251040Z == clock drift check ==
2019-11-14T18:50:12.1303190Z   local time: Thu Nov 14 18:50:12 UTC 2019
2019-11-14T18:50:12.2147450Z   network time: Thu, 14 Nov 2019 18:50:12 GMT
2019-11-14T18:50:12.2149900Z == end clock drift check ==
2019-11-14T18:50:12.2196910Z 
2019-11-14T18:50:12.2327770Z ##[error]Bash exited with code '1'.
2019-11-14T18:50:12.2373590Z ##[section]Starting: Checkout
2019-11-14T18:50:12.2376830Z ==============================================================================
2019-11-14T18:50:12.2376970Z Task         : Get sources
2019-11-14T18:50:12.2377060Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
