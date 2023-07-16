plain
2020-02-22T14:49:56.1836863Z ========================== Starting Command Output ===========================
2020-02-22T14:49:56.1841500Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/386b50ee-d074-4aba-b61e-11dc8feebd39.sh
2020-02-22T14:49:56.1841934Z 
2020-02-22T14:49:56.1846087Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T14:49:56.1864284Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T14:49:56.1867696Z Task         : Get sources
2020-02-22T14:49:56.1867956Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T14:49:56.1868207Z Version      : 1.0.0
2020-02-22T14:49:56.1868427Z Author       : Microsoft
---
2020-02-22T14:49:57.4775870Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T14:49:57.4787328Z ##[command]git config gc.auto 0
2020-02-22T14:49:57.4791330Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T14:49:57.4794329Z ##[command]git config --get-all http.proxy
2020-02-22T14:49:57.4802451Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-02-22T15:50:06.4396873Z .................................................................................................... 1700/9699
2020-02-22T15:50:10.8699922Z .................................................................................................... 1800/9699
2020-02-22T15:50:22.1254569Z ..........................................i......................................................... 1900/9699
2020-02-22T15:50:30.1900325Z .................................................................................................... 2000/9699
2020-02-22T15:50:44.3055555Z ................................iiiii............................................................... 2100/9699
2020-02-22T15:50:54.1862156Z .................................................................................................... 2300/9699
2020-02-22T15:50:56.6543922Z .................................................................................................... 2400/9699
2020-02-22T15:51:01.1106777Z .................................................................................................... 2500/9699
2020-02-22T15:51:22.1515791Z .................................................................................................... 2600/9699
---
2020-02-22T15:53:55.4872779Z ........i........................................................................................... 5000/9699
2020-02-22T15:54:04.2358851Z .................................................................................................... 5100/9699
2020-02-22T15:54:08.8380481Z ...................................i................................................................ 5200/9699
2020-02-22T15:54:18.6137726Z .................................................................................................... 5300/9699
2020-02-22T15:54:24.2766351Z ...........ii.ii........i...i....................................................................... 5400/9699
2020-02-22T15:54:32.9831055Z .................................................................................................... 5600/9699
2020-02-22T15:54:43.5952146Z ....................................................................F............................... 5700/9699
2020-02-22T15:54:50.9613319Z ..i................................................................................................. 5800/9699
2020-02-22T15:54:56.2359270Z .................................................................................................... 5900/9699
2020-02-22T15:54:56.2359270Z .................................................................................................... 5900/9699
2020-02-22T15:55:06.0742363Z .............................................................................................ii...i. 6000/9699
2020-02-22T15:55:18.0965052Z .ii...........i..................................................................................... 6100/9699
2020-02-22T15:55:34.9412100Z .................................................................................................... 6300/9699
2020-02-22T15:55:41.3089813Z .................................................................................................... 6400/9699
2020-02-22T15:55:41.3089813Z .................................................................................................... 6400/9699
2020-02-22T15:55:55.8847756Z ........................i..ii....................................................................... 6500/9699
2020-02-22T15:56:15.9398257Z .................................................................................................... 6700/9699
2020-02-22T15:56:18.1996929Z ................i................................................................................... 6800/9699
2020-02-22T15:56:20.2716277Z .................................................................................................... 6900/9699
2020-02-22T15:56:22.6406431Z ......................................i............................................................. 7000/9699
---
2020-02-22T15:58:01.0340364Z .................................................................................................... 7700/9699
2020-02-22T15:58:05.7920187Z .................................................................................................... 7800/9699
2020-02-22T15:58:11.9762499Z ..................................................................................i................. 7900/9699
2020-02-22T15:58:20.0013236Z ...............................................................F.................................... 8000/9699
2020-02-22T15:58:26.6976175Z ...................................iiiiiii.i........................................................ 8100/9699
2020-02-22T15:58:40.4582008Z .................................................................................................... 8300/9699
2020-02-22T15:58:46.6767391Z .................................................................................................... 8400/9699
2020-02-22T15:59:00.4331012Z .................................................................................................... 8500/9699
2020-02-22T15:59:06.9886807Z .................................................................................................... 8600/9699
---
2020-02-22T16:01:00.4170777Z 
2020-02-22T16:01:00.4171649Z ---- [ui] ui/macros/issue-68060.rs stdout ----
2020-02-22T16:01:00.4172125Z diff of stderr:
2020-02-22T16:01:00.4172501Z 
2020-02-22T16:01:00.4173142Z - error: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-22T16:01:00.4173722Z + error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-22T16:01:00.4174816Z 3    |
2020-02-22T16:01:00.4174816Z 3    |
2020-02-22T16:01:00.4175193Z 4 LL |             #[target_feature(enable = "")]
2020-02-22T16:01:00.4176183Z -    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can only be applied to `unsafe` functions
2020-02-22T16:01:00.4176893Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-22T16:01:00.4177320Z 6 ...
2020-02-22T16:01:00.4177646Z 7 LL |             |_| (),
2020-02-22T16:01:00.4177646Z 7 LL |             |_| (),
2020-02-22T16:01:00.4178478Z 8    |             ------ not an `unsafe` function
2020-02-22T16:01:00.4179125Z 
2020-02-22T16:01:00.4179812Z +    |
2020-02-22T16:01:00.4180690Z +    = note: see issue #69098 <***/issues/69098> for more information
2020-02-22T16:01:00.4181650Z +    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-22T16:01:00.4183077Z 10 error: the feature named `` is not valid for this target
2020-02-22T16:01:00.4183788Z 11   --> $DIR/issue-68060.rs:8:30
2020-02-22T16:01:00.4184199Z 
2020-02-22T16:01:00.4186201Z 21 
---
2020-02-22T16:01:00.4190857Z 
2020-02-22T16:01:00.4191248Z 
2020-02-22T16:01:00.4191582Z The actual stderr differed from the expected stderr.
2020-02-22T16:01:00.4192543Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/issue-68060.stderr
2020-02-22T16:01:00.4193518Z To update references, rerun the tests and pass the `--bless` flag
2020-02-22T16:01:00.4195323Z To only update this specific test, also pass `--test-args macros/issue-68060.rs`
2020-02-22T16:01:00.4198710Z error: 1 errors occurred comparing output.
2020-02-22T16:01:00.4199207Z status: exit code: 1
2020-02-22T16:01:00.4199207Z status: exit code: 1
2020-02-22T16:01:00.4201306Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-68060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/auxiliary"
2020-02-22T16:01:00.4202986Z ------------------------------------------
2020-02-22T16:01:00.4203772Z 
2020-02-22T16:01:00.4204512Z ------------------------------------------
2020-02-22T16:01:00.4205052Z stderr:
2020-02-22T16:01:00.4205052Z stderr:
2020-02-22T16:01:00.4205540Z ------------------------------------------
2020-02-22T16:01:00.4206166Z error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-22T16:01:00.4207360Z    |
2020-02-22T16:01:00.4207360Z    |
2020-02-22T16:01:00.4207678Z LL |             #[target_feature(enable = "")]
2020-02-22T16:01:00.4208326Z ...
2020-02-22T16:01:00.4208595Z LL |             |_| (),
2020-02-22T16:01:00.4211844Z    |             ------ not an `unsafe` function
2020-02-22T16:01:00.4212289Z    |
2020-02-22T16:01:00.4212289Z    |
2020-02-22T16:01:00.4213049Z    = note: see issue #69098 <***/issues/69098> for more information
2020-02-22T16:01:00.4213566Z    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-22T16:01:00.4214392Z error: the feature named `` is not valid for this target
2020-02-22T16:01:00.4215026Z   --> /checkout/src/test/ui/macros/issue-68060.rs:8:30
2020-02-22T16:01:00.4215375Z    |
2020-02-22T16:01:00.4215375Z    |
2020-02-22T16:01:00.4215690Z LL |             #[target_feature(enable = "")]
2020-02-22T16:01:00.4221693Z    |                              ^^^^^^^^^^^ `` is not valid for this target
2020-02-22T16:01:00.4222439Z error[E0737]: `#[track_caller]` requires Rust ABI
2020-02-22T16:01:00.4223407Z   --> /checkout/src/test/ui/macros/issue-68060.rs:11:13
2020-02-22T16:01:00.4223798Z    |
2020-02-22T16:01:00.4224064Z LL |             #[track_caller]
---
2020-02-22T16:01:00.4228187Z ---- [ui] ui/rfcs/rfc-2396-target_feature-11/trait-impl.rs stdout ----
2020-02-22T16:01:00.4228407Z 
2020-02-22T16:01:00.4228573Z error: ui test compiled successfully!
2020-02-22T16:01:00.4228916Z status: exit code: 0
2020-02-22T16:01:00.4230872Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/trait-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/trait-impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/trait-impl/auxiliary"
2020-02-22T16:01:00.4232422Z ------------------------------------------
2020-02-22T16:01:00.4232580Z 
2020-02-22T16:01:00.4233983Z ------------------------------------------
2020-02-22T16:01:00.4234231Z stderr:
---
2020-02-22T16:01:00.4237964Z test result: FAILED. 9643 passed; 2 failed; 54 ignored; 0 measured; 0 filtered out
2020-02-22T16:01:00.4238211Z 
2020-02-22T16:01:00.4238299Z 
2020-02-22T16:01:00.4238384Z 
2020-02-22T16:01:00.4241723Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-22T16:01:00.4244860Z 
2020-02-22T16:01:00.4244959Z 
2020-02-22T16:01:00.4245971Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-22T16:01:00.4246376Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-22T16:01:00.4246376Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-22T16:01:00.4248285Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-22T16:01:00.4248591Z Build completed unsuccessfully in 1:04:52
2020-02-22T16:01:00.4278086Z == clock drift check ==
2020-02-22T16:01:00.4304908Z   local time: Sat Feb 22 16:01:00 UTC 2020
2020-02-22T16:01:00.7196892Z   network time: Sat, 22 Feb 2020 16:01:00 GMT
2020-02-22T16:01:00.7201642Z == end clock drift check ==
2020-02-22T16:01:01.1652254Z 
2020-02-22T16:01:01.1723888Z ##[error]Bash exited with code '1'.
2020-02-22T16:01:01.1767621Z ##[section]Finishing: Run build
2020-02-22T16:01:01.1821558Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T16:01:01.1826403Z Task         : Get sources
2020-02-22T16:01:01.1826703Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T16:01:01.1826963Z Version      : 1.0.0
2020-02-22T16:01:01.1827144Z Author       : Microsoft
2020-02-22T16:01:01.1827144Z Author       : Microsoft
2020-02-22T16:01:01.1827452Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T16:01:01.1827786Z ==============================================================================
2020-02-22T16:01:01.5068873Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T16:01:01.5108070Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T16:01:01.5189062Z Cleaning up task key
2020-02-22T16:01:01.5190077Z Start cleaning up orphan processes.
2020-02-22T16:01:01.5344521Z Terminate orphan process: pid (3881) (python)
2020-02-22T16:01:01.5590661Z ##[section]Finishing: Finalize Job
