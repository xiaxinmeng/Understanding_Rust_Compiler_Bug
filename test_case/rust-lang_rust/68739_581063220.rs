plain
2020-02-01T18:42:05.4454127Z ========================== Starting Command Output ===========================
2020-02-01T18:42:05.4455740Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5cbb9bd0-9888-4b30-a94c-c3f8e552e2a7.sh
2020-02-01T18:42:05.4455828Z 
2020-02-01T18:42:05.4458902Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-01T18:42:05.4465170Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68739/merge to s
2020-02-01T18:42:05.4467800Z Task         : Get sources
2020-02-01T18:42:05.4467834Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T18:42:05.4467867Z Version      : 1.0.0
2020-02-01T18:42:05.4467901Z Author       : Microsoft
---
2020-02-01T18:42:06.4281457Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-01T18:42:06.4291090Z ##[command]git config gc.auto 0
2020-02-01T18:42:06.4292933Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-01T18:42:06.4294467Z ##[command]git config --get-all http.proxy
2020-02-01T18:42:06.4300968Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68739/merge:refs/remotes/pull/68739/merge
---
2020-02-01T19:36:43.9941131Z .................................................................................................... 1700/9561
2020-02-01T19:36:48.8647619Z .................................................................................................... 1800/9561
2020-02-01T19:37:01.1570863Z .........................i.......................................................................... 1900/9561
2020-02-01T19:37:07.9921127Z .................................................................................................... 2000/9561
2020-02-01T19:37:22.1039602Z ...............iiiii................................................................................ 2100/9561
2020-02-01T19:37:31.3333272Z .................................................................................................... 2300/9561
2020-02-01T19:37:33.5928649Z .................................................................................................... 2400/9561
2020-02-01T19:37:38.4482782Z .................................................................................................... 2500/9561
2020-02-01T19:37:58.9232556Z .................................................................................................... 2600/9561
---
2020-02-01T19:40:25.8004389Z .................................................................................................... 4800/9561
2020-02-01T19:40:30.5841628Z ..........................................................i...............i......................... 4900/9561
2020-02-01T19:40:37.9474537Z .................................................................................................... 5000/9561
2020-02-01T19:40:45.5405670Z .................................................................................................... 5100/9561
2020-02-01T19:40:50.0824278Z .i.................................................................................................. 5200/9561
2020-02-01T19:41:00.4824115Z ...........................................................................ii.ii........i...i....... 5300/9561
2020-02-01T19:41:08.4151108Z .............i...................................................................................... 5500/9561
2020-02-01T19:41:17.9052537Z .................................................................................................... 5600/9561
2020-02-01T19:41:23.8918779Z ..............................................................i..................................... 5700/9561
2020-02-01T19:41:30.5956317Z .................................................................................................... 5800/9561
2020-02-01T19:41:30.5956317Z .................................................................................................... 5800/9561
2020-02-01T19:41:38.0330572Z .................................................................................................... 5900/9561
2020-02-01T19:41:46.5783233Z .....................................................ii...i..ii...........i......................... 6000/9561
2020-02-01T19:42:07.0338176Z .................................................................................................... 6200/9561
2020-02-01T19:42:13.9845613Z .................................................................................................... 6300/9561
2020-02-01T19:42:13.9845613Z .................................................................................................... 6300/9561
2020-02-01T19:42:21.9662870Z .................................................................................i..ii.............. 6400/9561
2020-02-01T19:42:49.1563199Z .................................................................................................... 6600/9561
2020-02-01T19:42:54.2973140Z .........................................................i.......................................... 6700/9561
2020-02-01T19:42:56.2704761Z .................................................................................................... 6800/9561
2020-02-01T19:42:58.3200297Z ..........................................................i......................................... 6900/9561
---
2020-02-01T19:44:34.8265154Z .................................................................................................... 7600/9561
2020-02-01T19:44:39.9468884Z .................................................................................................... 7700/9561
2020-02-01T19:44:46.3814335Z ..........................................................F......................................... 7800/9561
2020-02-01T19:44:56.5081330Z .................................................................................................... 7900/9561
2020-02-01T19:45:02.2793980Z ..............iiiiiii.i............................................................................. 8000/9561
2020-02-01T19:45:16.3792644Z .................................................................................................... 8200/9561
2020-02-01T19:45:26.6691673Z .................................................................................................... 8300/9561
2020-02-01T19:45:39.5216731Z .................................................................................................... 8400/9561
2020-02-01T19:45:46.2358366Z .................................................................................................... 8500/9561
---
2020-02-01T19:47:36.8348457Z 
2020-02-01T19:47:36.8348901Z ---- [ui] ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs stdout ----
2020-02-01T19:47:36.8349090Z diff of run.stderr:
2020-02-01T19:47:36.8349237Z 
2020-02-01T19:47:36.8349569Z 1 [$DIR/dbg-macro-expected-behavior.rs:20] Unit = Unit
2020-02-01T19:47:36.8350231Z 2 [$DIR/dbg-macro-expected-behavior.rs:21] a = Unit
2020-02-01T19:47:36.8350708Z - [$DIR/dbg-macro-expected-behavior.rs:27] Point{x: 42, y: 24,} = Point {
2020-02-01T19:47:36.8351177Z + [$DIR/dbg-macro-expected-behavior.rs:27] Point { x: 42,  y: 24,} = Point {
2020-02-01T19:47:36.8351378Z 4     x: 42,
2020-02-01T19:47:36.8351567Z 5     y: 24,
2020-02-01T19:47:36.8351841Z 
2020-02-01T19:47:36.8351986Z 
2020-02-01T19:47:36.8352142Z The actual run.stderr differed from the expected run.stderr.
2020-02-01T19:47:36.8352142Z The actual run.stderr differed from the expected run.stderr.
2020-02-01T19:47:36.8352688Z Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/dbg-macro-expected-behavior.run.stderr
2020-02-01T19:47:36.8353065Z error: 1 errors occurred comparing run output.
2020-02-01T19:47:36.8353219Z status: exit code: 0
2020-02-01T19:47:36.8354434Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior/a"
2020-02-01T19:47:36.8354624Z stdout:
2020-02-01T19:47:36.8354624Z stdout:
2020-02-01T19:47:36.8354933Z ------------------------------------------
2020-02-01T19:47:36.8355098Z 
2020-02-01T19:47:36.8355425Z ------------------------------------------
2020-02-01T19:47:36.8355588Z stderr:
2020-02-01T19:47:36.8355886Z ------------------------------------------
2020-02-01T19:47:36.8356318Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:20] Unit = Unit
2020-02-01T19:47:36.8356713Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:21] a = Unit
2020-02-01T19:47:36.8357315Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:27] Point { x: 42,  y: 24,} = Point {
2020-02-01T19:47:36.8357528Z     x: 42,
2020-02-01T19:47:36.8357672Z     y: 24,
2020-02-01T19:47:36.8357795Z }
2020-02-01T19:47:36.8358173Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:28] b = Point {
2020-02-01T19:47:36.8358347Z     x: 42,
2020-02-01T19:47:36.8358494Z     y: 24,
2020-02-01T19:47:36.8358984Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:36]
2020-02-01T19:47:36.8358984Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:36]
2020-02-01T19:47:36.8359566Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:40] &a = NoCopy(
2020-02-01T19:47:36.8359990Z )
2020-02-01T19:47:36.8359990Z )
2020-02-01T19:47:36.8360613Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:40] dbg!(& a) = NoCopy(
2020-02-01T19:47:36.8360963Z )
2020-02-01T19:47:36.8360963Z )
2020-02-01T19:47:36.8361388Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:45] f(&42) = 42
2020-02-01T19:47:36.8361607Z before
2020-02-01T19:47:36.8362078Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:50] { foo += 1; eprintln!("before"); 7331 } = 7331
2020-02-01T19:47:36.8362763Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:58] ("Yeah",) = (
2020-02-01T19:47:36.8364094Z     "Yeah",
2020-02-01T19:47:36.8365133Z )
2020-02-01T19:47:36.8366509Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:61] 1 = 1
2020-02-01T19:47:36.8368536Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:61] 2 = 2
2020-02-01T19:47:36.8371687Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] 1u8 = 1
2020-02-01T19:47:36.8372238Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] 2u32 = 2
2020-02-01T19:47:36.8372528Z [/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:65] "Yeah" = "Yeah"
2020-02-01T19:47:36.8372832Z ------------------------------------------
2020-02-01T19:47:36.8372869Z 
2020-02-01T19:47:36.8372895Z 
2020-02-01T19:47:36.8372921Z 
---
2020-02-01T19:47:36.8374542Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-01T19:47:36.8374607Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-01T19:47:36.8385803Z 
2020-02-01T19:47:36.8386287Z 
2020-02-01T19:47:36.8388759Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-01T19:47:36.8389034Z 
2020-02-01T19:47:36.8389081Z 
2020-02-01T19:47:36.8401997Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-01T19:47:36.8402105Z Build completed unsuccessfully in 1:00:04
2020-02-01T19:47:36.8402105Z Build completed unsuccessfully in 1:00:04
2020-02-01T19:47:36.8461806Z == clock drift check ==
2020-02-01T19:47:36.8482890Z   local time: Sat Feb  1 19:47:36 UTC 2020
2020-02-01T19:47:37.1434783Z   network time: Sat, 01 Feb 2020 19:47:37 GMT
2020-02-01T19:47:37.1436207Z == end clock drift check ==
2020-02-01T19:47:37.7739531Z 
2020-02-01T19:47:37.7853168Z ##[error]Bash exited with code '1'.
2020-02-01T19:47:37.7866249Z ##[section]Finishing: Run build
2020-02-01T19:47:37.7903759Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68739/merge to s
2020-02-01T19:47:37.7905764Z Task         : Get sources
2020-02-01T19:47:37.7905822Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T19:47:37.7905861Z Version      : 1.0.0
2020-02-01T19:47:37.7905896Z Author       : Microsoft
2020-02-01T19:47:37.7905896Z Author       : Microsoft
2020-02-01T19:47:37.7905952Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-01T19:47:37.7905994Z ==============================================================================
2020-02-01T19:47:38.2019095Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-01T19:47:38.2056885Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68739/merge to s
2020-02-01T19:47:38.2161645Z Cleaning up task key
2020-02-01T19:47:38.2162425Z Start cleaning up orphan processes.
2020-02-01T19:47:38.2255865Z Terminate orphan process: pid (4000) (python)
2020-02-01T19:47:38.2447343Z ##[section]Finishing: Finalize Job
