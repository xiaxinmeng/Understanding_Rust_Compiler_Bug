plain
2019-10-02T23:35:59.6426900Z test [ui] ui/consts/const-expr-in-fixed-length-vec.rs ... ok
2019-10-02T23:35:59.7085720Z test [ui] ui/consts/const-extern-fn/const-extern-fn-call-extern-fn.rs ... ok
2019-10-02T23:35:59.7099120Z test [ui] ui/consts/const-eval/issue-52475.rs ... ok
2019-10-02T23:35:59.7511540Z test [ui] ui/consts/const-expr-in-vec-repeat.rs ... ok
2019-10-02T23:35:59.8015470Z test [ui] ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs ... ok
2019-10-02T23:35:59.8559740Z test [ui] ui/consts/const-extern-fn/feature-gate-const_extern_fn.rs ... ok
2019-10-02T23:35:59.8563740Z test [ui] ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs ... ok
2019-10-02T23:36:00.2783530Z test [ui] ui/consts/const-extern-function.rs ... ok
2019-10-02T23:36:00.2816240Z test [ui] ui/consts/const-extern-fn/const-extern-fn.rs ... ok
2019-10-02T23:36:00.3867390Z test [ui] ui/consts/const-fields-and-indexing.rs ... ok
2019-10-02T23:36:00.4282850Z test [ui] ui/consts/const-fn-destructuring-arg.rs ... ok
---
2019-10-02T23:46:45.1492040Z diff of stderr:
2019-10-02T23:46:45.1492110Z 
2019-10-02T23:46:45.1492220Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-10-02T23:46:45.1492360Z 9    |
2019-10-02T23:46:45.1493070Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-10-02T23:46:45.1493830Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-10-02T23:46:45.1494620Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-10-02T23:46:45.1494750Z 12 
2019-10-02T23:46:45.1495540Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-02T23:46:45.1496560Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-02T23:46:45.1497280Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-10-02T23:46:45.1497410Z 15    |
2019-10-02T23:46:45.1498120Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-10-02T23:46:45.1498280Z 
2019-10-02T23:46:45.1498640Z The actual stderr differed from the expected stderr.
2019-10-02T23:46:45.1499560Z Actual stderr saved to /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-10-02T23:46:45.1499560Z Actual stderr saved to /Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-10-02T23:46:45.1500470Z To update references, rerun the tests and pass the `--bless` flag
2019-10-02T23:46:45.1501180Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-10-02T23:46:45.1501360Z error: 1 errors occurred comparing output.
2019-10-02T23:46:45.1501460Z status: exit code: 1
2019-10-02T23:46:45.1501460Z status: exit code: 1
2019-10-02T23:46:45.1502880Z command: "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/vsts/agent/2.158.0/work/1/s/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-10-02T23:46:45.1503900Z ------------------------------------------
2019-10-02T23:46:45.1503980Z 
2019-10-02T23:46:45.1504620Z ------------------------------------------
2019-10-02T23:46:45.1504730Z stderr:
2019-10-02T23:46:45.1504730Z stderr:
2019-10-02T23:46:45.1505360Z ------------------------------------------
2019-10-02T23:46:45.1505470Z error[E0308]: cannot coerce intrinsics to function pointers
2019-10-02T23:46:45.1506190Z   --> /Users/vsts/agent/2.158.0/work/1/s/src/test/ui/reify-intrinsic.rs:6:64
2019-10-02T23:46:45.1506480Z    |
2019-10-02T23:46:45.1507180Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-10-02T23:46:45.1507490Z    |                                                                |
2019-10-02T23:46:45.1507630Z    |                                                                cannot coerce intrinsics to function pointers
2019-10-02T23:46:45.1508110Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-10-02T23:46:45.1508230Z    |
2019-10-02T23:46:45.1508230Z    |
2019-10-02T23:46:45.1508990Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-10-02T23:46:45.1509740Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-10-02T23:46:45.1509880Z 
2019-10-02T23:46:45.1510690Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-10-02T23:46:45.1511630Z    |
2019-10-02T23:46:45.1511630Z    |
2019-10-02T23:46:45.1512340Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-10-02T23:46:45.1512600Z 
2019-10-02T23:46:45.1512690Z error: aborting due to 2 previous errors
2019-10-02T23:46:45.1512750Z 
2019-10-02T23:46:45.1512840Z Some errors have detailed explanations: E0308, E0606.
---
2019-10-02T23:46:45.1578790Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-02T23:46:45.1579030Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-02T23:46:45.1596300Z 
2019-10-02T23:46:45.1596750Z 
2019-10-02T23:46:45.1600180Z command did not execute successfully: "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/vsts/agent/2.158.0/work/1/s/src/test/ui" "--build-base" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/vsts/agent/2.158.0/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-02T23:46:45.1601170Z 
2019-10-02T23:46:45.1601220Z 
2019-10-02T23:46:45.1611370Z failed to run: /Users/vsts/agent/2.158.0/work/1/s/build/bootstrap/debug/bootstrap test
2019-10-02T23:46:45.1611520Z Build completed unsuccessfully in 0:58:34
2019-10-02T23:46:45.1611520Z Build completed unsuccessfully in 0:58:34
2019-10-02T23:46:45.1685600Z == clock drift check ==
2019-10-02T23:46:45.1753490Z   local time: Wed Oct  2 23:46:45 UTC 2019
2019-10-02T23:46:45.2568140Z   network time: Wed, 02 Oct 2019 23:46:45 GMT
2019-10-02T23:46:45.2575560Z == end clock drift check ==
2019-10-02T23:46:45.2820800Z ##[error]Bash exited with code '1'.
2019-10-02T23:46:45.2869030Z ##[section]Starting: Upload CPU usage statistics
2019-10-02T23:46:45.2912230Z ==============================================================================
2019-10-02T23:46:45.2912350Z Task         : Bash
2019-10-02T23:46:45.2912440Z Description  : Run a Bash script on macOS, Linux, or Windows
