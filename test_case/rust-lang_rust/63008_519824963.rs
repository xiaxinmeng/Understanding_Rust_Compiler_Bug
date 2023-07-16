plain
2019-08-09T06:55:48.8790476Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-09T06:55:48.8971896Z ##[command]git config gc.auto 0
2019-08-09T06:55:48.9061969Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-09T06:55:48.9120498Z ##[command]git config --get-all http.proxy
2019-08-09T06:55:48.9279920Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63008/merge:refs/remotes/pull/63008/merge
---
2019-08-09T06:56:25.4243647Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T06:56:25.4243821Z 
2019-08-09T06:56:25.4244242Z   git checkout -b <new-branch-name>
2019-08-09T06:56:25.4244403Z 
2019-08-09T06:56:25.4244571Z HEAD is now at de4d95214 Merge 8307000cabb34c98ef5bbfcf93ec4b72099419c0 into 5aa3d9a7b5d3a46a7f158e8881146331a6bc9243
2019-08-09T06:56:25.4393822Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T06:56:25.4396518Z ==============================================================================
2019-08-09T06:56:25.4396576Z Task         : Bash
2019-08-09T06:56:25.4396621Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T07:58:56.9816040Z .................................................................................................... 1300/8853
2019-08-09T07:59:03.5246867Z ..........................................F......................................................... 1400/8853
2019-08-09T07:59:09.8433831Z .................................................................................................... 1500/8853
2019-08-09T07:59:20.6118015Z ....................................................................................i............... 1600/8853
2019-08-09T07:59:28.3105186Z i................................................................................................... 1700/8853
2019-08-09T07:59:35.4046536Z ......................................................................iiiii......................... 1800/8853
2019-08-09T07:59:57.4429661Z .................................................................................................... 2000/8853
2019-08-09T07:59:59.9570429Z .................................................................................................... 2100/8853
2019-08-09T08:00:02.9082913Z .................................................................................................... 2200/8853
2019-08-09T08:00:10.8335906Z .................................................................................................... 2300/8853
---
2019-08-09T08:04:00.2558651Z .................................................................................................... 5200/8853
2019-08-09T08:04:11.9145866Z ..............................................................................................i..... 5300/8853
2019-08-09T08:04:19.9196517Z .................................................................................................... 5400/8853
2019-08-09T08:04:24.7997237Z .................................................................................................... 5500/8853
2019-08-09T08:04:36.4317899Z ........................................................................................ii...i..ii.. 5600/8853
2019-08-09T08:05:00.9596698Z .................................................................................................... 5800/8853
2019-08-09T08:05:06.4558651Z .................................................................................................... 5900/8853
2019-08-09T08:05:06.4558651Z .................................................................................................... 5900/8853
2019-08-09T08:05:11.1698122Z .........................................................................................i..ii...... 6000/8853
2019-08-09T08:05:42.4151733Z .................................................................................................... 6200/8853
2019-08-09T08:05:44.5400301Z ................................i................................................................... 6300/8853
2019-08-09T08:05:46.7825191Z .................................................................................................... 6400/8853
2019-08-09T08:05:49.4950995Z ....i............................................................................................... 6500/8853
---
2019-08-09T08:09:56.7030070Z 
2019-08-09T08:09:56.7030252Z 1 error[E0658]: casting pointers to integers in constants is unstable
2019-08-09T08:09:56.7030681Z 2   --> $DIR/match-test-ptr-null.rs:6:15
2019-08-09T08:09:56.7030872Z 3    |
2019-08-09T08:09:56.7031735Z - LL |         match &1 as *const i32 as usize { 
2019-08-09T08:09:56.7032397Z + LL |         match &1 as *const i32 as usize {
2019-08-09T08:09:56.7032749Z 6    |
2019-08-09T08:09:56.7032749Z 6    |
2019-08-09T08:09:56.7033755Z 7    = note: see issue #51910 <***/issues/51910> for more information
2019-08-09T08:09:56.7034184Z 10 error[E0019]: constant contains unimplemented expression type
2019-08-09T08:09:56.7034616Z 11   --> $DIR/match-test-ptr-null.rs:6:15
2019-08-09T08:09:56.7034828Z 12    |
2019-08-09T08:09:56.7034828Z 12    |
2019-08-09T08:09:56.7035200Z - LL |         match &1 as *const i32 as usize { 
2019-08-09T08:09:56.7035389Z + LL |         match &1 as *const i32 as usize {
2019-08-09T08:09:56.7035719Z 15 
2019-08-09T08:09:56.7035882Z 16 error[E0019]: constant contains unimplemented expression type
2019-08-09T08:09:56.7036011Z 
2019-08-09T08:09:56.7036156Z 22 error[E0080]: evaluation of constant value failed
2019-08-09T08:09:56.7036156Z 22 error[E0080]: evaluation of constant value failed
2019-08-09T08:09:56.7036526Z 23   --> $DIR/match-test-ptr-null.rs:6:15
2019-08-09T08:09:56.7036724Z 24    |
2019-08-09T08:09:56.7037110Z - LL |         match &1 as *const i32 as usize { 
2019-08-09T08:09:56.7037311Z + LL |         match &1 as *const i32 as usize {
2019-08-09T08:09:56.7037747Z 26    |               ^^^^^^^^^^^^^^^^^^^^^^^^^ "pointer-to-integer cast" needs an rfc before being allowed inside constants
2019-08-09T08:09:56.7038104Z 28 error: aborting due to 4 previous errors
2019-08-09T08:09:56.7038228Z 
2019-08-09T08:09:56.7038346Z 
2019-08-09T08:09:56.7038502Z The actual stderr differed from the expected stderr.
2019-08-09T08:09:56.7038502Z The actual stderr differed from the expected stderr.
2019-08-09T08:09:56.7039147Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/match-test-ptr-null.stderr
2019-08-09T08:09:56.7039649Z To update references, rerun the tests and pass the `--bless` flag
2019-08-09T08:09:56.7040126Z To only update this specific test, also pass `--test-args consts/const-eval/match-test-ptr-null.rs`
2019-08-09T08:09:56.7040432Z error: 1 errors occurred comparing output.
2019-08-09T08:09:56.7040615Z status: exit code: 1
2019-08-09T08:09:56.7040615Z status: exit code: 1
2019-08-09T08:09:56.7041537Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/auxiliary" "-A" "unused"
2019-08-09T08:09:56.7042156Z ------------------------------------------
2019-08-09T08:09:56.7042336Z 
2019-08-09T08:09:56.7042695Z ------------------------------------------
2019-08-09T08:09:56.7042873Z stderr:
2019-08-09T08:09:56.7042873Z stderr:
2019-08-09T08:09:56.7043234Z ------------------------------------------
2019-08-09T08:09:56.7043852Z error[E0658]: casting pointers to integers in constants is unstable
2019-08-09T08:09:56.7044319Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6:15
2019-08-09T08:09:56.7044538Z    |
2019-08-09T08:09:56.7044683Z LL |         match &1 as *const i32 as usize {
2019-08-09T08:09:56.7045013Z    |
2019-08-09T08:09:56.7045013Z    |
2019-08-09T08:09:56.7045476Z    = note: see issue #51910 <***/issues/51910> for more information
2019-08-09T08:09:56.7045716Z    = help: add `#![feature(const_raw_ptr_to_usize_cast)]` to the crate attributes to enable
2019-08-09T08:09:56.7046003Z error[E0019]: constant contains unimplemented expression type
2019-08-09T08:09:56.7046430Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6:15
2019-08-09T08:09:56.7046611Z    |
2019-08-09T08:09:56.7046611Z    |
2019-08-09T08:09:56.7046752Z LL |         match &1 as *const i32 as usize {
2019-08-09T08:09:56.7047056Z 
2019-08-09T08:09:56.7047199Z error[E0019]: constant contains unimplemented expression type
2019-08-09T08:09:56.7047612Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:12:13
2019-08-09T08:09:56.7047791Z    |
2019-08-09T08:09:56.7047791Z    |
2019-08-09T08:09:56.7047958Z LL |             0 => 42, //~ ERROR constant contains unimplemented expression type
2019-08-09T08:09:56.7048229Z 
2019-08-09T08:09:56.7048370Z error[E0080]: evaluation of constant value failed
2019-08-09T08:09:56.7048791Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6:15
2019-08-09T08:09:56.7048975Z    |
2019-08-09T08:09:56.7048975Z    |
2019-08-09T08:09:56.7049135Z LL |         match &1 as *const i32 as usize {
2019-08-09T08:09:56.7049575Z    |               ^^^^^^^^^^^^^^^^^^^^^^^^^ "pointer-to-integer cast" needs an rfc before being allowed inside constants
2019-08-09T08:09:56.7049908Z error: aborting due to 4 previous errors
2019-08-09T08:09:56.7050050Z 
2019-08-09T08:09:56.7050195Z Some errors have detailed explanations: E0019, E0080, E0658.
2019-08-09T08:09:56.7050602Z For more information about an error, try `rustc --explain E0019`.
---
2019-08-09T08:09:56.7069099Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-09T08:09:56.7069399Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-09T08:09:56.7088042Z 
2019-08-09T08:09:56.7088360Z 
2019-08-09T08:09:56.7090249Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-09T08:09:56.7091427Z 
2019-08-09T08:09:56.7091460Z 
2019-08-09T08:09:56.7095722Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-09T08:09:56.7095800Z Build completed unsuccessfully in 1:07:10
2019-08-09T08:09:56.7095800Z Build completed unsuccessfully in 1:07:10
2019-08-09T08:09:57.4866968Z ##[error]Bash exited with code '1'.
2019-08-09T08:09:57.4917274Z ##[section]Starting: Checkout
2019-08-09T08:09:57.4919080Z ==============================================================================
2019-08-09T08:09:57.4919136Z Task         : Get sources
2019-08-09T08:09:57.4919184Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
