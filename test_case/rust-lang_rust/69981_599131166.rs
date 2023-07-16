plain
2020-03-14T19:13:34.3062030Z ========================== Starting Command Output ===========================
2020-03-14T19:13:34.3065812Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/216ce654-d821-4215-a390-d88f1abb24e9.sh
2020-03-14T19:13:34.3066377Z 
2020-03-14T19:13:34.3071613Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-14T19:13:34.3092577Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-14T19:13:34.3096174Z Task         : Get sources
2020-03-14T19:13:34.3096527Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T19:13:34.3096843Z Version      : 1.0.0
2020-03-14T19:13:34.3097061Z Author       : Microsoft
---
2020-03-14T19:13:35.2994397Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-14T19:13:35.3000035Z ##[command]git config gc.auto 0
2020-03-14T19:13:35.3003988Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-14T19:13:35.3007640Z ##[command]git config --get-all http.proxy
2020-03-14T19:13:35.3014217Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69981/merge:refs/remotes/pull/69981/merge
---
2020-03-14T20:16:22.1709524Z .................................................................................................... 1700/9771
2020-03-14T20:16:26.5549808Z .................................................................................................... 1800/9771
2020-03-14T20:16:38.3208068Z ...................................................................i................................ 1900/9771
2020-03-14T20:16:45.3922599Z .................................................................................................... 2000/9771
2020-03-14T20:17:00.0946750Z .........................................................iiiii...................................... 2100/9771
2020-03-14T20:17:11.2396096Z .................................................................................................... 2300/9771
2020-03-14T20:17:13.4671166Z .................................................................................................... 2400/9771
2020-03-14T20:17:16.6153397Z .................................................................................................... 2500/9771
2020-03-14T20:17:38.8006670Z .................................................................................................... 2600/9771
---
2020-03-14T20:20:17.9068092Z .............................i...............i...................................................... 5000/9771
2020-03-14T20:20:27.8227177Z .................................................................................................... 5100/9771
2020-03-14T20:20:34.1933586Z ........................................................................i........................... 5200/9771
2020-03-14T20:20:40.1691280Z .................................................................................................... 5300/9771
2020-03-14T20:20:50.6076242Z .....................................................ii.ii........i...i............................. 5400/9771
2020-03-14T20:20:59.5908448Z .................................................................................................... 5600/9771
2020-03-14T20:21:09.8135173Z .................................................................................................... 5700/9771
2020-03-14T20:21:16.4605429Z .............................................i...................................................... 5800/9771
2020-03-14T20:21:23.2958209Z .................................................................................................... 5900/9771
2020-03-14T20:21:23.2958209Z .................................................................................................... 5900/9771
2020-03-14T20:21:33.9650134Z .................................................................................................... 6000/9771
2020-03-14T20:21:40.0126089Z .......................................ii...i..ii...........i....................................... 6100/9771
2020-03-14T20:22:00.6263257Z .................................................................................................... 6300/9771
2020-03-14T20:22:07.8115872Z .................................................................................................... 6400/9771
2020-03-14T20:22:07.8115872Z .................................................................................................... 6400/9771
2020-03-14T20:22:17.4962061Z .....................................................................i..ii.......................... 6500/9771
2020-03-14T20:22:40.4805655Z .................................................................................................... 6700/9771
2020-03-14T20:22:48.9045042Z ...................................................................i................................ 6800/9771
2020-03-14T20:22:50.8953284Z .................................................................................................... 6900/9771
2020-03-14T20:22:53.1004761Z .................................................................................................... 7000/9771
---
2020-03-14T20:24:39.5514104Z .................................................................................................... 7800/9771
2020-03-14T20:24:45.7588655Z .................................................................................................... 7900/9771
2020-03-14T20:24:51.8648885Z ...................................................i................................................ 8000/9771
2020-03-14T20:25:02.9087929Z .................................................................................................... 8100/9771
2020-03-14T20:25:08.6676235Z iiiiiiiiii.i........................................................................................ 8200/9771
2020-03-14T20:25:23.2420793Z .................................................................................................... 8400/9771
2020-03-14T20:25:34.3653251Z .................................................................................................... 8500/9771
2020-03-14T20:25:47.7366357Z .................................................................................................... 8600/9771
2020-03-14T20:25:53.7834052Z .................................................................................................... 8700/9771
---
2020-03-14T20:27:49.8302410Z -   --> $DIR/issue-62504.rs:18:25
2020-03-14T20:27:49.8302678Z + error[E0308]: mismatched types
2020-03-14T20:27:49.8303107Z +   --> $DIR/issue-62504.rs:18:21
2020-03-14T20:27:49.8303307Z 3    |
2020-03-14T20:27:49.8303555Z 4 LL |         ArrayHolder([0; Self::SIZE])
2020-03-14T20:27:49.8304374Z +    |                     ^^^^^^^^^^^^^^^ expected `X`, found `Self::SIZE`
2020-03-14T20:27:49.8306559Z +    |
2020-03-14T20:27:49.8306559Z +    |
2020-03-14T20:27:49.8306769Z +    = note: expected array `[u32; _]`
2020-03-14T20:27:49.8307028Z +               found array `[u32; _]`
2020-03-14T20:27:49.8307432Z 7 error: aborting due to previous error
2020-03-14T20:27:49.8307624Z 8 
2020-03-14T20:27:49.8307730Z 
2020-03-14T20:27:49.8308257Z + For more information about this error, try `rustc --explain E0308`.
2020-03-14T20:27:49.8308257Z + For more information about this error, try `rustc --explain E0308`.
2020-03-14T20:27:49.8308505Z 9 
2020-03-14T20:27:49.8308607Z 
2020-03-14T20:27:49.8308888Z 
2020-03-14T20:27:49.8309134Z The actual stderr differed from the expected stderr.
2020-03-14T20:27:49.8309881Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62504/issue-62504.stderr
2020-03-14T20:27:49.8310689Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T20:27:49.8311348Z To only update this specific test, also pass `--test-args const-generics/issues/issue-62504.rs`
2020-03-14T20:27:49.8311814Z error: 1 errors occurred comparing output.
2020-03-14T20:27:49.8312077Z status: exit code: 1
2020-03-14T20:27:49.8312077Z status: exit code: 1
2020-03-14T20:27:49.8314130Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-62504.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62504" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62504/auxiliary"
2020-03-14T20:27:49.8315825Z ------------------------------------------
2020-03-14T20:27:49.8316052Z 
2020-03-14T20:27:49.8316450Z ------------------------------------------
2020-03-14T20:27:49.8316656Z stderr:
2020-03-14T20:27:49.8316656Z stderr:
2020-03-14T20:27:49.8317022Z ------------------------------------------
2020-03-14T20:27:49.8317294Z error[E0308]: mismatched types
2020-03-14T20:27:49.8317806Z   --> /checkout/src/test/ui/const-generics/issues/issue-62504.rs:18:21
2020-03-14T20:27:49.8318070Z    |
2020-03-14T20:27:49.8318310Z LL |         ArrayHolder([0; Self::SIZE])
2020-03-14T20:27:49.8318680Z    |                     ^^^^^^^^^^^^^^^ expected `X`, found `Self::SIZE`
2020-03-14T20:27:49.8318965Z    |
2020-03-14T20:27:49.8319187Z    = note: expected array `[u32; _]`
2020-03-14T20:27:49.8319441Z               found array `[u32; _]`
2020-03-14T20:27:49.8319816Z error: aborting due to previous error
2020-03-14T20:27:49.8319984Z 
2020-03-14T20:27:49.8320427Z For more information about this error, try `rustc --explain E0308`.
2020-03-14T20:27:49.8320648Z 
2020-03-14T20:27:49.8320648Z 
2020-03-14T20:27:49.8321024Z ------------------------------------------
2020-03-14T20:27:49.8321199Z 
2020-03-14T20:27:49.8321298Z 
2020-03-14T20:27:49.8321710Z ---- [ui] ui/const-generics/issues/issue-67739.rs stdout ----
2020-03-14T20:27:49.8321920Z 
2020-03-14T20:27:49.8322204Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-14T20:27:49.8322513Z status: exit code: 101
2020-03-14T20:27:49.8324859Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67739.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739/auxiliary"
2020-03-14T20:27:49.8329694Z ------------------------------------------
2020-03-14T20:27:49.8329876Z 
2020-03-14T20:27:49.8330249Z ------------------------------------------
2020-03-14T20:27:49.8330454Z stderr:
2020-03-14T20:27:49.8330454Z stderr:
2020-03-14T20:27:49.8330847Z ------------------------------------------
2020-03-14T20:27:49.8331715Z error: internal compiler error: src/librustc_traits/normalize_erasing_regions.rs:35: could not fully normalize `fn() -> usize {std::mem::size_of::<<Self as Trait>::Associated>}`
2020-03-14T20:27:49.8332841Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-03-14T20:27:49.8333250Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T20:27:49.8343091Z 
2020-03-14T20:27:49.8343464Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-14T20:27:49.8343464Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-14T20:27:49.8343671Z 
2020-03-14T20:27:49.8344829Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-14T20:27:49.8345138Z 
2020-03-14T20:27:49.8345723Z note: rustc 1.43.0-nightly (dab99769e 2020-03-14) running on x86_64-unknown-linux-gnu
2020-03-14T20:27:49.8345980Z 
2020-03-14T20:27:49.8346623Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-14T20:27:49.8347192Z error: aborting due to previous error
2020-03-14T20:27:49.8347362Z 
2020-03-14T20:27:49.8347460Z 
2020-03-14T20:27:49.8347847Z ------------------------------------------
---
2020-03-14T20:27:49.8349989Z test result: FAILED. 9712 passed; 2 failed; 57 ignored; 0 measured; 0 filtered out
2020-03-14T20:27:49.8350262Z 
2020-03-14T20:27:49.8350450Z 
2020-03-14T20:27:49.8350548Z 
2020-03-14T20:27:49.8354296Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-14T20:27:49.8357013Z 
2020-03-14T20:27:49.8357137Z 
2020-03-14T20:27:49.8357656Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-14T20:27:49.8358082Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T20:27:49.8358082Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T20:27:49.8358489Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-14T20:27:49.8358819Z Build completed unsuccessfully in 1:07:15
2020-03-14T20:27:49.8393545Z == clock drift check ==
2020-03-14T20:27:49.8410162Z   local time: Sat Mar 14 20:27:49 UTC 2020
2020-03-14T20:27:50.3943661Z   network time: Sat, 14 Mar 2020 20:27:50 GMT
2020-03-14T20:27:50.3945344Z == end clock drift check ==
2020-03-14T20:27:50.9738449Z 
2020-03-14T20:27:50.9819163Z ##[error]Bash exited with code '1'.
2020-03-14T20:27:50.9835654Z ##[section]Finishing: Run build
2020-03-14T20:27:50.9894208Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-14T20:27:50.9900191Z Task         : Get sources
2020-03-14T20:27:50.9900584Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T20:27:50.9900963Z Version      : 1.0.0
2020-03-14T20:27:50.9901220Z Author       : Microsoft
2020-03-14T20:27:50.9901220Z Author       : Microsoft
2020-03-14T20:27:50.9901835Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-14T20:27:50.9902319Z ==============================================================================
2020-03-14T20:27:51.3588377Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-14T20:27:51.3664222Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-14T20:27:51.3759474Z Cleaning up task key
2020-03-14T20:27:51.3760868Z Start cleaning up orphan processes.
2020-03-14T20:27:51.3957896Z Terminate orphan process: pid (3722) (python)
2020-03-14T20:27:51.4128007Z ##[section]Finishing: Finalize Job
