plain
2019-10-09T03:24:07.7879320Z diff of stderr:
2019-10-09T03:24:07.7879430Z 
2019-10-09T03:24:07.7880090Z 10   --> $DIR/raw-ptr-const-param.rs:7:38
2019-10-09T03:24:07.7880230Z 11    |
2019-10-09T03:24:07.7880320Z 12 LL |     let _: Const<{15 as *const _}> = Const::<{10 as *const _}>;
2019-10-09T03:24:07.7881300Z -    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Scalar(0x000000000000000f) : *const u32`, found `Scalar(0x000000000000000a) : *const u32`
2019-10-09T03:24:07.7881540Z +    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Scalar(0x0000000f) : *const u32`, found `Scalar(0x0000000a) : *const u32`
2019-10-09T03:24:07.7881670Z 14    |
2019-10-09T03:24:07.7882500Z -    = note: expected type `Const<Scalar(0x000000000000000f) : *const u32>`
2019-10-09T03:24:07.7883190Z -               found type `Const<Scalar(0x000000000000000a) : *const u32>`
2019-10-09T03:24:07.7883340Z +    = note: expected type `Const<Scalar(0x0000000f) : *const u32>`
2019-10-09T03:24:07.7883440Z +               found type `Const<Scalar(0x0000000a) : *const u32>`
2019-10-09T03:24:07.7883640Z 18 error: aborting due to previous error
2019-10-09T03:24:07.7883710Z 19 
2019-10-09T03:24:07.7883750Z 
2019-10-09T03:24:07.7883810Z 
2019-10-09T03:24:07.7883810Z 
2019-10-09T03:24:07.7883880Z The actual stderr differed from the expected stderr.
2019-10-09T03:24:07.7884670Z Actual stderr saved to /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/ui/const-generics/raw-ptr-const-param/raw-ptr-const-param.stderr
2019-10-09T03:24:07.7885370Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T03:24:07.7886220Z To only update this specific test, also pass `--test-args const-generics/raw-ptr-const-param.rs`
2019-10-09T03:24:07.7886400Z error: 1 errors occurred comparing output.
2019-10-09T03:24:07.7886490Z status: exit code: 1
2019-10-09T03:24:07.7886490Z status: exit code: 1
2019-10-09T03:24:07.7887880Z command: "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.158.0/work/1/s/src/test/ui/const-generics/raw-ptr-const-param.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/ui/const-generics/raw-ptr-const-param" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/ui/const-generics/raw-ptr-const-param/auxiliary" "-A" "unused"
2019-10-09T03:24:07.7889170Z ------------------------------------------
2019-10-09T03:24:07.7889240Z 
2019-10-09T03:24:07.7889850Z ------------------------------------------
2019-10-09T03:24:07.7889950Z stderr:
---
2019-10-09T03:24:07.7892080Z 
2019-10-09T03:24:07.7892360Z error[E0308]: mismatched types
2019-10-09T03:24:07.7893130Z   --> /Users/vsts/agent/2.158.0/work/1/s/src/test/ui/const-generics/raw-ptr-const-param.rs:7:38
2019-10-09T03:24:07.7893280Z    |
2019-10-09T03:24:07.7893370Z LL |     let _: Const<{15 as *const _}> = Const::<{10 as *const _}>; //~ mismatched types
2019-10-09T03:24:07.7893520Z    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Scalar(0x0000000f) : *const u32`, found `Scalar(0x0000000a) : *const u32`
2019-10-09T03:24:07.7893650Z    |
2019-10-09T03:24:07.7893730Z    = note: expected type `Const<Scalar(0x0000000f) : *const u32>`
2019-10-09T03:24:07.7893860Z               found type `Const<Scalar(0x0000000a) : *const u32>`
2019-10-09T03:24:07.7894010Z error: aborting due to previous error
2019-10-09T03:24:07.7894060Z 
2019-10-09T03:24:07.7894730Z For more information about this error, try `rustc --explain E0308`.
2019-10-09T03:24:07.7894820Z 
---
2019-10-09T03:24:07.7958000Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-09T03:24:07.7958240Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-09T03:24:07.7973810Z 
2019-10-09T03:24:07.7974270Z 
2019-10-09T03:24:07.7977970Z command did not execute successfully: "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.158.0/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-09T03:24:07.7979220Z 
2019-10-09T03:24:07.7979270Z 
2019-10-09T03:24:07.7989460Z failed to run: /Users/vsts/agent/2.158.0/work/1/s/build/bootstrap/debug/bootstrap test
2019-10-09T03:24:07.7989640Z Build completed unsuccessfully in 1:33:38
2019-10-09T03:24:07.7989640Z Build completed unsuccessfully in 1:33:38
2019-10-09T03:24:07.8075440Z == clock drift check ==
2019-10-09T03:24:07.8129440Z   local time: Wed Oct  9 03:24:07 UTC 2019
2019-10-09T03:24:07.9001710Z   network time: Wed, 09 Oct 2019 03:24:07 GMT
2019-10-09T03:24:07.9002330Z == end clock drift check ==
2019-10-09T03:24:07.9166740Z ##[error]Bash exited with code '1'.
2019-10-09T03:24:07.9212420Z ##[section]Starting: Upload CPU usage statistics
2019-10-09T03:24:07.9241100Z ==============================================================================
2019-10-09T03:24:07.9241440Z Task         : Bash
2019-10-09T03:24:07.9241650Z Description  : Run a Bash script on macOS, Linux, or Windows
