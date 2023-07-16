plain
2020-02-01T17:47:34.1293920Z ========================== Starting Command Output ===========================
2020-02-01T17:47:34.1295586Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3bc15b2b-5bad-4dce-9963-629cd70fcefb.sh
2020-02-01T17:47:34.1295626Z 
2020-02-01T17:47:34.1297917Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-01T17:47:34.1305093Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68737/merge to s
2020-02-01T17:47:34.1306904Z Task         : Get sources
2020-02-01T17:47:34.1306937Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T17:47:34.1307014Z Version      : 1.0.0
2020-02-01T17:47:34.1307047Z Author       : Microsoft
---
2020-02-01T17:47:35.1127043Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-01T17:47:35.1231337Z ##[command]git config gc.auto 0
2020-02-01T17:47:35.1320888Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-01T17:47:35.1387313Z ##[command]git config --get-all http.proxy
2020-02-01T17:47:35.1545857Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68737/merge:refs/remotes/pull/68737/merge
---
2020-02-01T18:47:16.4439220Z .................................................................................................... 1700/9561
2020-02-01T18:47:21.5965159Z .................................................................................................... 1800/9561
2020-02-01T18:47:34.9780333Z .........................i.......................................................................... 1900/9561
2020-02-01T18:47:42.2866976Z .................................................................................................... 2000/9561
2020-02-01T18:47:57.6858836Z ...............iiiii................................................................................ 2100/9561
2020-02-01T18:48:07.7931469Z .................................................................................................... 2300/9561
2020-02-01T18:48:10.3021229Z .................................................................................................... 2400/9561
2020-02-01T18:48:15.5132958Z .................................................................................................... 2500/9561
2020-02-01T18:48:37.2805361Z .................................................................................................... 2600/9561
---
2020-02-01T18:51:17.2185263Z .................................................................................................... 4800/9561
2020-02-01T18:51:22.4530288Z ..........................................................i...............i......................... 4900/9561
2020-02-01T18:51:30.4999424Z .................................................................................................... 5000/9561
2020-02-01T18:51:38.9711932Z .................................................................................................... 5100/9561
2020-02-01T18:51:43.8872468Z .i.................................................................................................. 5200/9561
2020-02-01T18:51:55.3005517Z ...........................................................................ii.ii........i...i....... 5300/9561
2020-02-01T18:52:03.9721263Z .............i...................................................................................... 5500/9561
2020-02-01T18:52:14.2907815Z .................................................................................................... 5600/9561
2020-02-01T18:52:20.8301047Z ..............................................................i..................................... 5700/9561
2020-02-01T18:52:28.1077471Z .................................................................................................... 5800/9561
2020-02-01T18:52:28.1077471Z .................................................................................................... 5800/9561
2020-02-01T18:52:36.0646615Z .................................................................................................... 5900/9561
2020-02-01T18:52:45.2466910Z .....................................................ii...i..ii...........i......................... 6000/9561
2020-02-01T18:53:07.4174726Z .................................................................................................... 6200/9561
2020-02-01T18:53:15.2465055Z .................................................................................................... 6300/9561
2020-02-01T18:53:15.2465055Z .................................................................................................... 6300/9561
2020-02-01T18:53:23.6112555Z .................................................................................i..ii.............. 6400/9561
2020-02-01T18:53:50.8481624Z .................................................................................................... 6600/9561
2020-02-01T18:53:56.5194524Z .........................................................i.......................................... 6700/9561
2020-02-01T18:53:58.7833022Z .................................................................................................... 6800/9561
2020-02-01T18:54:01.1938689Z ..........................................................i......................................... 6900/9561
---
2020-02-01T18:55:44.6511086Z .................................................................................................... 7600/9561
2020-02-01T18:55:50.0931905Z .................................................................................................... 7700/9561
2020-02-01T18:55:56.9601556Z ..........................................................F......................................... 7800/9561
2020-02-01T18:56:07.5601455Z .................................................................................................... 7900/9561
2020-02-01T18:56:13.7225485Z ...............iiiiiiii............................................................................. 8000/9561
2020-02-01T18:56:28.6583982Z .................................................................................................... 8200/9561
2020-02-01T18:56:39.2245001Z .................................................................................................... 8300/9561
2020-02-01T18:56:52.9000913Z .................................................................................................... 8400/9561
2020-02-01T18:56:59.9654038Z .................................................................................................... 8500/9561
---
2020-02-01T18:58:59.3304418Z 
2020-02-01T18:58:59.3304960Z ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
2020-02-01T18:58:59.3305385Z diff of run.stderr:
2020-02-01T18:58:59.3305419Z 
2020-02-01T18:58:59.3305811Z 1 [$DIR/dbg-macro-expected-behavior.rs:20] Unit = Unit
2020-02-01T18:58:59.3306055Z 2 [$DIR/dbg-macro-expected-behavior.rs:21] a = Unit
2020-02-01T18:58:59.3306881Z - [$DIR/dbg-macro-expected-behavior.rs:27] Point{x: 42, y: 24,} = Point {
2020-02-01T18:58:59.3307194Z + [$DIR/dbg-macro-expected-behavior.rs:27] Point { x: 42,  y: 24,} = Point {
2020-02-01T18:58:59.3307254Z 4     x: 42,
2020-02-01T18:58:59.3307301Z 5     y: 24,
2020-02-01T18:58:59.3307393Z 
2020-02-01T18:58:59.3307421Z 
2020-02-01T18:58:59.3307469Z The actual run.stderr differed from the expected run.stderr.
2020-02-01T18:58:59.3307469Z The actual run.stderr differed from the expected run.stderr.
2020-02-01T18:58:59.3307850Z Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/dbg-macro-expected-behavior.run.stderr
2020-02-01T18:58:59.3307945Z error: 1 errors occurred comparing run output.
2020-02-01T18:58:59.3308025Z status: exit code: 0
2020-02-01T18:58:59.3308337Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a"
2020-02-01T18:58:59.3308392Z stdout:
2020-02-01T18:58:59.3308392Z stdout:
2020-02-01T18:58:59.3308610Z ------------------------------------------
2020-02-01T18:58:59.3308662Z 
2020-02-01T18:58:59.3308876Z ------------------------------------------
2020-02-01T18:58:59.3308924Z stderr:
2020-02-01T18:58:59.3309151Z ------------------------------------------
2020-02-01T18:58:59.3309419Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:20] Unit = Unit
2020-02-01T18:58:59.3309685Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:21] a = Unit
2020-02-01T18:58:59.3310320Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:27] Point { x: 42,  y: 24,} = Point {
2020-02-01T18:58:59.3310368Z     x: 42,
2020-02-01T18:58:59.3310404Z     y: 24,
2020-02-01T18:58:59.3310449Z }
2020-02-01T18:58:59.3310697Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:28] b = Point {
2020-02-01T18:58:59.3310741Z     x: 42,
2020-02-01T18:58:59.3310785Z     y: 24,
2020-02-01T18:58:59.3311062Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:36]
2020-02-01T18:58:59.3311062Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:36]
2020-02-01T18:58:59.3311296Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:40] &a = NoCopy(
2020-02-01T18:58:59.3311399Z )
2020-02-01T18:58:59.3311399Z )
2020-02-01T18:58:59.3311635Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:40] dbg!(& a) = NoCopy(
2020-02-01T18:58:59.3311733Z )
2020-02-01T18:58:59.3311733Z )
2020-02-01T18:58:59.3311963Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:45] f(&42) = 42
2020-02-01T18:58:59.3312007Z before
2020-02-01T18:58:59.3312290Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:50] { foo += 1; eprintln!("before"); 7331 } = 7331
2020-02-01T18:58:59.3312539Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:58] ("Yeah",) = (
2020-02-01T18:58:59.3312592Z     "Yeah",
2020-02-01T18:58:59.3312646Z )
2020-02-01T18:58:59.3312872Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:61] 1 = 1
2020-02-01T18:58:59.3313099Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:61] 2 = 2
2020-02-01T18:58:59.3313349Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] 1u8 = 1
2020-02-01T18:58:59.3313579Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] 2u32 = 2
2020-02-01T18:58:59.3313999Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] "Yeah" = "Yeah"
2020-02-01T18:58:59.3314244Z ------------------------------------------
2020-02-01T18:58:59.3314272Z 
2020-02-01T18:58:59.3314297Z 
2020-02-01T18:58:59.3314322Z 
---
2020-02-01T18:58:59.3330539Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-01T18:58:59.3330615Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-01T18:58:59.3340327Z 
2020-02-01T18:58:59.3341491Z 
2020-02-01T18:58:59.3343881Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-01T18:58:59.3344363Z 
2020-02-01T18:58:59.3344420Z 
2020-02-01T18:58:59.3359459Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-01T18:58:59.3359538Z Build completed unsuccessfully in 1:05:48
2020-02-01T18:58:59.3359538Z Build completed unsuccessfully in 1:05:48
2020-02-01T18:58:59.3415202Z == clock drift check ==
2020-02-01T18:58:59.3436230Z   local time: Sat Feb  1 18:58:59 UTC 2020
2020-02-01T18:58:59.6395254Z   network time: Sat, 01 Feb 2020 18:58:59 GMT
2020-02-01T18:58:59.6397166Z == end clock drift check ==
2020-02-01T18:59:00.0037076Z 
2020-02-01T18:59:00.0148319Z ##[error]Bash exited with code '1'.
2020-02-01T18:59:00.0161606Z ##[section]Finishing: Run build
2020-02-01T18:59:00.0188191Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68737/merge to s
2020-02-01T18:59:00.0190440Z Task         : Get sources
2020-02-01T18:59:00.0190484Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T18:59:00.0190529Z Version      : 1.0.0
2020-02-01T18:59:00.0190608Z Author       : Microsoft
2020-02-01T18:59:00.0190608Z Author       : Microsoft
2020-02-01T18:59:00.0190652Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-01T18:59:00.0190700Z ==============================================================================
2020-02-01T18:59:00.4573383Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-01T18:59:00.4618235Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68737/merge to s
2020-02-01T18:59:00.4738661Z Cleaning up task key
2020-02-01T18:59:00.4763816Z Start cleaning up orphan processes.
2020-02-01T18:59:00.4875257Z Terminate orphan process: pid (4321) (python)
2020-02-01T18:59:00.5160773Z ##[section]Finishing: Finalize Job
