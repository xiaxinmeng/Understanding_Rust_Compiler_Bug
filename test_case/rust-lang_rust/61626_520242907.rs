plain
2019-08-11T15:38:28.4621752Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-11T15:38:28.4803235Z ##[command]git config gc.auto 0
2019-08-11T15:38:28.4882879Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-11T15:38:28.4933395Z ##[command]git config --get-all http.proxy
2019-08-11T15:38:28.5057339Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61626/merge:refs/remotes/pull/61626/merge
---
2019-08-11T15:39:02.0851449Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-11T15:39:02.0851693Z 
2019-08-11T15:39:02.0852253Z   git checkout -b <new-branch-name>
2019-08-11T15:39:02.0852491Z 
2019-08-11T15:39:02.0852676Z HEAD is now at 642f8797a Merge a44a72ef37bcc7aa9f78bea0503b211681fa552b into 2b78e10ac1454d2d4190c575f6ece03f484ac398
2019-08-11T15:39:02.1005782Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-11T15:39:02.1009532Z ==============================================================================
2019-08-11T15:39:02.1009608Z Task         : Bash
2019-08-11T15:39:02.1009654Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-11T16:36:17.6145994Z .................................................................................................... 1300/8871
2019-08-11T16:36:23.6685562Z .................................................................................................... 1400/8871
2019-08-11T16:36:29.5502371Z .................................................................................................... 1500/8871
2019-08-11T16:36:39.4917449Z ....................................................................................i............... 1600/8871
2019-08-11T16:36:46.6112077Z i................................................................................................... 1700/8871
2019-08-11T16:36:52.8625463Z ...........................................................................iiiii.................... 1800/8871
2019-08-11T16:37:13.3120944Z .................................................................................................... 2000/8871
2019-08-11T16:37:15.4930144Z .................................................................................................... 2100/8871
2019-08-11T16:37:17.9529233Z .................................................................................................... 2200/8871
2019-08-11T16:37:25.1611313Z .................................................................................................... 2300/8871
---
2019-08-11T16:41:04.7165984Z .................................................................................................... 5300/8871
2019-08-11T16:41:11.4115710Z ......i............................................................................................. 5400/8871
2019-08-11T16:41:16.4117267Z .................................................................................................... 5500/8871
2019-08-11T16:41:28.0097165Z .................................................................................................... 5600/8871
2019-08-11T16:41:41.1629624Z .ii...i..ii...........i............................................................................. 5700/8871
2019-08-11T16:41:55.7834253Z .................................................................................................... 5900/8871
2019-08-11T16:42:00.1071705Z .................................................................................................... 6000/8871
2019-08-11T16:42:00.1071705Z .................................................................................................... 6000/8871
2019-08-11T16:42:13.4028133Z ..i..ii............................................................................................. 6100/8871
2019-08-11T16:42:30.8934397Z .............................................i...................................................... 6300/8871
2019-08-11T16:42:32.8590221Z .................................................................................................... 6400/8871
2019-08-11T16:42:35.1812422Z .................i.................................................................................. 6500/8871
2019-08-11T16:42:39.3208422Z .................................................................................................... 6600/8871
---
2019-08-11T16:46:21.5702973Z failures:
2019-08-11T16:46:21.5703035Z 
2019-08-11T16:46:21.5703577Z ---- [compile-fail] compile-fail/consts/issue-55878.rs stdout ----
2019-08-11T16:46:21.5703800Z 
2019-08-11T16:46:21.5704840Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-08-11T16:46:21.5704888Z status: exit code: 101
2019-08-11T16:46:21.5706067Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/consts/issue-55878.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/consts/issue-55878" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/consts/issue-55878/auxiliary" "-A" "unused"
2019-08-11T16:46:21.5706994Z ------------------------------------------
2019-08-11T16:46:21.5707032Z 
2019-08-11T16:46:21.5707197Z ------------------------------------------
2019-08-11T16:46:21.5707254Z stderr:
2019-08-11T16:46:21.5707254Z stderr:
2019-08-11T16:46:21.5707411Z ------------------------------------------
2019-08-11T16:46:21.5708623Z thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: SizeOverflow([u8; 18446744073709551615])', src/libcore/result.rs:1084:5
2019-08-11T16:46:21.5708765Z 
2019-08-11T16:46:21.5708798Z error: internal compiler error: unexpected panic
2019-08-11T16:46:21.5708820Z 
2019-08-11T16:46:21.5708871Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-11T16:46:21.5708871Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-11T16:46:21.5708893Z 
2019-08-11T16:46:21.5709296Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-11T16:46:21.5709535Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-08-11T16:46:21.5709559Z 
2019-08-11T16:46:21.5709559Z 
2019-08-11T16:46:21.5709760Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-11T16:46:21.5709827Z 
2019-08-11T16:46:21.5709985Z ------------------------------------------
2019-08-11T16:46:21.5710008Z 
2019-08-11T16:46:21.5710045Z 
---
2019-08-11T16:46:21.5711881Z 
2019-08-11T16:46:21.5712140Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-11T16:46:21.5713834Z 
2019-08-11T16:46:21.5713884Z 
2019-08-11T16:46:21.5715488Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-11T16:46:21.5715720Z 
2019-08-11T16:46:21.5715742Z 
2019-08-11T16:46:21.5721453Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-11T16:46:21.5721571Z Build completed unsuccessfully in 1:01:18
2019-08-11T16:46:21.5721571Z Build completed unsuccessfully in 1:01:18
2019-08-11T16:46:22.3546838Z ##[error]Bash exited with code '1'.
2019-08-11T16:46:22.3587975Z ##[section]Starting: Checkout
2019-08-11T16:46:22.3589450Z ==============================================================================
2019-08-11T16:46:22.3589496Z Task         : Get sources
2019-08-11T16:46:22.3589553Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
