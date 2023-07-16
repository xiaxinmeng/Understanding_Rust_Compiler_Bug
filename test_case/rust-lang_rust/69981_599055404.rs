plain
2020-03-14T11:36:30.2594119Z ========================== Starting Command Output ===========================
2020-03-14T11:36:30.2598783Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e161e647-91b6-4be9-9855-0ce7f523a067.sh
2020-03-14T11:36:30.2599241Z 
2020-03-14T11:36:30.2603609Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-14T11:36:30.2622032Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-14T11:36:30.2625245Z Task         : Get sources
2020-03-14T11:36:30.2625537Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T11:36:30.2625835Z Version      : 1.0.0
2020-03-14T11:36:30.2626029Z Author       : Microsoft
---
2020-03-14T11:36:31.4422490Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-14T11:36:31.4433926Z ##[command]git config gc.auto 0
2020-03-14T11:36:31.4440272Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-14T11:36:31.4446646Z ##[command]git config --get-all http.proxy
2020-03-14T11:36:31.4456681Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69981/merge:refs/remotes/pull/69981/merge
---
2020-03-14T12:39:32.3480059Z .................................................................................................... 1700/9772
2020-03-14T12:39:36.8154928Z .................................................................................................... 1800/9772
2020-03-14T12:39:48.6113677Z ....................................................................i............................... 1900/9772
2020-03-14T12:39:55.5269778Z .................................................................................................... 2000/9772
2020-03-14T12:40:10.2811706Z ...........................................................iiiii.................................... 2100/9772
2020-03-14T12:40:21.1883158Z .................................................................................................... 2300/9772
2020-03-14T12:40:23.4089051Z .................................................................................................... 2400/9772
2020-03-14T12:40:26.5042405Z .................................................................................................... 2500/9772
2020-03-14T12:40:47.5931626Z .................................................................................................... 2600/9772
---
2020-03-14T12:43:29.1207505Z ..............................i...............i..................................................... 5000/9772
2020-03-14T12:43:38.7421842Z .................................................................................................... 5100/9772
2020-03-14T12:43:44.8709094Z .........................................................................i.......................... 5200/9772
2020-03-14T12:43:50.4326362Z .................................................................................................... 5300/9772
2020-03-14T12:44:00.5173393Z ......................................................ii.ii........i...i............................ 5400/9772
2020-03-14T12:44:08.6145236Z .................................................................................................... 5600/9772
2020-03-14T12:44:18.4108203Z .................................................................................................... 5700/9772
2020-03-14T12:44:24.7050415Z ..............................................i..................................................... 5800/9772
2020-03-14T12:44:31.3894898Z .................................................................................................... 5900/9772
2020-03-14T12:44:31.3894898Z .................................................................................................... 5900/9772
2020-03-14T12:44:41.6067269Z .................................................................................................... 6000/9772
2020-03-14T12:44:47.7481200Z ........................................ii...i..ii...........i...................................... 6100/9772
2020-03-14T12:45:08.1627760Z .................................................................................................... 6300/9772
2020-03-14T12:45:15.1776965Z .................................................................................................... 6400/9772
2020-03-14T12:45:15.1776965Z .................................................................................................... 6400/9772
2020-03-14T12:45:24.6545223Z ......................................................................i..ii......................... 6500/9772
2020-03-14T12:45:53.5614386Z .................................................................................................... 6700/9772
2020-03-14T12:46:02.2765772Z ....................................................................i............................... 6800/9772
2020-03-14T12:46:04.2853205Z .................................................................................................... 6900/9772
2020-03-14T12:46:06.4413678Z .................................................................................................... 7000/9772
---
2020-03-14T12:47:53.1150625Z .................................................................................................... 7800/9772
2020-03-14T12:47:59.3075796Z .................................................................................................... 7900/9772
2020-03-14T12:48:05.1766426Z ....................................................i............................................... 8000/9772
2020-03-14T12:48:15.7087849Z .................................................................................................... 8100/9772
2020-03-14T12:48:21.1950439Z .iiiiiiiiii.i....................................................................................... 8200/9772
2020-03-14T12:48:35.2022641Z .................................................................................................... 8400/9772
2020-03-14T12:48:46.5693852Z .................................................................................................... 8500/9772
2020-03-14T12:48:59.5206169Z .................................................................................................... 8600/9772
2020-03-14T12:49:05.4262740Z .................................................................................................... 8700/9772
---
2020-03-14T12:50:59.6805284Z ---- [ui] ui/const-generics/issues/issue-61336-2.rs stdout ----
2020-03-14T12:50:59.6808889Z 
2020-03-14T12:50:59.6809467Z error: test compilation failed although it shouldn't!
2020-03-14T12:50:59.6809936Z status: exit code: 1
2020-03-14T12:50:59.6812111Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-61336-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336-2/auxiliary"
2020-03-14T12:50:59.6813792Z ------------------------------------------
2020-03-14T12:50:59.6813970Z 
2020-03-14T12:50:59.6814344Z ------------------------------------------
2020-03-14T12:50:59.6814548Z stderr:
---
2020-03-14T12:50:59.6817203Z 
2020-03-14T12:50:59.6817465Z error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
2020-03-14T12:50:59.6818066Z   --> /checkout/src/test/ui/const-generics/issues/issue-61336-2.rs:11:5
2020-03-14T12:50:59.6818339Z    |
2020-03-14T12:50:59.6818490Z LL |     [x; {N}]
2020-03-14T12:50:59.6819075Z    |
2020-03-14T12:50:59.6819362Z help: consider restricting this type parameter with `T: std::marker::Copy`
2020-03-14T12:50:59.6820539Z   --> /checkout/src/test/ui/const-generics/issues/issue-61336-2.rs:10:6
2020-03-14T12:50:59.6820822Z    |
2020-03-14T12:50:59.6820822Z    |
2020-03-14T12:50:59.6821448Z LL | fn g<T, const N: usize>(x: T) -> [T; N] {
2020-03-14T12:50:59.6822200Z    = note: the `Copy` trait is required because the repeated element will be copied
2020-03-14T12:50:59.6822481Z 
2020-03-14T12:50:59.6822665Z error: aborting due to previous error
2020-03-14T12:50:59.6822832Z 
---
2020-03-14T12:50:59.6825529Z ---- [ui] ui/const-generics/issues/issue-61336.rs stdout ----
2020-03-14T12:50:59.6825739Z 
2020-03-14T12:50:59.6826342Z error: test compilation failed although it shouldn't!
2020-03-14T12:50:59.6826856Z status: exit code: 1
2020-03-14T12:50:59.6829786Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-61336.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61336/auxiliary"
2020-03-14T12:50:59.6832581Z ------------------------------------------
2020-03-14T12:50:59.6832766Z 
2020-03-14T12:50:59.6833163Z ------------------------------------------
2020-03-14T12:50:59.6833367Z stderr:
---
2020-03-14T12:50:59.6854796Z    |
2020-03-14T12:50:59.6855083Z help: consider restricting this type parameter with `T: std::marker::Copy`
2020-03-14T12:50:59.6856886Z   --> /checkout/src/test/ui/const-generics/issues/issue-61336.rs:10:6
2020-03-14T12:50:59.6858004Z    |
2020-03-14T12:50:59.6859032Z LL | fn g<T, const N: usize>(x: T) -> [T; N] {
2020-03-14T12:50:59.6859930Z    = note: the `Copy` trait is required because the repeated element will be copied
2020-03-14T12:50:59.6860208Z 
2020-03-14T12:50:59.6860394Z error: aborting due to previous error
2020-03-14T12:50:59.6860559Z 
---
2020-03-14T12:50:59.6865972Z -   --> $DIR/issue-62504.rs:18:25
2020-03-14T12:50:59.6866238Z + error[E0308]: mismatched types
2020-03-14T12:50:59.6866650Z +   --> $DIR/issue-62504.rs:18:21
2020-03-14T12:50:59.6866873Z 3    |
2020-03-14T12:50:59.6867098Z 4 LL |         ArrayHolder([0; Self::SIZE])
2020-03-14T12:50:59.6868417Z +    |                     ^^^^^^^^^^^^^^^ expected `X`, found `Self::SIZE`
2020-03-14T12:50:59.6869020Z +    |
2020-03-14T12:50:59.6869020Z +    |
2020-03-14T12:50:59.6869224Z +    = note: expected array `[u32; _]`
2020-03-14T12:50:59.6869499Z +               found array `[u32; _]`
2020-03-14T12:50:59.6869874Z 7 error: aborting due to previous error
2020-03-14T12:50:59.6870251Z 8 
2020-03-14T12:50:59.6870363Z 
2020-03-14T12:50:59.6870902Z + For more information about this error, try `rustc --explain E0308`.
2020-03-14T12:50:59.6870902Z + For more information about this error, try `rustc --explain E0308`.
2020-03-14T12:50:59.6871365Z 9 
2020-03-14T12:50:59.6871492Z 
2020-03-14T12:50:59.6871589Z 
2020-03-14T12:50:59.6871794Z The actual stderr differed from the expected stderr.
2020-03-14T12:50:59.6872558Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62504/issue-62504.stderr
2020-03-14T12:50:59.6873751Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T12:50:59.6874486Z To only update this specific test, also pass `--test-args const-generics/issues/issue-62504.rs`
2020-03-14T12:50:59.6874970Z error: 1 errors occurred comparing output.
2020-03-14T12:50:59.6875538Z status: exit code: 1
2020-03-14T12:50:59.6875538Z status: exit code: 1
2020-03-14T12:50:59.6877825Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-62504.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62504" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62504/auxiliary"
2020-03-14T12:50:59.6879623Z ------------------------------------------
2020-03-14T12:50:59.6879803Z 
2020-03-14T12:50:59.6880164Z ------------------------------------------
2020-03-14T12:50:59.6880366Z stderr:
2020-03-14T12:50:59.6880366Z stderr:
2020-03-14T12:50:59.6881208Z ------------------------------------------
2020-03-14T12:50:59.6881469Z error[E0308]: mismatched types
2020-03-14T12:50:59.6881980Z   --> /checkout/src/test/ui/const-generics/issues/issue-62504.rs:18:21
2020-03-14T12:50:59.6882574Z    |
2020-03-14T12:50:59.6882790Z LL |         ArrayHolder([0; Self::SIZE])
2020-03-14T12:50:59.6883143Z    |                     ^^^^^^^^^^^^^^^ expected `X`, found `Self::SIZE`
2020-03-14T12:50:59.6883442Z    |
2020-03-14T12:50:59.6883630Z    = note: expected array `[u32; _]`
2020-03-14T12:50:59.6883877Z               found array `[u32; _]`
2020-03-14T12:50:59.6884248Z error: aborting due to previous error
2020-03-14T12:50:59.6884413Z 
2020-03-14T12:50:59.6884918Z For more information about this error, try `rustc --explain E0308`.
2020-03-14T12:50:59.6885156Z 
2020-03-14T12:50:59.6885156Z 
2020-03-14T12:50:59.6885515Z ------------------------------------------
2020-03-14T12:50:59.6885687Z 
2020-03-14T12:50:59.6885783Z 
2020-03-14T12:50:59.6886202Z ---- [ui] ui/const-generics/issues/issue-67739.rs stdout ----
2020-03-14T12:50:59.6886410Z 
2020-03-14T12:50:59.6886668Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-14T12:50:59.6887389Z status: exit code: 101
2020-03-14T12:50:59.6889677Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67739.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739/auxiliary"
2020-03-14T12:50:59.6891350Z ------------------------------------------
2020-03-14T12:50:59.6891527Z 
2020-03-14T12:50:59.6891900Z ------------------------------------------
2020-03-14T12:50:59.6892101Z stderr:
2020-03-14T12:50:59.6892101Z stderr:
2020-03-14T12:50:59.6892466Z ------------------------------------------
2020-03-14T12:50:59.6893330Z error: internal compiler error: src/librustc_traits/normalize_erasing_regions.rs:35: could not fully normalize `fn() -> usize {std::mem::size_of::<<Self as Trait>::Associated>}`
2020-03-14T12:50:59.6894269Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-03-14T12:50:59.6894663Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T12:50:59.6894913Z 
2020-03-14T12:50:59.6895132Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-14T12:50:59.6895132Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-14T12:50:59.6895332Z 
2020-03-14T12:50:59.6896025Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-14T12:50:59.6896792Z note: rustc 1.43.0-nightly (9385783ed 2020-03-14) running on x86_64-unknown-linux-gnu
2020-03-14T12:50:59.6897043Z 
2020-03-14T12:50:59.6897043Z 
2020-03-14T12:50:59.6897687Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-14T12:50:59.6898224Z error: aborting due to previous error
2020-03-14T12:50:59.6898405Z 
2020-03-14T12:50:59.6898501Z 
2020-03-14T12:50:59.6898856Z ------------------------------------------
---
2020-03-14T12:50:59.6903784Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-14T12:50:59.6909268Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T12:50:59.6910397Z 
2020-03-14T12:50:59.6917261Z 
2020-03-14T12:50:59.6921547Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-14T12:50:59.6932291Z 
2020-03-14T12:50:59.6932421Z 
2020-03-14T12:50:59.6932713Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-14T12:50:59.6933085Z Build completed unsuccessfully in 1:08:45
2020-03-14T12:50:59.6933085Z Build completed unsuccessfully in 1:08:45
2020-03-14T12:50:59.6933421Z == clock drift check ==
2020-03-14T12:50:59.6957021Z   local time: Sat Mar 14 12:50:59 UTC 2020
2020-03-14T12:50:59.7895666Z   network time: Sat, 14 Mar 2020 12:50:59 GMT
2020-03-14T12:50:59.7900331Z == end clock drift check ==
2020-03-14T12:51:00.1660622Z 
2020-03-14T12:51:00.1743701Z ##[error]Bash exited with code '1'.
2020-03-14T12:51:00.1762342Z ##[section]Finishing: Run build
2020-03-14T12:51:00.1827077Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-14T12:51:00.1832546Z Task         : Get sources
2020-03-14T12:51:00.1832948Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T12:51:00.1833322Z Version      : 1.0.0
2020-03-14T12:51:00.1833569Z Author       : Microsoft
2020-03-14T12:51:00.1833569Z Author       : Microsoft
2020-03-14T12:51:00.1833960Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-14T12:51:00.1834436Z ==============================================================================
2020-03-14T12:51:00.5386361Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-14T12:51:00.5427804Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-14T12:51:00.5520727Z Cleaning up task key
2020-03-14T12:51:00.5522122Z Start cleaning up orphan processes.
2020-03-14T12:51:00.5729161Z Terminate orphan process: pid (3454) (python)
2020-03-14T12:51:00.5921759Z ##[section]Finishing: Finalize Job
