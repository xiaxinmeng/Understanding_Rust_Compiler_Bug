plain
2020-02-22T16:10:38.3058155Z ========================== Starting Command Output ===========================
2020-02-22T16:10:38.3062978Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/54c7dbdd-48c9-462f-bb9e-e57555cbe701.sh
2020-02-22T16:10:38.3063363Z 
2020-02-22T16:10:38.3068193Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T16:10:38.3088038Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T16:10:38.3090860Z Task         : Get sources
2020-02-22T16:10:38.3091099Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T16:10:38.3091369Z Version      : 1.0.0
2020-02-22T16:10:38.3091528Z Author       : Microsoft
---
2020-02-22T16:10:39.5593565Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T16:10:39.5655855Z ##[command]git config gc.auto 0
2020-02-22T16:10:39.5659699Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T16:10:39.5662344Z ##[command]git config --get-all http.proxy
2020-02-22T16:10:39.5669161Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-02-22T17:10:31.5444283Z .................................................................................................... 1700/9699
2020-02-22T17:10:36.0487205Z .................................................................................................... 1800/9699
2020-02-22T17:10:46.8449047Z ..........................................i......................................................... 1900/9699
2020-02-22T17:10:54.1429908Z .................................................................................................... 2000/9699
2020-02-22T17:11:07.1480912Z ................................iiiii............................................................... 2100/9699
2020-02-22T17:11:16.2720671Z .................................................................................................... 2300/9699
2020-02-22T17:11:18.4354279Z .................................................................................................... 2400/9699
2020-02-22T17:11:22.5339714Z .................................................................................................... 2500/9699
2020-02-22T17:11:41.8628225Z .................................................................................................... 2600/9699
---
2020-02-22T17:14:10.6138117Z ........i........................................................................................... 5000/9699
2020-02-22T17:14:18.9014217Z .................................................................................................... 5100/9699
2020-02-22T17:14:23.2644366Z ...................................i................................................................ 5200/9699
2020-02-22T17:14:32.7352187Z .................................................................................................... 5300/9699
2020-02-22T17:14:38.3120167Z ...........ii.ii........i...i....................................................................... 5400/9699
2020-02-22T17:14:46.2800777Z .................................................................................................... 5600/9699
2020-02-22T17:14:56.2009939Z ....................................................................F............................... 5700/9699
2020-02-22T17:15:02.8454073Z ..i................................................................................................. 5800/9699
2020-02-22T17:15:07.7779313Z .................................................................................................... 5900/9699
2020-02-22T17:15:07.7779313Z .................................................................................................... 5900/9699
2020-02-22T17:15:16.8160865Z .............................................................................................ii...i. 6000/9699
2020-02-22T17:15:28.0481090Z .ii...........i..................................................................................... 6100/9699
2020-02-22T17:15:43.6966278Z .................................................................................................... 6300/9699
2020-02-22T17:15:49.9884466Z .................................................................................................... 6400/9699
2020-02-22T17:15:49.9884466Z .................................................................................................... 6400/9699
2020-02-22T17:16:06.1480618Z ........................i..ii....................................................................... 6500/9699
2020-02-22T17:16:25.3816979Z .................................................................................................... 6700/9699
2020-02-22T17:16:27.3614611Z ................i................................................................................... 6800/9699
2020-02-22T17:16:29.3439025Z .................................................................................................... 6900/9699
2020-02-22T17:16:31.5876340Z ......................................i............................................................. 7000/9699
---
2020-02-22T17:18:07.0599826Z .................................................................................................... 7700/9699
2020-02-22T17:18:11.8690610Z .................................................................................................... 7800/9699
2020-02-22T17:18:17.8028687Z ..................................................................................i................. 7900/9699
2020-02-22T17:18:26.0963955Z ...............................................................F.................................... 8000/9699
2020-02-22T17:18:33.0706276Z ...................................iiiiiii.i........................................................ 8100/9699
2020-02-22T17:18:46.7151574Z .................................................................................................... 8300/9699
2020-02-22T17:18:53.2484147Z .................................................................................................... 8400/9699
2020-02-22T17:19:07.1089766Z .................................................................................................... 8500/9699
2020-02-22T17:19:13.5284172Z .................................................................................................... 8600/9699
---
2020-02-22T17:21:01.3932843Z 
2020-02-22T17:21:01.3933633Z ---- [ui] ui/macros/issue-68060.rs stdout ----
2020-02-22T17:21:01.3934063Z diff of stderr:
2020-02-22T17:21:01.3934360Z 
2020-02-22T17:21:01.3934943Z - error: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-22T17:21:01.3935466Z + error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-22T17:21:01.3936465Z 3    |
2020-02-22T17:21:01.3936465Z 3    |
2020-02-22T17:21:01.3936828Z 4 LL |             #[target_feature(enable = "")]
2020-02-22T17:21:01.3938235Z -    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can only be applied to `unsafe` functions
2020-02-22T17:21:01.3938704Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-22T17:21:01.3938993Z 6 ...
2020-02-22T17:21:01.3939217Z 7 LL |             |_| (),
2020-02-22T17:21:01.3939217Z 7 LL |             |_| (),
2020-02-22T17:21:01.3939758Z 8    |             ------ not an `unsafe` function
2020-02-22T17:21:01.3940026Z 
2020-02-22T17:21:01.3940239Z +    |
2020-02-22T17:21:01.3940912Z +    = note: see issue #69098 <***/issues/69098> for more information
2020-02-22T17:21:01.3941533Z +    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-22T17:21:01.3942067Z 10 error: the feature named `` is not valid for this target
2020-02-22T17:21:01.3942555Z 11   --> $DIR/issue-68060.rs:8:30
2020-02-22T17:21:01.3942788Z 
2020-02-22T17:21:01.3942963Z 21 
---
2020-02-22T17:21:01.3945303Z 
2020-02-22T17:21:01.3945455Z 
2020-02-22T17:21:01.3945688Z The actual stderr differed from the expected stderr.
2020-02-22T17:21:01.3946320Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/issue-68060.stderr
2020-02-22T17:21:01.3946917Z To update references, rerun the tests and pass the `--bless` flag
2020-02-22T17:21:01.3947515Z To only update this specific test, also pass `--test-args macros/issue-68060.rs`
2020-02-22T17:21:01.3948237Z error: 1 errors occurred comparing output.
2020-02-22T17:21:01.3948528Z status: exit code: 1
2020-02-22T17:21:01.3948528Z status: exit code: 1
2020-02-22T17:21:01.3950255Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-68060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/auxiliary"
2020-02-22T17:21:01.3951874Z ------------------------------------------
2020-02-22T17:21:01.3952140Z 
2020-02-22T17:21:01.3952530Z ------------------------------------------
2020-02-22T17:21:01.3952789Z stderr:
2020-02-22T17:21:01.3952789Z stderr:
2020-02-22T17:21:01.3953197Z ------------------------------------------
2020-02-22T17:21:01.3953567Z error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-22T17:21:01.3954546Z    |
2020-02-22T17:21:01.3954546Z    |
2020-02-22T17:21:01.3954785Z LL |             #[target_feature(enable = "")]
2020-02-22T17:21:01.3955324Z ...
2020-02-22T17:21:01.3955525Z LL |             |_| (),
2020-02-22T17:21:01.3955972Z    |             ------ not an `unsafe` function
2020-02-22T17:21:01.3956239Z    |
2020-02-22T17:21:01.3956239Z    |
2020-02-22T17:21:01.3956796Z    = note: see issue #69098 <***/issues/69098> for more information
2020-02-22T17:21:01.3957201Z    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-22T17:21:01.3957924Z error: the feature named `` is not valid for this target
2020-02-22T17:21:01.3960010Z   --> /checkout/src/test/ui/macros/issue-68060.rs:8:30
2020-02-22T17:21:01.3960389Z    |
2020-02-22T17:21:01.3960389Z    |
2020-02-22T17:21:01.3960671Z LL |             #[target_feature(enable = "")]
2020-02-22T17:21:01.3961042Z    |                              ^^^^^^^^^^^ `` is not valid for this target
2020-02-22T17:21:01.3961576Z error[E0737]: `#[track_caller]` requires Rust ABI
2020-02-22T17:21:01.3962307Z   --> /checkout/src/test/ui/macros/issue-68060.rs:11:13
2020-02-22T17:21:01.3962624Z    |
2020-02-22T17:21:01.3962850Z LL |             #[track_caller]
---
2020-02-22T17:21:01.3968758Z ---- [ui] ui/rfcs/rfc-2396-target_feature-11/trait-impl.rs stdout ----
2020-02-22T17:21:01.3969081Z 
2020-02-22T17:21:01.3969327Z error: ui test compiled successfully!
2020-02-22T17:21:01.3969602Z status: exit code: 0
2020-02-22T17:21:01.3971684Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/trait-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/trait-impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/trait-impl/auxiliary"
2020-02-22T17:21:01.3973649Z ------------------------------------------
2020-02-22T17:21:01.3975766Z 
2020-02-22T17:21:01.3980615Z ------------------------------------------
2020-02-22T17:21:01.3981150Z stderr:
---
2020-02-22T17:21:01.3985709Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-22T17:21:01.3986021Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-22T17:21:01.3986237Z 
2020-02-22T17:21:01.3987255Z 
2020-02-22T17:21:01.3991321Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-22T17:21:01.3995187Z 
2020-02-22T17:21:01.3995286Z 
2020-02-22T17:21:01.3995486Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-22T17:21:01.3995732Z Build completed unsuccessfully in 1:03:35
2020-02-22T17:21:01.3995732Z Build completed unsuccessfully in 1:03:35
2020-02-22T17:21:01.4049376Z == clock drift check ==
2020-02-22T17:21:01.4065507Z   local time: Sat Feb 22 17:21:01 UTC 2020
2020-02-22T17:21:01.5487241Z   network time: Sat, 22 Feb 2020 17:21:01 GMT
2020-02-22T17:21:01.5487657Z == end clock drift check ==
2020-02-22T17:21:02.0103119Z 
2020-02-22T17:21:02.0167400Z ##[error]Bash exited with code '1'.
2020-02-22T17:21:02.0182534Z ##[section]Finishing: Run build
2020-02-22T17:21:02.0222521Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T17:21:02.0226537Z Task         : Get sources
2020-02-22T17:21:02.0227440Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T17:21:02.0227749Z Version      : 1.0.0
2020-02-22T17:21:02.0227982Z Author       : Microsoft
2020-02-22T17:21:02.0227982Z Author       : Microsoft
2020-02-22T17:21:02.0228328Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T17:21:02.0228726Z ==============================================================================
2020-02-22T17:21:02.3446068Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T17:21:02.3491752Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T17:21:02.3580212Z Cleaning up task key
2020-02-22T17:21:02.3581291Z Start cleaning up orphan processes.
2020-02-22T17:21:02.3753788Z Terminate orphan process: pid (5699) (python)
2020-02-22T17:21:02.3958836Z ##[section]Finishing: Finalize Job
