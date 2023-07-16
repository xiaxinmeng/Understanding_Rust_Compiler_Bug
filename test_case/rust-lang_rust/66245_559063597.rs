plain
2019-11-27T12:15:55.6287750Z failures:
2019-11-27T12:15:55.6287820Z 
2019-11-27T12:15:55.6288540Z ---- [ui] ui/sanitize-cfg.rs#leak stdout ----
2019-11-27T12:15:55.6289010Z 
2019-11-27T12:15:55.6289760Z error in revision `leak`: test compilation failed although it shouldn't!
2019-11-27T12:15:55.6289930Z status: exit code: 1
2019-11-27T12:15:55.6291530Z command: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui/sanitize-cfg.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--cfg" "leak" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/sanitize-cfg.leak" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-Zsanitizer=leak" "--cfg" "leak" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/sanitize-cfg.leak/auxiliary" "-A" "unused"
2019-11-27T12:15:55.6292630Z ------------------------------------------
2019-11-27T12:15:55.6292730Z 
2019-11-27T12:15:55.6293410Z ------------------------------------------
2019-11-27T12:15:55.6293540Z stderr:
2019-11-27T12:15:55.6293540Z stderr:
2019-11-27T12:15:55.6294220Z ------------------------------------------
2019-11-27T12:15:55.6294960Z error: LeakSanitizer only works with the `x86_64-unknown-linux-gnu` target
2019-11-27T12:15:55.6295140Z error: aborting due to previous error
2019-11-27T12:15:55.6295210Z 
2019-11-27T12:15:55.6295260Z 
2019-11-27T12:15:55.6295930Z ------------------------------------------
2019-11-27T12:15:55.6295930Z ------------------------------------------
2019-11-27T12:15:55.6296010Z 
2019-11-27T12:15:55.6296050Z 
2019-11-27T12:15:55.6296720Z ---- [ui] ui/sanitize-cfg.rs#memory stdout ----
2019-11-27T12:15:55.6296800Z 
2019-11-27T12:15:55.6297500Z error in revision `memory`: test compilation failed although it shouldn't!
2019-11-27T12:15:55.6297630Z status: exit code: 1
2019-11-27T12:15:55.6299230Z command: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui/sanitize-cfg.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--cfg" "memory" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/sanitize-cfg.memory" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-Zsanitizer=memory" "--cfg" "memory" "-L" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui/sanitize-cfg.memory/auxiliary" "-A" "unused"
2019-11-27T12:15:55.6300340Z ------------------------------------------
2019-11-27T12:15:55.6300420Z 
2019-11-27T12:15:55.6301070Z ------------------------------------------
2019-11-27T12:15:55.6301200Z stderr:
2019-11-27T12:15:55.6301200Z stderr:
2019-11-27T12:15:55.6301960Z ------------------------------------------
2019-11-27T12:15:55.6302910Z error: MemorySanitizer only works with the `x86_64-unknown-linux-gnu` target
2019-11-27T12:15:55.6303240Z error: aborting due to previous error
2019-11-27T12:15:55.6303600Z 
2019-11-27T12:15:55.6303750Z 
2019-11-27T12:15:55.6304660Z ------------------------------------------
---
2019-11-27T12:15:55.6439510Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-27T12:15:55.6439780Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-27T12:15:55.6461730Z 
2019-11-27T12:15:55.6461870Z 
2019-11-27T12:15:55.6465740Z command did not execute successfully: "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.160.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/test/ui" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.160.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-27T12:15:55.6467220Z 
2019-11-27T12:15:55.6467270Z 
2019-11-27T12:15:55.6477090Z failed to run: /Users/runner/runners/2.160.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-11-27T12:15:55.6477230Z Build completed unsuccessfully in 1:00:13
2019-11-27T12:15:55.6477230Z Build completed unsuccessfully in 1:00:13
2019-11-27T12:15:55.6548510Z == clock drift check ==
2019-11-27T12:15:55.6609680Z   local time: Wed Nov 27 12:15:55 UTC 2019
2019-11-27T12:15:55.7027030Z   network time: Wed, 27 Nov 2019 12:15:55 GMT
2019-11-27T12:15:55.7029590Z == end clock drift check ==
2019-11-27T12:15:55.7078380Z 
2019-11-27T12:15:55.7215620Z ##[error]Bash exited with code '1'.
2019-11-27T12:15:55.7266540Z ##[section]Starting: Checkout
2019-11-27T12:15:55.7269920Z ==============================================================================
2019-11-27T12:15:55.7270040Z Task         : Get sources
2019-11-27T12:15:55.7270230Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
