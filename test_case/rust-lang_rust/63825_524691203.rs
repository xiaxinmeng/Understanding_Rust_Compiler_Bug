plain
2019-08-26T01:11:42.2304245Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T01:11:42.2485061Z ##[command]git config gc.auto 0
2019-08-26T01:11:42.2532536Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T01:11:42.2582171Z ##[command]git config --get-all http.proxy
2019-08-26T01:11:42.2707133Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63825/merge:refs/remotes/pull/63825/merge
---
2019-08-26T01:12:16.2442000Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T01:12:16.2442023Z 
2019-08-26T01:12:16.2442184Z   git checkout -b <new-branch-name>
2019-08-26T01:12:16.2442207Z 
2019-08-26T01:12:16.2442261Z HEAD is now at 258f84984 Merge 29cc305e936d9637f12f37e0fb6e38e04009d610 into 521d78407471cb78e9bbf47160f6aa23047ac499
2019-08-26T01:12:16.2581600Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T01:12:16.2584017Z ==============================================================================
2019-08-26T01:12:16.2584060Z Task         : Bash
2019-08-26T01:12:16.2584110Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T02:09:48.6788006Z .................................................................................................... 1500/8953
2019-08-26T02:09:53.9076357Z .................................................................................................... 1600/8953
2019-08-26T02:10:05.5501478Z .............................................i...............i...................................... 1700/8953
2019-08-26T02:10:13.0023819Z .................................................................................................... 1800/8953
2019-08-26T02:10:25.9442778Z .....................................iiiii.......................................................... 1900/8953
2019-08-26T02:10:35.4629534Z .................................................................................................... 2100/8953
2019-08-26T02:10:37.7014429Z .................................................................................................... 2200/8953
2019-08-26T02:10:41.5578349Z .................................................................................................... 2300/8953
2019-08-26T02:10:48.1215673Z .................................................................................................... 2400/8953
---
2019-08-26T02:13:29.9595274Z .........................i...............i.......................................................... 4700/8953
2019-08-26T02:13:40.6811563Z .................................................................................................... 4800/8953
2019-08-26T02:13:46.6843206Z .................................................................................................... 4900/8953
2019-08-26T02:13:56.3073241Z .................................................................................................... 5000/8953
2019-08-26T02:14:01.3981268Z ......ii.ii......................................................................................... 5100/8953
2019-08-26T02:14:14.4263835Z .................................................................................................... 5300/8953
2019-08-26T02:14:20.8232614Z ..............................................................i..................................... 5400/8953
2019-08-26T02:14:27.2372657Z .................................................................................................... 5500/8953
2019-08-26T02:14:34.3069123Z .................................................................................................... 5600/8953
2019-08-26T02:14:34.3069123Z .................................................................................................... 5600/8953
2019-08-26T02:14:44.2388744Z ........................................................ii...i..ii...........i...................... 5700/8953
2019-08-26T02:15:05.3834533Z .................................................................................................... 5900/8953
2019-08-26T02:15:09.8345842Z .................................................................................................... 6000/8953
2019-08-26T02:15:09.8345842Z .................................................................................................... 6000/8953
2019-08-26T02:15:16.2958225Z .........................................................i..ii...................................... 6100/8953
2019-08-26T02:15:41.1398436Z .................................................................................................... 6300/8953
2019-08-26T02:15:43.0168886Z ...i................................................................................................ 6400/8953
2019-08-26T02:15:44.9037814Z ...........................................................................i........................ 6500/8953
2019-08-26T02:15:47.2749594Z .................................................................................................... 6600/8953
---
2019-08-26T02:19:20.9718377Z 
2019-08-26T02:19:20.9719844Z ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
2019-08-26T02:19:20.9720100Z diff of run.stderr:
2019-08-26T02:19:20.9720255Z 
2019-08-26T02:19:20.9720724Z - [$DIR/dbg-macro-expected-behavior.rs:20] Unit = Unit
2019-08-26T02:19:20.9721232Z - [$DIR/dbg-macro-expected-behavior.rs:21] a = Unit
2019-08-26T02:19:20.9721691Z - [$DIR/dbg-macro-expected-behavior.rs:27] Point{x: 42, y: 24,} = Point {
2019-08-26T02:19:20.9722464Z + [$DIR/dbg-macro-expected-behavior.rs:23] Unit = Unit
2019-08-26T02:19:20.9723086Z + [$DIR/dbg-macro-expected-behavior.rs:24] a = Unit
2019-08-26T02:19:20.9723678Z + [$DIR/dbg-macro-expected-behavior.rs:30] Point{x: 42, y: 24,} = Point {
2019-08-26T02:19:20.9723868Z 4     x: 42,
2019-08-26T02:19:20.9724181Z 5     y: 24,
2019-08-26T02:19:20.9724625Z 
2019-08-26T02:19:20.9724625Z 
2019-08-26T02:19:20.9725153Z - [$DIR/dbg-macro-expected-behavior.rs:28] b = Point {
2019-08-26T02:19:20.9725708Z + [$DIR/dbg-macro-expected-behavior.rs:31] b = Point {
2019-08-26T02:19:20.9761533Z 8     x: 42,
2019-08-26T02:19:20.9813326Z 9     y: 24,
2019-08-26T02:19:20.9813823Z 
2019-08-26T02:19:20.9813823Z 
2019-08-26T02:19:20.9814230Z - [$DIR/dbg-macro-expected-behavior.rs:36]
2019-08-26T02:19:20.9814450Z - [$DIR/dbg-macro-expected-behavior.rs:40] &a = NoCopy(
2019-08-26T02:19:20.9814828Z + [$DIR/dbg-macro-expected-behavior.rs:39]
2019-08-26T02:19:20.9815166Z + [$DIR/dbg-macro-expected-behavior.rs:43] &a = NoCopy(
2019-08-26T02:19:20.9815474Z 14 )
2019-08-26T02:19:20.9815474Z 14 )
2019-08-26T02:19:20.9815717Z - [$DIR/dbg-macro-expected-behavior.rs:40] dbg!(& a) = NoCopy(
2019-08-26T02:19:20.9816092Z + [$DIR/dbg-macro-expected-behavior.rs:43] dbg!(& a) = NoCopy(
2019-08-26T02:19:20.9816397Z 17 )
2019-08-26T02:19:20.9816397Z 17 )
2019-08-26T02:19:20.9816657Z - [$DIR/dbg-macro-expected-behavior.rs:45] f(&42) = 42
2019-08-26T02:19:20.9816999Z + [$DIR/dbg-macro-expected-behavior.rs:48] f(&42) = 42
2019-08-26T02:19:20.9817151Z 19 before
2019-08-26T02:19:20.9817508Z - [$DIR/dbg-macro-expected-behavior.rs:50] { foo += 1; eprintln!("before"); 7331 } = 7331
2019-08-26T02:19:20.9817704Z - [$DIR/dbg-macro-expected-behavior.rs:58] ("Yeah",) = (
2019-08-26T02:19:20.9818089Z + [$DIR/dbg-macro-expected-behavior.rs:53] { foo += 1; eprintln!("before"); 7331 } = 7331
2019-08-26T02:19:20.9818433Z + [$DIR/dbg-macro-expected-behavior.rs:61] ("Yeah",) = (
2019-08-26T02:19:20.9818584Z 22     "Yeah",
2019-08-26T02:19:20.9818723Z 23 )
2019-08-26T02:19:20.9819378Z - [$DIR/dbg-macro-expected-behavior.rs:61] 1 = 1
2019-08-26T02:19:20.9819840Z - [$DIR/dbg-macro-expected-behavior.rs:61] 2 = 2
2019-08-26T02:19:20.9821410Z - [$DIR/dbg-macro-expected-behavior.rs:65] 1u8 = 1
2019-08-26T02:19:20.9821878Z - [$DIR/dbg-macro-expected-behavior.rs:65] 2u32 = 2
2019-08-26T02:19:20.9824258Z - [$DIR/dbg-macro-expected-behavior.rs:65] "Yeah" = "Yeah"
2019-08-26T02:19:20.9825482Z + [$DIR/dbg-macro-expected-behavior.rs:64] 1 = 1
2019-08-26T02:19:20.9825867Z + [$DIR/dbg-macro-expected-behavior.rs:64] 2 = 2
2019-08-26T02:19:20.9826234Z + [$DIR/dbg-macro-expected-behavior.rs:68] 1u8 = 1
2019-08-26T02:19:20.9826594Z + [$DIR/dbg-macro-expected-behavior.rs:68] 2u32 = 2
2019-08-26T02:19:20.9826953Z + [$DIR/dbg-macro-expected-behavior.rs:68] "Yeah" = "Yeah"
2019-08-26T02:19:20.9827390Z 
2019-08-26T02:19:20.9827414Z 
2019-08-26T02:19:20.9827479Z The actual run.stderr differed from the expected run.stderr.
2019-08-26T02:19:20.9827479Z The actual run.stderr differed from the expected run.stderr.
2019-08-26T02:19:20.9828019Z Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/dbg-macro-expected-behavior.run.stderr
2019-08-26T02:19:20.9875378Z error: 1 errors occured comparing run output.
2019-08-26T02:19:20.9875516Z status: exit code: 0
2019-08-26T02:19:20.9875994Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a"
2019-08-26T02:19:20.9876046Z stdout:
2019-08-26T02:19:20.9876046Z stdout:
2019-08-26T02:19:20.9876262Z ------------------------------------------
2019-08-26T02:19:20.9876909Z 
2019-08-26T02:19:20.9877176Z ------------------------------------------
2019-08-26T02:19:20.9877237Z stderr:
2019-08-26T02:19:20.9877422Z ------------------------------------------
2019-08-26T02:19:20.9877646Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:23] Unit = Unit
2019-08-26T02:19:20.9877882Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:24] a = Unit
2019-08-26T02:19:20.9878298Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:30] Point{x: 42, y: 24,} = Point {
2019-08-26T02:19:20.9878346Z     x: 42,
2019-08-26T02:19:20.9878392Z     y: 24,
2019-08-26T02:19:20.9878427Z }
2019-08-26T02:19:20.9878686Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:31] b = Point {
2019-08-26T02:19:20.9878742Z     x: 42,
2019-08-26T02:19:20.9878777Z     y: 24,
2019-08-26T02:19:20.9880478Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:39]
2019-08-26T02:19:20.9880478Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:39]
2019-08-26T02:19:20.9881330Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:43] &a = NoCopy(
2019-08-26T02:19:20.9881434Z )
2019-08-26T02:19:20.9881434Z )
2019-08-26T02:19:20.9881744Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:43] dbg!(& a) = NoCopy(
2019-08-26T02:19:20.9882206Z )
2019-08-26T02:19:20.9882206Z )
2019-08-26T02:19:20.9882743Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:48] f(&42) = 42
2019-08-26T02:19:20.9882787Z before
2019-08-26T02:19:20.9883468Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:53] { foo += 1; eprintln!("before"); 7331 } = 7331
2019-08-26T02:19:20.9884034Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:61] ("Yeah",) = (
2019-08-26T02:19:20.9884090Z     "Yeah",
2019-08-26T02:19:20.9884123Z )
2019-08-26T02:19:20.9884373Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:64] 1 = 1
2019-08-26T02:19:20.9884605Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:64] 2 = 2
2019-08-26T02:19:20.9884833Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:68] 1u8 = 1
2019-08-26T02:19:20.9885073Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:68] 2u32 = 2
2019-08-26T02:19:20.9885306Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:68] "Yeah" = "Yeah"
2019-08-26T02:19:20.9885532Z ------------------------------------------
2019-08-26T02:19:20.9885570Z 
2019-08-26T02:19:20.9885592Z 
2019-08-26T02:19:20.9885613Z 
---
2019-08-26T02:19:20.9886715Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-26T02:19:20.9886763Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-26T02:19:20.9886789Z 
2019-08-26T02:19:20.9886809Z 
2019-08-26T02:19:20.9888002Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-26T02:19:20.9888281Z 
2019-08-26T02:19:20.9888306Z 
2019-08-26T02:19:20.9888359Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-26T02:19:20.9888406Z Build completed unsuccessfully in 1:00:54
2019-08-26T02:19:20.9888406Z Build completed unsuccessfully in 1:00:54
2019-08-26T02:19:20.9888490Z == clock drift check ==
2019-08-26T02:19:20.9898371Z   local time: Mon Aug 26 02:19:20 UTC 2019
2019-08-26T02:19:21.1387764Z   network time: Mon, 26 Aug 2019 02:19:21 GMT
2019-08-26T02:19:21.1387926Z == end clock drift check ==
2019-08-26T02:19:22.0396707Z ##[error]Bash exited with code '1'.
2019-08-26T02:19:22.0436057Z ##[section]Starting: Checkout
2019-08-26T02:19:22.0437718Z ==============================================================================
2019-08-26T02:19:22.0437777Z Task         : Get sources
2019-08-26T02:19:22.0437833Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
