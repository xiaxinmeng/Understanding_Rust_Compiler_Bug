plain
2019-08-17T14:08:21.1312072Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-17T14:08:21.1572497Z ##[command]git config gc.auto 0
2019-08-17T14:08:21.1651440Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-17T14:08:21.1711500Z ##[command]git config --get-all http.proxy
2019-08-17T14:08:21.1852073Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63580/merge:refs/remotes/pull/63580/merge
---
2019-08-17T14:08:54.3632490Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-17T14:08:54.3632562Z 
2019-08-17T14:08:54.3632788Z   git checkout -b <new-branch-name>
2019-08-17T14:08:54.3632852Z 
2019-08-17T14:08:54.3632902Z HEAD is now at 208f967f8 Merge 580b2e27d5d2c11082473bada1fc93be25cd0594 into ac60ca0643feb3452688a9ca97c839c155742915
2019-08-17T14:08:54.3826189Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-17T14:08:54.3829588Z ==============================================================================
2019-08-17T14:08:54.3829681Z Task         : Bash
2019-08-17T14:08:54.3829742Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-17T15:10:20.7703040Z .................................................................................................... 1500/8924
2019-08-17T15:10:26.4530044Z .................................................................................................... 1600/8924
2019-08-17T15:10:38.9975594Z ................................i...............i................................................... 1700/8924
2019-08-17T15:10:46.5004881Z .................................................................................................... 1800/8924
2019-08-17T15:11:00.5932585Z .......................iiiii........................................................................ 1900/8924
2019-08-17T15:11:11.0416543Z .................................................................................................... 2100/8924
2019-08-17T15:11:13.6234841Z .................................................................................................... 2200/8924
2019-08-17T15:11:18.4383405Z .................................................................................................... 2300/8924
2019-08-17T15:11:25.1278545Z .................................................................................................... 2400/8924
---
2019-08-17T15:14:16.1863418Z .................................................................................................... 4600/8924
2019-08-17T15:14:23.2351308Z .....i...............i.............................................................................. 4700/8924
2019-08-17T15:14:34.5696608Z .................................................................................................... 4800/8924
2019-08-17T15:14:40.3405984Z .................................................................................................... 4900/8924
2019-08-17T15:14:52.1799391Z ......................................................................................ii.ii......... 5000/8924
2019-08-17T15:15:01.5234508Z .................................................................................................... 5200/8924
2019-08-17T15:15:11.0413497Z .................................................................................................... 5300/8924
2019-08-17T15:15:17.8842115Z ..........................................i......................................................... 5400/8924
2019-08-17T15:15:24.3147991Z .................................................................................................... 5500/8924
2019-08-17T15:15:24.3147991Z .................................................................................................... 5500/8924
2019-08-17T15:15:35.1115535Z .................................................................................................... 5600/8924
2019-08-17T15:15:46.5769685Z ...................................ii...i..ii...........i........................................... 5700/8924
2019-08-17T15:16:01.9398637Z .................................................................................................... 5900/8924
2019-08-17T15:16:06.8014276Z .................................................................................................... 6000/8924
2019-08-17T15:16:06.8014276Z .................................................................................................... 6000/8924
2019-08-17T15:16:20.2552239Z ....................................i..ii........................................................... 6100/8924
2019-08-17T15:16:41.2860842Z ..............................................................................i..................... 6300/8924
2019-08-17T15:16:43.1027629Z .................................................................................................... 6400/8924
2019-08-17T15:16:45.3816891Z ..................................................i................................................. 6500/8924
2019-08-17T15:16:48.5850787Z .................................................................................................... 6600/8924
---
2019-08-17T15:20:43.0247855Z 11 error[E0080]: evaluation of constant expression failed
2019-08-17T15:20:43.0248375Z -   --> $DIR/issue-50814.rs:18:5
2019-08-17T15:20:43.0248923Z +   --> $DIR/issue-50814.rs:17:5
2019-08-17T15:20:43.0249221Z 13    |
2019-08-17T15:20:43.0249679Z 14 LL |     &Sum::<U8,U8>::MAX
2019-08-17T15:20:43.0250254Z 15    |     ^-----------------
2019-08-17T15:20:43.0250757Z 
2019-08-17T15:20:43.0250981Z The actual stderr differed from the expected stderr.
2019-08-17T15:20:43.0251556Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814/issue-50814.stderr
2019-08-17T15:20:43.0251556Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814/issue-50814.stderr
2019-08-17T15:20:43.0252141Z To update references, rerun the tests and pass the `--bless` flag
2019-08-17T15:20:43.0252718Z To only update this specific test, also pass `--test-args consts/const-eval/issue-50814.rs`
2019-08-17T15:20:43.0253242Z error: 1 errors occurred comparing output.
2019-08-17T15:20:43.0253465Z status: exit code: 1
2019-08-17T15:20:43.0253465Z status: exit code: 1
2019-08-17T15:20:43.0254507Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-50814.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-50814/auxiliary" "-A" "unused"
2019-08-17T15:20:43.0255865Z ------------------------------------------
2019-08-17T15:20:43.0256204Z 
2019-08-17T15:20:43.0256724Z ------------------------------------------
2019-08-17T15:20:43.0257042Z stderr:
2019-08-17T15:20:43.0257042Z stderr:
2019-08-17T15:20:43.0257515Z ------------------------------------------
2019-08-17T15:20:43.0258378Z error: any use of this value will cause an error
2019-08-17T15:20:43.0258965Z   --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:13:21
2019-08-17T15:20:43.0259175Z    |
2019-08-17T15:20:43.0259343Z LL |     const MAX: u8 = A::MAX + B::MAX; //~ ERROR any use of this value will cause an error
2019-08-17T15:20:43.0259757Z    |     ----------------^^^^^^^^^^^^^^^-
2019-08-17T15:20:43.0260121Z    |                     attempt to add with overflow
2019-08-17T15:20:43.0260270Z    |
2019-08-17T15:20:43.0260428Z    = note: `#[deny(const_err)]` on by default
2019-08-17T15:20:43.0260553Z 
2019-08-17T15:20:43.0260553Z 
2019-08-17T15:20:43.0260713Z error[E0080]: evaluation of constant expression failed
2019-08-17T15:20:43.0261114Z   --> /checkout/src/test/ui/consts/const-eval/issue-50814.rs:17:5
2019-08-17T15:20:43.0261325Z    |
2019-08-17T15:20:43.0261477Z LL |     &Sum::<U8,U8>::MAX //~ ERROR E0080
2019-08-17T15:20:43.0261827Z    |     ^-----------------
2019-08-17T15:20:43.0262184Z    |      referenced constant has errors
2019-08-17T15:20:43.0262307Z 
2019-08-17T15:20:43.0262455Z error: aborting due to 2 previous errors
2019-08-17T15:20:43.0262593Z 
---
2019-08-17T15:20:43.0286580Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-17T15:20:43.0286926Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-17T15:20:43.0301775Z 
2019-08-17T15:20:43.0303508Z 
2019-08-17T15:20:43.0312028Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-17T15:20:43.0312869Z 
2019-08-17T15:20:43.0313105Z 
2019-08-17T15:20:43.0318578Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-17T15:20:43.0319080Z Build completed unsuccessfully in 1:05:12
2019-08-17T15:20:43.0319080Z Build completed unsuccessfully in 1:05:12
2019-08-17T15:20:43.0376640Z == clock drift check ==
2019-08-17T15:20:43.0392006Z   local time: Sat Aug 17 15:20:43 UTC 2019
2019-08-17T15:20:43.3165663Z   network time: Sat, 17 Aug 2019 15:20:43 GMT
2019-08-17T15:20:43.3168915Z == end clock drift check ==
2019-08-17T15:20:44.2519630Z ##[error]Bash exited with code '1'.
2019-08-17T15:20:44.2585062Z ##[section]Starting: Checkout
2019-08-17T15:20:44.2587512Z ==============================================================================
2019-08-17T15:20:44.2587589Z Task         : Get sources
2019-08-17T15:20:44.2587670Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
