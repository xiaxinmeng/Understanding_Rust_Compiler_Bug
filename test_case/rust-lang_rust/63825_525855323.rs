plain
2019-08-28T16:44:57.6204075Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-28T16:44:58.3706816Z ##[command]git config gc.auto 0
2019-08-28T16:44:58.3710267Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-28T16:44:58.3714874Z ##[command]git config --get-all http.proxy
2019-08-28T16:44:58.3717823Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63825/merge:refs/remotes/pull/63825/merge
---
2019-08-28T17:46:55.7341970Z .................................................................................................... 1500/8969
2019-08-28T17:47:01.3252427Z .................................................................................................... 1600/8969
2019-08-28T17:47:13.9182498Z .............................................i...............i...................................... 1700/8969
2019-08-28T17:47:21.9798489Z .................................................................................................... 1800/8969
2019-08-28T17:47:36.0480651Z ....................................iiiii........................................................... 1900/8969
2019-08-28T17:47:46.4479577Z .................................................................................................... 2100/8969
2019-08-28T17:47:48.8854225Z .................................................................................................... 2200/8969
2019-08-28T17:47:53.0595357Z .................................................................................................... 2300/8969
2019-08-28T17:48:00.1616968Z .................................................................................................... 2400/8969
---
2019-08-28T17:50:57.0689012Z .......................i...............i............................................................ 4700/8969
2019-08-28T17:51:08.7870379Z .................................................................................................... 4800/8969
2019-08-28T17:51:14.8275757Z .................................................................................................... 4900/8969
2019-08-28T17:51:25.9867414Z .................................................................................................... 5000/8969
2019-08-28T17:51:31.2614194Z ....ii.ii........................................................................................... 5100/8969
2019-08-28T17:51:45.2542080Z .................................................................................................... 5300/8969
2019-08-28T17:51:52.6986935Z ...................................................................i................................ 5400/8969
2019-08-28T17:52:00.0291134Z .................................................................................................... 5500/8969
2019-08-28T17:52:06.9739285Z .................................................................................................... 5600/8969
2019-08-28T17:52:06.9739285Z .................................................................................................... 5600/8969
2019-08-28T17:52:17.3835175Z .............................................................ii...i..ii...........i................. 5700/8969
2019-08-28T17:52:42.8754361Z .................................................................................................... 5900/8969
2019-08-28T17:52:52.5960232Z .................................................................................................... 6000/8969
2019-08-28T17:52:52.5960232Z .................................................................................................... 6000/8969
2019-08-28T17:53:03.9490335Z ..............................................................i..ii................................. 6100/8969
2019-08-28T17:53:36.8950407Z .................................................................................................... 6300/8969
2019-08-28T17:53:39.7244022Z .................i.................................................................................. 6400/8969
2019-08-28T17:53:42.5441291Z .........................................................................................i.......... 6500/8969
2019-08-28T17:53:46.0615233Z .................................................................................................... 6600/8969
---
2019-08-28T17:58:06.8324433Z 
2019-08-28T17:58:06.8325015Z ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
2019-08-28T17:58:06.8325110Z diff of run.stderr:
2019-08-28T17:58:06.8325171Z 
2019-08-28T17:58:06.8325465Z - [$DIR/dbg-macro-expected-behavior.rs:23] Unit = Unit
2019-08-28T17:58:06.8325732Z - [$DIR/dbg-macro-expected-behavior.rs:24] a = Unit
2019-08-28T17:58:06.8326031Z - [$DIR/dbg-macro-expected-behavior.rs:30] Point{x: 42, y: 24,} = Point {
2019-08-28T17:58:06.8326352Z + [$DIR/dbg-macro-expected-behavior.rs:20] Unit = Unit
2019-08-28T17:58:06.8326664Z + [$DIR/dbg-macro-expected-behavior.rs:21] a = Unit
2019-08-28T17:58:06.8327171Z + [$DIR/dbg-macro-expected-behavior.rs:27] Point{x: 42, y: 24,} = Point {
2019-08-28T17:58:06.8327243Z 4     x: 42,
2019-08-28T17:58:06.8327286Z 5     y: 24,
2019-08-28T17:58:06.8327374Z 
2019-08-28T17:58:06.8327374Z 
2019-08-28T17:58:06.8327638Z - [$DIR/dbg-macro-expected-behavior.rs:31] b = Point {
2019-08-28T17:58:06.8327904Z + [$DIR/dbg-macro-expected-behavior.rs:28] b = Point {
2019-08-28T17:58:06.8327954Z 8     x: 42,
2019-08-28T17:58:06.8328172Z 9     y: 24,
2019-08-28T17:58:06.8328237Z 
2019-08-28T17:58:06.8328237Z 
2019-08-28T17:58:06.8328487Z - [$DIR/dbg-macro-expected-behavior.rs:39]
2019-08-28T17:58:06.8328887Z - [$DIR/dbg-macro-expected-behavior.rs:43] &a = NoCopy(
2019-08-28T17:58:06.8329255Z + [$DIR/dbg-macro-expected-behavior.rs:36]
2019-08-28T17:58:06.8330628Z + [$DIR/dbg-macro-expected-behavior.rs:40] &a = NoCopy(
2019-08-28T17:58:06.8330756Z 14 )
2019-08-28T17:58:06.8330756Z 14 )
2019-08-28T17:58:06.8331021Z - [$DIR/dbg-macro-expected-behavior.rs:43] dbg!(& a) = NoCopy(
2019-08-28T17:58:06.8331438Z + [$DIR/dbg-macro-expected-behavior.rs:40] dbg!(& a) = NoCopy(
2019-08-28T17:58:06.8331756Z 17 )
2019-08-28T17:58:06.8331756Z 17 )
2019-08-28T17:58:06.8332030Z - [$DIR/dbg-macro-expected-behavior.rs:48] f(&42) = 42
2019-08-28T17:58:06.8332594Z + [$DIR/dbg-macro-expected-behavior.rs:45] f(&42) = 42
2019-08-28T17:58:06.8332637Z 19 before
2019-08-28T17:58:06.8333466Z - [$DIR/dbg-macro-expected-behavior.rs:53] { foo += 1; eprintln!("before"); 7331 } = 7331
2019-08-28T17:58:06.8333775Z - [$DIR/dbg-macro-expected-behavior.rs:61] ("Yeah",) = (
2019-08-28T17:58:06.8334075Z + [$DIR/dbg-macro-expected-behavior.rs:50] { foo += 1; eprintln!("before"); 7331 } = 7331
2019-08-28T17:58:06.8334355Z + [$DIR/dbg-macro-expected-behavior.rs:58] ("Yeah",) = (
2019-08-28T17:58:06.8334418Z 22     "Yeah",
2019-08-28T17:58:06.8334459Z 23 )
2019-08-28T17:58:06.8334714Z - [$DIR/dbg-macro-expected-behavior.rs:64] 1 = 1
2019-08-28T17:58:06.8334990Z - [$DIR/dbg-macro-expected-behavior.rs:64] 2 = 2
2019-08-28T17:58:06.8335250Z - [$DIR/dbg-macro-expected-behavior.rs:68] 1u8 = 1
2019-08-28T17:58:06.8335505Z - [$DIR/dbg-macro-expected-behavior.rs:68] 2u32 = 2
2019-08-28T17:58:06.8335789Z - [$DIR/dbg-macro-expected-behavior.rs:68] "Yeah" = "Yeah"
2019-08-28T17:58:06.8336037Z + [$DIR/dbg-macro-expected-behavior.rs:61] 1 = 1
2019-08-28T17:58:06.8336284Z + [$DIR/dbg-macro-expected-behavior.rs:61] 2 = 2
2019-08-28T17:58:06.8336556Z + [$DIR/dbg-macro-expected-behavior.rs:65] 1u8 = 1
2019-08-28T17:58:06.8337118Z + [$DIR/dbg-macro-expected-behavior.rs:65] 2u32 = 2
2019-08-28T17:58:06.8337541Z + [$DIR/dbg-macro-expected-behavior.rs:65] "Yeah" = "Yeah"
2019-08-28T17:58:06.8337649Z 
2019-08-28T17:58:06.8337838Z 
2019-08-28T17:58:06.8338007Z The actual run.stderr differed from the expected run.stderr.
2019-08-28T17:58:06.8338007Z The actual run.stderr differed from the expected run.stderr.
2019-08-28T17:58:06.8338727Z Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/dbg-macro-expected-behavior.run.stderr
2019-08-28T17:58:06.8339052Z error: 1 errors occured comparing run output.
2019-08-28T17:58:06.8339097Z status: exit code: 0
2019-08-28T17:58:06.8339432Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a"
2019-08-28T17:58:06.8339483Z stdout:
2019-08-28T17:58:06.8339483Z stdout:
2019-08-28T17:58:06.8339711Z ------------------------------------------
2019-08-28T17:58:06.8339758Z 
2019-08-28T17:58:06.8339988Z ------------------------------------------
2019-08-28T17:58:06.8340032Z stderr:
2019-08-28T17:58:06.8340254Z ------------------------------------------
2019-08-28T17:58:06.8340552Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:20] Unit = Unit
2019-08-28T17:58:06.8340975Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:21] a = Unit
2019-08-28T17:58:06.8341491Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:27] Point{x: 42, y: 24,} = Point {
2019-08-28T17:58:06.8341621Z     x: 42,
2019-08-28T17:58:06.8341659Z     y: 24,
2019-08-28T17:58:06.8341696Z }
2019-08-28T17:58:06.8341982Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:28] b = Point {
2019-08-28T17:58:06.8342030Z     x: 42,
2019-08-28T17:58:06.8342067Z     y: 24,
2019-08-28T17:58:06.8342375Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:36]
2019-08-28T17:58:06.8342375Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:36]
2019-08-28T17:58:06.8342853Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:40] &a = NoCopy(
2019-08-28T17:58:06.8343449Z )
2019-08-28T17:58:06.8343449Z )
2019-08-28T17:58:06.8343793Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:40] dbg!(& a) = NoCopy(
2019-08-28T17:58:06.8343917Z )
2019-08-28T17:58:06.8343917Z )
2019-08-28T17:58:06.8344203Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:45] f(&42) = 42
2019-08-28T17:58:06.8344256Z before
2019-08-28T17:58:06.8344702Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:50] { foo += 1; eprintln!("before"); 7331 } = 7331
2019-08-28T17:58:06.8345037Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:58] ("Yeah",) = (
2019-08-28T17:58:06.8345090Z     "Yeah",
2019-08-28T17:58:06.8345148Z )
2019-08-28T17:58:06.8345424Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:61] 1 = 1
2019-08-28T17:58:06.8345705Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:61] 2 = 2
2019-08-28T17:58:06.8346008Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] 1u8 = 1
2019-08-28T17:58:06.8346296Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] 2u32 = 2
2019-08-28T17:58:06.8346585Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] "Yeah" = "Yeah"
2019-08-28T17:58:06.8347163Z ------------------------------------------
2019-08-28T17:58:06.8347195Z 
2019-08-28T17:58:06.8347220Z 
2019-08-28T17:58:06.8347415Z 
---
2019-08-28T17:58:06.8364938Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-28T17:58:06.8365025Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-28T17:58:06.8382470Z 
2019-08-28T17:58:06.8382573Z 
2019-08-28T17:58:06.8384914Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-28T17:58:06.8385394Z 
2019-08-28T17:58:06.8385437Z 
2019-08-28T17:58:06.8390758Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-28T17:58:06.8391040Z Build completed unsuccessfully in 1:06:01
2019-08-28T17:58:06.8391040Z Build completed unsuccessfully in 1:06:01
2019-08-28T17:58:06.8442274Z == clock drift check ==
2019-08-28T17:58:06.8458553Z   local time: Wed Aug 28 17:58:06 UTC 2019
2019-08-28T17:58:06.9965572Z   network time: Wed, 28 Aug 2019 17:58:06 GMT
2019-08-28T17:58:06.9968925Z == end clock drift check ==
2019-08-28T17:58:07.8098812Z ##[error]Bash exited with code '1'.
2019-08-28T17:58:07.8135174Z ##[section]Starting: Checkout
2019-08-28T17:58:07.8136999Z ==============================================================================
2019-08-28T17:58:07.8137046Z Task         : Get sources
2019-08-28T17:58:07.8137103Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
