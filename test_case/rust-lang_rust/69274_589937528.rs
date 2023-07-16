plain
2020-02-22T08:29:55.8763347Z ========================== Starting Command Output ===========================
2020-02-22T08:29:55.8765813Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9e0cee25-2da1-4870-89b3-10a8fc304662.sh
2020-02-22T08:29:55.8766026Z 
2020-02-22T08:29:55.8770089Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T08:29:55.8790624Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T08:29:55.8794963Z Task         : Get sources
2020-02-22T08:29:55.8795265Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T08:29:55.8795512Z Version      : 1.0.0
2020-02-22T08:29:55.8795915Z Author       : Microsoft
---
2020-02-22T08:29:56.8987564Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T08:29:56.8992433Z ##[command]git config gc.auto 0
2020-02-22T08:29:56.8995879Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T08:29:56.8999144Z ##[command]git config --get-all http.proxy
2020-02-22T08:29:56.9004942Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-02-22T09:26:48.0161364Z .................................................................................................... 1700/9698
2020-02-22T09:26:52.0779649Z .................................................................................................... 1800/9698
2020-02-22T09:27:02.1877485Z ..........................................i......................................................... 1900/9698
2020-02-22T09:27:09.1230152Z .................................................................................................... 2000/9698
2020-02-22T09:27:21.4450196Z ................................iiiii............................................................... 2100/9698
2020-02-22T09:27:29.9515857Z .................................................................................................... 2300/9698
2020-02-22T09:27:32.0739889Z .................................................................................................... 2400/9698
2020-02-22T09:27:35.9826698Z .................................................................................................... 2500/9698
2020-02-22T09:27:53.9091736Z .................................................................................................... 2600/9698
---
2020-02-22T09:30:15.6499423Z ........i........................................................................................... 5000/9698
2020-02-22T09:30:24.0726761Z .................................................................................................... 5100/9698
2020-02-22T09:30:28.4740120Z ...................................i................................................................ 5200/9698
2020-02-22T09:30:37.6673766Z .................................................................................................... 5300/9698
2020-02-22T09:30:42.8634507Z ...........ii.ii........i...i....................................................................... 5400/9698
2020-02-22T09:30:50.5270674Z .................................................................................................... 5600/9698
2020-02-22T09:31:00.0546228Z .....................................................................F.............................. 5700/9698
2020-02-22T09:31:06.5847946Z ..i................................................................................................. 5800/9698
2020-02-22T09:31:11.5789041Z .................................................................................................... 5900/9698
2020-02-22T09:31:11.5789041Z .................................................................................................... 5900/9698
2020-02-22T09:31:20.4049244Z .............................................................................................ii...i. 6000/9698
2020-02-22T09:31:31.0955295Z .ii...........i..................................................................................... 6100/9698
2020-02-22T09:31:45.9312520Z .................................................................................................... 6300/9698
2020-02-22T09:31:51.7247050Z .................................................................................................... 6400/9698
2020-02-22T09:31:51.7247050Z .................................................................................................... 6400/9698
2020-02-22T09:32:03.9380140Z ........................i..ii....................................................................... 6500/9698
2020-02-22T09:32:21.7594045Z .................................................................................................... 6700/9698
2020-02-22T09:32:23.6345765Z ................i................................................................................... 6800/9698
2020-02-22T09:32:25.5206170Z .................................................................................................... 6900/9698
2020-02-22T09:32:27.7261511Z ......................................i............................................................. 7000/9698
---
2020-02-22T09:33:51.2091111Z .................................................................................................... 7600/9698
2020-02-22T09:33:56.1505477Z .................................................................................................... 7700/9698
2020-02-22T09:34:00.5332674Z .................................................................................................... 7800/9698
2020-02-22T09:34:06.0636234Z ..................................................................................i................. 7900/9698
2020-02-22T09:34:13.7132449Z ...........................................................F..F..................................... 8000/9698
2020-02-22T09:34:20.0664966Z ..................................iiiiiii.i......................................................... 8100/9698
2020-02-22T09:34:32.7074679Z .................................................................................................... 8300/9698
2020-02-22T09:34:40.0444532Z .................................................................................................... 8400/9698
2020-02-22T09:34:52.1152043Z .................................................................................................... 8500/9698
2020-02-22T09:34:58.2199308Z .................................................................................................... 8600/9698
---
2020-02-22T09:36:45.4546962Z 
2020-02-22T09:36:45.4548421Z ---- [ui] ui/macros/issue-68060.rs stdout ----
2020-02-22T09:36:45.4548933Z diff of stderr:
2020-02-22T09:36:45.4549342Z 
2020-02-22T09:36:45.4550139Z - error: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-22T09:36:45.4550997Z + error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-22T09:36:45.4553475Z 3    |
2020-02-22T09:36:45.4553475Z 3    |
2020-02-22T09:36:45.4553941Z 4 LL |             #[target_feature(enable = "")]
2020-02-22T09:36:45.4555067Z -    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can only be applied to `unsafe` functions
2020-02-22T09:36:45.4555759Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-22T09:36:45.4556048Z 6 ...
2020-02-22T09:36:45.4556319Z 7 LL |             |_| (),
2020-02-22T09:36:45.4556319Z 7 LL |             |_| (),
2020-02-22T09:36:45.4557327Z 8    |             ------ not an `unsafe` function
2020-02-22T09:36:45.4557670Z 
2020-02-22T09:36:45.4558226Z +    |
2020-02-22T09:36:45.4558904Z +    = note: see issue #69098 <***/issues/69098> for more information
2020-02-22T09:36:45.4559554Z +    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-22T09:36:45.4560191Z 10 error: the feature named `` is not valid for this target
2020-02-22T09:36:45.4560761Z 11   --> $DIR/issue-68060.rs:8:30
2020-02-22T09:36:45.4561030Z 
2020-02-22T09:36:45.4561229Z 21 
---
2020-02-22T09:36:45.4563878Z 
2020-02-22T09:36:45.4564050Z 
2020-02-22T09:36:45.4564332Z The actual stderr differed from the expected stderr.
2020-02-22T09:36:45.4565228Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/issue-68060.stderr
2020-02-22T09:36:45.4565970Z To update references, rerun the tests and pass the `--bless` flag
2020-02-22T09:36:45.4566635Z To only update this specific test, also pass `--test-args macros/issue-68060.rs`
2020-02-22T09:36:45.4567249Z error: 1 errors occurred comparing output.
2020-02-22T09:36:45.4567532Z status: exit code: 1
2020-02-22T09:36:45.4567532Z status: exit code: 1
2020-02-22T09:36:45.4569395Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-68060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/auxiliary"
2020-02-22T09:36:45.4571340Z ------------------------------------------
2020-02-22T09:36:45.4571624Z 
2020-02-22T09:36:45.4572333Z ------------------------------------------
2020-02-22T09:36:45.4572664Z stderr:
2020-02-22T09:36:45.4572664Z stderr:
2020-02-22T09:36:45.4573127Z ------------------------------------------
2020-02-22T09:36:45.4573710Z error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-22T09:36:45.4574678Z    |
2020-02-22T09:36:45.4574678Z    |
2020-02-22T09:36:45.4575127Z LL |             #[target_feature(enable = "")]
2020-02-22T09:36:45.4575705Z ...
2020-02-22T09:36:45.4575932Z LL |             |_| (),
2020-02-22T09:36:45.4576439Z    |             ------ not an `unsafe` function
2020-02-22T09:36:45.4578947Z    |
2020-02-22T09:36:45.4578947Z    |
2020-02-22T09:36:45.4579907Z    = note: see issue #69098 <***/issues/69098> for more information
2020-02-22T09:36:45.4580426Z    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-22T09:36:45.4581048Z error: the feature named `` is not valid for this target
2020-02-22T09:36:45.4581961Z   --> /checkout/src/test/ui/macros/issue-68060.rs:8:30
2020-02-22T09:36:45.4582318Z    |
2020-02-22T09:36:45.4582318Z    |
2020-02-22T09:36:45.4582590Z LL |             #[target_feature(enable = "")]
2020-02-22T09:36:45.4582980Z    |                              ^^^^^^^^^^^ `` is not valid for this target
2020-02-22T09:36:45.4583538Z error[E0737]: `#[track_caller]` requires Rust ABI
2020-02-22T09:36:45.4584109Z   --> /checkout/src/test/ui/macros/issue-68060.rs:11:13
2020-02-22T09:36:45.4584460Z    |
2020-02-22T09:36:45.4585049Z LL |             #[track_caller]
---
2020-02-22T09:36:45.4592303Z 
2020-02-22T09:36:45.4593696Z ------------------------------------------
2020-02-22T09:36:45.4594253Z 
2020-02-22T09:36:45.4594478Z 
2020-02-22T09:36:45.4595184Z ---- [ui] ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs stdout ----
2020-02-22T09:36:45.4595760Z 
2020-02-22T09:36:45.4595867Z 3    |
2020-02-22T09:36:45.4595867Z 3    |
2020-02-22T09:36:45.4596040Z 4 LL | #[target_feature(enable = "sse2")]
2020-02-22T09:36:45.4597213Z - LL | fn foo() {} 
2020-02-22T09:36:45.4597372Z + LL | fn foo() {}
2020-02-22T09:36:45.4598051Z 7    | ----------- not an `unsafe` function
2020-02-22T09:36:45.4598214Z 8    |
2020-02-22T09:36:45.4598214Z 8    |
2020-02-22T09:36:45.4598721Z 9    = note: see issue #69098 <***/issues/69098> for more information
2020-02-22T09:36:45.4599166Z 
2020-02-22T09:36:45.4599340Z The actual stderr differed from the expected stderr.
2020-02-22T09:36:45.4599340Z The actual stderr differed from the expected stderr.
2020-02-22T09:36:45.4600064Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11/feature-gate-target_feature_11.stderr
2020-02-22T09:36:45.4601073Z To update references, rerun the tests and pass the `--bless` flag
2020-02-22T09:36:45.4601637Z To only update this specific test, also pass `--test-args rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs`
2020-02-22T09:36:45.4602061Z error: 1 errors occurred comparing output.
2020-02-22T09:36:45.4602250Z status: exit code: 1
2020-02-22T09:36:45.4602250Z status: exit code: 1
2020-02-22T09:36:45.4604233Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11/auxiliary"
2020-02-22T09:36:45.4605649Z ------------------------------------------
2020-02-22T09:36:45.4605785Z 
2020-02-22T09:36:45.4606069Z ------------------------------------------
2020-02-22T09:36:45.4606243Z stderr:
2020-02-22T09:36:45.4606243Z stderr:
2020-02-22T09:36:45.4606588Z ------------------------------------------
2020-02-22T09:36:45.4606868Z error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-22T09:36:45.4607405Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs:1:1
2020-02-22T09:36:45.4607657Z    |
2020-02-22T09:36:45.4607896Z LL | #[target_feature(enable = "sse2")] //~ ERROR can only be applied to `unsafe` functions
2020-02-22T09:36:45.4608328Z LL | fn foo() {}
2020-02-22T09:36:45.4608657Z    | ----------- not an `unsafe` function
2020-02-22T09:36:45.4608811Z    |
2020-02-22T09:36:45.4608811Z    |
2020-02-22T09:36:45.4609248Z    = note: see issue #69098 <***/issues/69098> for more information
2020-02-22T09:36:45.4609572Z    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-22T09:36:45.4609909Z error: aborting due to previous error
2020-02-22T09:36:45.4610036Z 
2020-02-22T09:36:45.4610398Z For more information about this error, try `rustc --explain E0658`.
2020-02-22T09:36:45.4610568Z 
2020-02-22T09:36:45.4610568Z 
2020-02-22T09:36:45.4610851Z ------------------------------------------
2020-02-22T09:36:45.4610986Z 
2020-02-22T09:36:45.4611078Z 
2020-02-22T09:36:45.4611431Z ---- [ui] ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs stdout ----
2020-02-22T09:36:45.4611704Z 
2020-02-22T09:36:45.4612253Z error: /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:23: expected error not found: cannot be applied to safe trait method
2020-02-22T09:36:45.4612742Z error: 0 unexpected errors found, 1 expected errors not found
2020-02-22T09:36:45.4612951Z status: exit code: 1
2020-02-22T09:36:45.4612951Z status: exit code: 1
2020-02-22T09:36:45.4614624Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls/auxiliary"
2020-02-22T09:36:45.4615952Z     Error {
2020-02-22T09:36:45.4616095Z         line_num: 23,
2020-02-22T09:36:45.4616268Z         kind: Some(
2020-02-22T09:36:45.4616415Z             Error,
2020-02-22T09:36:45.4616415Z             Error,
2020-02-22T09:36:45.4616542Z         ),
2020-02-22T09:36:45.4616739Z         msg: "cannot be applied to safe trait method",
2020-02-22T09:36:45.4616916Z     },
2020-02-22T09:36:45.4617016Z ]
2020-02-22T09:36:45.4617095Z 
2020-02-22T09:36:45.4617639Z thread '[ui] ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-02-22T09:36:45.4618218Z 
2020-02-22T09:36:45.4618309Z 
2020-02-22T09:36:45.4618410Z failures:
2020-02-22T09:36:45.4618691Z     [ui] ui/macros/issue-68060.rs
2020-02-22T09:36:45.4618691Z     [ui] ui/macros/issue-68060.rs
2020-02-22T09:36:45.4619289Z     [ui] ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs
2020-02-22T09:36:45.4620121Z     [ui] ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs
2020-02-22T09:36:45.4620723Z test result: FAILED. 9641 passed; 3 failed; 54 ignored; 0 measured; 0 filtered out
2020-02-22T09:36:45.4620970Z 
2020-02-22T09:36:45.4621390Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-22T09:36:45.4621611Z 
2020-02-22T09:36:45.4621611Z 
2020-02-22T09:36:45.4621693Z 
2020-02-22T09:36:45.4625328Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-22T09:36:45.4627955Z 
2020-02-22T09:36:45.4628049Z 
2020-02-22T09:36:45.4640641Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-22T09:36:45.4641423Z Build completed unsuccessfully in 0:59:58
2020-02-22T09:36:45.4641423Z Build completed unsuccessfully in 0:59:58
2020-02-22T09:36:45.4700399Z == clock drift check ==
2020-02-22T09:36:45.4723792Z   local time: Sat Feb 22 09:36:45 UTC 2020
2020-02-22T09:36:45.7693441Z   network time: Sat, 22 Feb 2020 09:36:45 GMT
2020-02-22T09:36:45.7698698Z == end clock drift check ==
2020-02-22T09:36:46.2919556Z 
2020-02-22T09:36:46.2956947Z ##[error]Bash exited with code '1'.
2020-02-22T09:36:46.2972790Z ##[section]Finishing: Run build
2020-02-22T09:36:46.3015997Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T09:36:46.3019887Z Task         : Get sources
2020-02-22T09:36:46.3020148Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T09:36:46.3020411Z Version      : 1.0.0
2020-02-22T09:36:46.3020580Z Author       : Microsoft
2020-02-22T09:36:46.3020580Z Author       : Microsoft
2020-02-22T09:36:46.3021030Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T09:36:46.3021360Z ==============================================================================
2020-02-22T09:36:46.6038146Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T09:36:46.6084687Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T09:36:46.6163685Z Cleaning up task key
2020-02-22T09:36:46.6164970Z Start cleaning up orphan processes.
2020-02-22T09:36:46.6324997Z Terminate orphan process: pid (4612) (python)
2020-02-22T09:36:46.6546445Z ##[section]Finishing: Finalize Job
