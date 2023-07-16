plain
2019-11-17T14:04:13.6857426Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-17T14:04:13.7036998Z ##[command]git config gc.auto 0
2019-11-17T14:04:13.7121906Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-17T14:04:13.7186162Z ##[command]git config --get-all http.proxy
2019-11-17T14:04:13.7348365Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66489/merge:refs/remotes/pull/66489/merge
---
2019-11-17T15:04:34.3328578Z .................................................................................................... 1500/9251
2019-11-17T15:04:40.6673614Z .................................................................................................... 1600/9251
2019-11-17T15:04:49.5773822Z .................................................................................................... 1700/9251
2019-11-17T15:04:59.0786962Z ........i........................................................................................... 1800/9251
2019-11-17T15:05:05.7814690Z .............................................................................................iiiii.. 1900/9251
2019-11-17T15:05:28.4643153Z .................................................................................................... 2100/9251
2019-11-17T15:05:30.9648256Z .................................................................................................... 2200/9251
2019-11-17T15:05:33.5618155Z .................................................................................................... 2300/9251
2019-11-17T15:05:40.1255306Z .................................................................................................... 2400/9251
---
2019-11-17T15:09:35.5241871Z .................................................................................................... 5400/9251
2019-11-17T15:09:46.8205099Z ...............................................................................i.................... 5500/9251
2019-11-17T15:09:55.1728338Z .................................................................................................... 5600/9251
2019-11-17T15:10:01.9597623Z .................................................................................................... 5700/9251
2019-11-17T15:10:13.0506365Z .................................................................ii...i..ii...........i............. 5800/9251
2019-11-17T15:10:36.5261971Z .................................................................................................... 6000/9251
2019-11-17T15:10:45.4794807Z .................................................................................................... 6100/9251
2019-11-17T15:10:45.4794807Z .................................................................................................... 6100/9251
2019-11-17T15:10:53.0329308Z ....................................................................................i..ii........... 6200/9251
2019-11-17T15:11:08.0181549Z .................................................................................................... 6300/9251
2019-11-17T15:11:18.2405832Z .........................................................................................FF.F.FF.... 6400/9251
2019-11-17T15:11:27.1166581Z .................................................................................................... 6600/9251
2019-11-17T15:11:29.6672780Z .........................................i.......................................................... 6700/9251
2019-11-17T15:11:32.9580530Z .................................................................................................... 6800/9251
2019-11-17T15:11:39.6187744Z .................................................................................................... 6900/9251
---
2019-11-17T15:16:32.3868619Z 
2019-11-17T15:16:32.3869449Z ---- [ui] ui/panic-runtime/abort-link-to-unwind-dylib.rs stdout ----
2019-11-17T15:16:32.3869684Z diff of stderr:
2019-11-17T15:16:32.3869817Z 
2019-11-17T15:16:32.3870893Z - error: the linked panic runtime `panic_unwind` is not compiled with this crate's panic strategy `abort`
2019-11-17T15:16:32.3871180Z + error: the crate `panic_abort` does not have the panic strategy `abort`
2019-11-17T15:16:32.3871490Z 3 error: aborting due to previous error
2019-11-17T15:16:32.3871649Z 4 
2019-11-17T15:16:32.3871794Z 
2019-11-17T15:16:32.3871915Z 
2019-11-17T15:16:32.3871915Z 
2019-11-17T15:16:32.3872078Z The actual stderr differed from the expected stderr.
2019-11-17T15:16:32.3872616Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwind-dylib/abort-link-to-unwind-dylib.stderr
2019-11-17T15:16:32.3873085Z To update references, rerun the tests and pass the `--bless` flag
2019-11-17T15:16:32.3873608Z To only update this specific test, also pass `--test-args panic-runtime/abort-link-to-unwind-dylib.rs`
2019-11-17T15:16:32.3874101Z error: 1 errors occurred comparing output.
2019-11-17T15:16:32.3874425Z status: exit code: 1
2019-11-17T15:16:32.3874425Z status: exit code: 1
2019-11-17T15:16:32.3875353Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/abort-link-to-unwind-dylib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwind-dylib" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-C" "prefer-dynamic" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwind-dylib/auxiliary" "-A" "unused"
2019-11-17T15:16:32.3875948Z ------------------------------------------
2019-11-17T15:16:32.3876493Z 
2019-11-17T15:16:32.3876921Z ------------------------------------------
2019-11-17T15:16:32.3877101Z stderr:
2019-11-17T15:16:32.3877101Z stderr:
2019-11-17T15:16:32.3877480Z ------------------------------------------
2019-11-17T15:16:32.3877660Z error: the crate `panic_abort` does not have the panic strategy `abort`
2019-11-17T15:16:32.3877961Z error: aborting due to previous error
2019-11-17T15:16:32.3878082Z 
2019-11-17T15:16:32.3878201Z 
2019-11-17T15:16:32.3878569Z ------------------------------------------
2019-11-17T15:16:32.3878569Z ------------------------------------------
2019-11-17T15:16:32.3878724Z 
2019-11-17T15:16:32.3878856Z 
2019-11-17T15:16:32.3879210Z ---- [ui] ui/panic-runtime/abort.rs stdout ----
2019-11-17T15:16:32.3879386Z 
2019-11-17T15:16:32.3879902Z error: test compilation failed although it shouldn't!
2019-11-17T15:16:32.3880068Z status: exit code: 1
2019-11-17T15:16:32.3880904Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/auxiliary"
2019-11-17T15:16:32.3881472Z ------------------------------------------
2019-11-17T15:16:32.3881617Z 
2019-11-17T15:16:32.3881957Z ------------------------------------------
2019-11-17T15:16:32.3882487Z stderr:
2019-11-17T15:16:32.3882487Z stderr:
2019-11-17T15:16:32.3882849Z ------------------------------------------
2019-11-17T15:16:32.3883045Z error: the crate `panic_abort` does not have the panic strategy `abort`
2019-11-17T15:16:32.3883858Z error: aborting due to previous error
2019-11-17T15:16:32.3883994Z 
2019-11-17T15:16:32.3884131Z 
2019-11-17T15:16:32.3884640Z ------------------------------------------
2019-11-17T15:16:32.3884640Z ------------------------------------------
2019-11-17T15:16:32.3884819Z 
2019-11-17T15:16:32.3884934Z 
2019-11-17T15:16:32.3885342Z ---- [ui] ui/panic-runtime/abort-link-to-unwinding-crates.rs stdout ----
2019-11-17T15:16:32.3885490Z 
2019-11-17T15:16:32.3885850Z error: test compilation failed although it shouldn't!
2019-11-17T15:16:32.3886035Z status: exit code: 1
2019-11-17T15:16:32.3888174Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/abort-link-to-unwinding-crates.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/auxiliary"
2019-11-17T15:16:32.3888856Z ------------------------------------------
2019-11-17T15:16:32.3889036Z 
2019-11-17T15:16:32.3889391Z ------------------------------------------
2019-11-17T15:16:32.3889561Z stderr:
2019-11-17T15:16:32.3889561Z stderr:
2019-11-17T15:16:32.3889923Z ------------------------------------------
2019-11-17T15:16:32.3890103Z error: the crate `panic_abort` does not have the panic strategy `abort`
2019-11-17T15:16:32.3890567Z error: aborting due to previous error
2019-11-17T15:16:32.3890686Z 
2019-11-17T15:16:32.3890800Z 
2019-11-17T15:16:32.3891151Z ------------------------------------------
2019-11-17T15:16:32.3891151Z ------------------------------------------
2019-11-17T15:16:32.3891317Z 
2019-11-17T15:16:32.3891437Z 
2019-11-17T15:16:32.3891791Z ---- [ui] ui/panic-runtime/link-to-abort.rs stdout ----
2019-11-17T15:16:32.3891963Z 
2019-11-17T15:16:32.3892365Z error: test compilation failed although it shouldn't!
2019-11-17T15:16:32.3893759Z status: exit code: 1
2019-11-17T15:16:32.3894607Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/link-to-abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/link-to-abort/auxiliary"
2019-11-17T15:16:32.3894941Z ------------------------------------------
2019-11-17T15:16:32.3894983Z 
2019-11-17T15:16:32.3895214Z ------------------------------------------
2019-11-17T15:16:32.3895261Z stderr:
2019-11-17T15:16:32.3895261Z stderr:
2019-11-17T15:16:32.3895470Z ------------------------------------------
2019-11-17T15:16:32.3895743Z error: the linked panic runtime `panic_abort` is not compiled with this crate's panic strategy `abort`
2019-11-17T15:16:32.3895856Z error: aborting due to previous error
2019-11-17T15:16:32.3895886Z 
2019-11-17T15:16:32.3895911Z 
2019-11-17T15:16:32.3896739Z ------------------------------------------
2019-11-17T15:16:32.3896739Z ------------------------------------------
2019-11-17T15:16:32.3896776Z 
2019-11-17T15:16:32.3896801Z 
2019-11-17T15:16:32.3897022Z ---- [ui] ui/panic-runtime/lto-abort.rs stdout ----
2019-11-17T15:16:32.3897074Z 
2019-11-17T15:16:32.3897296Z error: test compilation failed although it shouldn't!
2019-11-17T15:16:32.3897345Z status: exit code: 1
2019-11-17T15:16:32.3898233Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/lto-abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/lto-abort/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/lto-abort/auxiliary"
2019-11-17T15:16:32.3898664Z ------------------------------------------
2019-11-17T15:16:32.3898700Z 
2019-11-17T15:16:32.3898914Z ------------------------------------------
2019-11-17T15:16:32.3898959Z stderr:
2019-11-17T15:16:32.3898959Z stderr:
2019-11-17T15:16:32.3899185Z ------------------------------------------
2019-11-17T15:16:32.3899239Z error: the crate `panic_abort` does not have the panic strategy `abort`
2019-11-17T15:16:32.3899335Z error: aborting due to previous error
2019-11-17T15:16:32.3899366Z 
2019-11-17T15:16:32.3899392Z 
2019-11-17T15:16:32.3899613Z ------------------------------------------
---
2019-11-17T15:16:32.3900509Z 
2019-11-17T15:16:32.3900908Z 1 error: building tests with panic=abort is not supported without `-Zpanic_abort_tests`
2019-11-17T15:16:32.3900958Z 2 
2019-11-17T15:16:32.3901159Z - error: aborting due to previous error
2019-11-17T15:16:32.3901229Z + error: the crate `panic_abort` does not have the panic strategy `abort`
2019-11-17T15:16:32.3901317Z + error: aborting due to 2 previous errors
2019-11-17T15:16:32.3901356Z 4 
2019-11-17T15:16:32.3901410Z 5 
2019-11-17T15:16:32.3901435Z 
2019-11-17T15:16:32.3901435Z 
2019-11-17T15:16:32.3901460Z 
2019-11-17T15:16:32.3901503Z The actual stderr differed from the expected stderr.
2019-11-17T15:16:32.3901824Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort-disabled/test-panic-abort-disabled.stderr
2019-11-17T15:16:32.3902072Z To update references, rerun the tests and pass the `--bless` flag
2019-11-17T15:16:32.3902343Z To only update this specific test, also pass `--test-args test-panic-abort-disabled.rs`
2019-11-17T15:16:32.3902420Z error: 1 errors occurred comparing output.
2019-11-17T15:16:32.3902470Z status: exit code: 1
2019-11-17T15:16:32.3902470Z status: exit code: 1
2019-11-17T15:16:32.3903223Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-panic-abort-disabled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort-disabled" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-Cpanic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort-disabled/auxiliary" "-A" "unused"
2019-11-17T15:16:32.3903545Z ------------------------------------------
2019-11-17T15:16:32.3903577Z 
2019-11-17T15:16:32.3903777Z ------------------------------------------
2019-11-17T15:16:32.3903838Z stderr:
2019-11-17T15:16:32.3903838Z stderr:
2019-11-17T15:16:32.3904041Z ------------------------------------------
2019-11-17T15:16:32.3904295Z error: building tests with panic=abort is not supported without `-Zpanic_abort_tests`
2019-11-17T15:16:32.3904331Z 
2019-11-17T15:16:32.3904398Z error: the crate `panic_abort` does not have the panic strategy `abort`
2019-11-17T15:16:32.3904471Z error: aborting due to 2 previous errors
2019-11-17T15:16:32.3904499Z 
2019-11-17T15:16:32.3904540Z 
2019-11-17T15:16:32.3904745Z ------------------------------------------
2019-11-17T15:16:32.3904745Z ------------------------------------------
2019-11-17T15:16:32.3904776Z 
2019-11-17T15:16:32.3904800Z 
2019-11-17T15:16:32.3905018Z ---- [ui] ui/test-panic-abort.rs stdout ----
2019-11-17T15:16:32.3905049Z 
2019-11-17T15:16:32.3905261Z error: test compilation failed although it shouldn't!
2019-11-17T15:16:32.3905412Z status: exit code: 1
2019-11-17T15:16:32.3906677Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-panic-abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-Cpanic=abort" "-Zpanic_abort_tests" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-panic-abort/auxiliary" "-A" "unused"
2019-11-17T15:16:32.3907061Z ------------------------------------------
2019-11-17T15:16:32.3907095Z 
2019-11-17T15:16:32.3907306Z ------------------------------------------
2019-11-17T15:16:32.3907369Z stderr:
2019-11-17T15:16:32.3907369Z stderr:
2019-11-17T15:16:32.3907578Z ------------------------------------------
2019-11-17T15:16:32.3907630Z error: the crate `panic_abort` does not have the panic strategy `abort`
2019-11-17T15:16:32.3907737Z error: aborting due to previous error
2019-11-17T15:16:32.3907766Z 
2019-11-17T15:16:32.3907791Z 
2019-11-17T15:16:32.3908002Z ------------------------------------------
---
2019-11-17T15:16:32.3914343Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-17T15:16:32.3914419Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-17T15:16:32.3920538Z 
2019-11-17T15:16:32.3920643Z 
2019-11-17T15:16:32.3922436Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-17T15:16:32.3922688Z 
2019-11-17T15:16:32.3922735Z 
2019-11-17T15:16:32.3930923Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-17T15:16:32.3931473Z Build completed unsuccessfully in 1:06:13
2019-11-17T15:16:32.3931473Z Build completed unsuccessfully in 1:06:13
2019-11-17T15:16:32.3991956Z == clock drift check ==
2019-11-17T15:16:32.4007670Z   local time: Sun Nov 17 15:16:32 UTC 2019
2019-11-17T15:16:32.6783249Z   network time: Sun, 17 Nov 2019 15:16:32 GMT
2019-11-17T15:16:32.6784656Z == end clock drift check ==
2019-11-17T15:16:33.3715099Z 
2019-11-17T15:16:33.3852967Z ##[error]Bash exited with code '1'.
2019-11-17T15:16:33.3891879Z ##[section]Starting: Checkout
2019-11-17T15:16:33.3893642Z ==============================================================================
2019-11-17T15:16:33.3893698Z Task         : Get sources
2019-11-17T15:16:33.3893863Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
