plain
2020-02-02T22:51:38.7616769Z ========================== Starting Command Output ===========================
2020-02-02T22:51:38.7618948Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ed40b5dd-afdb-4de3-83f4-15e1b2956ba8.sh
2020-02-02T22:51:38.7618988Z 
2020-02-02T22:51:38.7621470Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-02T22:51:38.7626604Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-02T22:51:38.7628191Z Task         : Get sources
2020-02-02T22:51:38.7628220Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T22:51:38.7628248Z Version      : 1.0.0
2020-02-02T22:51:38.7628290Z Author       : Microsoft
---
2020-02-02T22:51:39.7090014Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-02T22:51:39.7182086Z ##[command]git config gc.auto 0
2020-02-02T22:51:39.7246377Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-02T22:51:39.7312481Z ##[command]git config --get-all http.proxy
2020-02-02T22:51:39.7437133Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68654/merge:refs/remotes/pull/68654/merge
---
2020-02-02T23:45:00.0049332Z .................................................................................................... 1700/9566
2020-02-02T23:45:04.6879509Z ..................................F.F............................................................... 1800/9566
2020-02-02T23:45:16.9872413Z ..........................i......................................................................... 1900/9566
2020-02-02T23:45:23.8747290Z .................................................................................................... 2000/9566
2020-02-02T23:45:38.3116094Z ................iiiii............................................................................... 2100/9566
2020-02-02T23:45:47.6083290Z .................................................................................................... 2300/9566
2020-02-02T23:45:49.8757789Z .................................................................................................... 2400/9566
2020-02-02T23:45:54.5252481Z .................................................................................................... 2500/9566
2020-02-02T23:46:14.2929365Z .................................................................................................... 2600/9566
---
2020-02-02T23:48:41.0859705Z .................................................................................................... 4800/9566
2020-02-02T23:48:46.3244308Z ...........................................................i...............i........................ 4900/9566
2020-02-02T23:48:54.2329240Z .................................................................................................... 5000/9566
2020-02-02T23:49:01.4190097Z .................................................................................................... 5100/9566
2020-02-02T23:49:06.2139192Z ..i................................................................................................. 5200/9566
2020-02-02T23:49:17.6908617Z ............................................................................ii.ii........i...i...... 5300/9566
2020-02-02T23:49:26.5241233Z ..............i..................................................................................... 5500/9566
2020-02-02T23:49:35.9368356Z .................................................................................................... 5600/9566
2020-02-02T23:49:42.2426021Z ...............................................................i.................................... 5700/9566
2020-02-02T23:49:49.1389113Z .................................................................................................... 5800/9566
2020-02-02T23:49:49.1389113Z .................................................................................................... 5800/9566
2020-02-02T23:49:56.2573460Z .................................................................................................... 5900/9566
2020-02-02T23:50:04.8894712Z ......................................................ii...i..ii...........i........................ 6000/9566
2020-02-02T23:50:26.6066744Z .................................................................................................... 6200/9566
2020-02-02T23:50:33.9797768Z .................................................................................................... 6300/9566
2020-02-02T23:50:39.2411335Z ..................................................................................i...ii............ 6400/9566
2020-02-02T23:50:51.7648015Z .................................................................................................... 6500/9566
---
2020-02-02T23:52:49.3574650Z .................................................................................................... 7600/9566
2020-02-02T23:52:54.0261335Z .................................................................................................... 7700/9566
2020-02-02T23:53:00.1343510Z .................................................................................................... 7800/9566
2020-02-02T23:53:10.4640586Z .................................................................................................... 7900/9566
2020-02-02T23:53:16.3081882Z ...................iiiiiii.i........................................................................ 8000/9566
2020-02-02T23:53:30.4160465Z .................................................................................................... 8200/9566
2020-02-02T23:53:39.0483174Z .................................................................................................... 8300/9566
2020-02-02T23:53:51.4057417Z .................................................................................................... 8400/9566
2020-02-02T23:53:57.9655758Z .................................................................................................... 8500/9566
---
2020-02-02T23:55:48.0496562Z 
2020-02-02T23:55:48.0497818Z ---- [ui] ui/consts/miri_unleashed/mutable_const2.rs stdout ----
2020-02-02T23:55:48.0498415Z diff of stderr:
2020-02-02T23:55:48.0498562Z 
2020-02-02T23:55:48.0498738Z 10 LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-02T23:55:48.0499036Z 12 
2020-02-02T23:55:48.0499036Z 12 
2020-02-02T23:55:48.0501488Z - thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:357:17
2020-02-02T23:55:48.0502111Z + thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', $SRC_DIR/librustc_errors/lib.rs:LL:COL
2020-02-02T23:55:48.0502710Z 15 
2020-02-02T23:55:48.0502858Z 16 error: internal compiler error: unexpected panic
2020-02-02T23:55:48.0503029Z 
2020-02-02T23:55:48.0503161Z 
2020-02-02T23:55:48.0503161Z 
2020-02-02T23:55:48.0503312Z The actual stderr differed from the expected stderr.
2020-02-02T23:55:48.0503802Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/mutable_const2.stderr
2020-02-02T23:55:48.0504215Z To update references, rerun the tests and pass the `--bless` flag
2020-02-02T23:55:48.0504697Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const2.rs`
2020-02-02T23:55:48.0505066Z error: 1 errors occurred comparing output.
2020-02-02T23:55:48.0505239Z status: exit code: 101
2020-02-02T23:55:48.0505239Z status: exit code: 101
2020-02-02T23:55:48.0506400Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/auxiliary" "-A" "unused"
2020-02-02T23:55:48.0507021Z ------------------------------------------
2020-02-02T23:55:48.0507185Z 
2020-02-02T23:55:48.0507565Z ------------------------------------------
2020-02-02T23:55:48.0507758Z stderr:
2020-02-02T23:55:48.0507758Z stderr:
2020-02-02T23:55:48.0508424Z ------------------------------------------
2020-02-02T23:55:48.0508621Z warning: skipping const checks
2020-02-02T23:55:48.0508973Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:38
2020-02-02T23:55:48.0509174Z    |
2020-02-02T23:55:48.0509324Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-02T23:55:48.0527150Z 
2020-02-02T23:55:48.0527368Z error: internal compiler error: mutable allocation in constant
2020-02-02T23:55:48.0528078Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:1
2020-02-02T23:55:48.0528317Z    |
2020-02-02T23:55:48.0528317Z    |
2020-02-02T23:55:48.0528479Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-02T23:55:48.0528983Z 
2020-02-02T23:55:48.0528983Z 
2020-02-02T23:55:48.0529432Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', /checkout/src/librustc_errors/lib.rs:357:17
2020-02-02T23:55:48.0530060Z 
2020-02-02T23:55:48.0530192Z error: internal compiler error: unexpected panic
2020-02-02T23:55:48.0530303Z 
2020-02-02T23:55:48.0530477Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-02T23:55:48.0530477Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-02T23:55:48.0530593Z 
2020-02-02T23:55:48.0531168Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-02T23:55:48.0531344Z 
2020-02-02T23:55:48.0534002Z note: rustc 1.42.0-nightly (3d9110bd0 2020-02-02) running on x86_64-unknown-linux-gnu
2020-02-02T23:55:48.0534225Z 
2020-02-02T23:55:48.0534720Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-02T23:55:48.0535233Z 
2020-02-02T23:55:48.0535912Z ------------------------------------------
2020-02-02T23:55:48.0536333Z 
2020-02-02T23:55:48.0536459Z 
2020-02-02T23:55:48.0536459Z 
2020-02-02T23:55:48.0536848Z ---- [ui] ui/consts/miri_unleashed/mutable_references_ice.rs stdout ----
2020-02-02T23:55:48.0537035Z diff of stderr:
2020-02-02T23:55:48.0537159Z 
2020-02-02T23:55:48.0537318Z 6 
2020-02-02T23:55:48.0537692Z 7 thread 'rustc' panicked at 'assertion failed: `(left != right)`
2020-02-02T23:55:48.0537877Z 8   left: `Const`,
2020-02-02T23:55:48.0538466Z -  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', src/librustc_mir/interpret/intern.rs:LL:CC
2020-02-02T23:55:48.0541921Z +  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', $SRC_DIR/librustc_mir/interpret/intern.rs:LL:COL
2020-02-02T23:55:48.0542746Z 11 
2020-02-02T23:55:48.0543110Z 12 error: internal compiler error: unexpected panic
2020-02-02T23:55:48.0543542Z 
2020-02-02T23:55:48.0543689Z 
2020-02-02T23:55:48.0543689Z 
2020-02-02T23:55:48.0543880Z The actual stderr differed from the expected stderr.
2020-02-02T23:55:48.0545895Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/mutable_references_ice.stderr
2020-02-02T23:55:48.0546763Z To update references, rerun the tests and pass the `--bless` flag
2020-02-02T23:55:48.0547409Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references_ice.rs`
2020-02-02T23:55:48.0547762Z error: 1 errors occurred comparing output.
2020-02-02T23:55:48.0547924Z status: exit code: 101
2020-02-02T23:55:48.0547924Z status: exit code: 101
2020-02-02T23:55:48.0548950Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/auxiliary" "-A" "unused"
2020-02-02T23:55:48.0549844Z ------------------------------------------
2020-02-02T23:55:48.0550012Z 
2020-02-02T23:55:48.0550541Z ------------------------------------------
2020-02-02T23:55:48.0550750Z stderr:
2020-02-02T23:55:48.0550750Z stderr:
2020-02-02T23:55:48.0551091Z ------------------------------------------
2020-02-02T23:55:48.0551271Z warning: skipping const checks
2020-02-02T23:55:48.0551660Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs:22:8
2020-02-02T23:55:48.0551846Z    |
2020-02-02T23:55:48.0552006Z LL |     x: &UnsafeCell::new(42), //~ WARN: skipping const checks
2020-02-02T23:55:48.0552292Z 
2020-02-02T23:55:48.0552657Z thread 'rustc' panicked at 'assertion failed: `(left != right)`
2020-02-02T23:55:48.0552864Z   left: `Const`,
2020-02-02T23:55:48.0552864Z   left: `Const`,
2020-02-02T23:55:48.0553441Z  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', /checkout/src/librustc_mir/interpret/intern.rs:173:21
2020-02-02T23:55:48.0554533Z 
2020-02-02T23:55:48.0555009Z error: internal compiler error: unexpected panic
2020-02-02T23:55:48.0556105Z 
2020-02-02T23:55:48.0556180Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-02T23:55:48.0556180Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-02T23:55:48.0556206Z 
2020-02-02T23:55:48.0556657Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-02T23:55:48.0556695Z 
2020-02-02T23:55:48.0556955Z note: rustc 1.42.0-nightly (3d9110bd0 2020-02-02) running on x86_64-unknown-linux-gnu
2020-02-02T23:55:48.0557014Z 
2020-02-02T23:55:48.0557688Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-02T23:55:48.0557752Z 
2020-02-02T23:55:48.0557984Z ------------------------------------------
2020-02-02T23:55:48.0558025Z 
2020-02-02T23:55:48.0558048Z 
2020-02-02T23:55:48.0558048Z 
2020-02-02T23:55:48.0558270Z ---- [ui] ui/pattern/const-pat-ice.rs stdout ----
2020-02-02T23:55:48.0558336Z diff of stderr:
2020-02-02T23:55:48.0558363Z 
2020-02-02T23:55:48.0559025Z - thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir_build/hair/pattern/_match.rs:LL:CC
2020-02-02T23:55:48.0559553Z + thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', $SRC_DIR/librustc_mir_build/hair/pattern/_match.rs:LL:COL
2020-02-02T23:55:48.0559686Z 3 
2020-02-02T23:55:48.0559729Z 4 error: internal compiler error: unexpected panic
2020-02-02T23:55:48.0559778Z 
2020-02-02T23:55:48.0559804Z 
2020-02-02T23:55:48.0559804Z 
2020-02-02T23:55:48.0559846Z The actual stderr differed from the expected stderr.
2020-02-02T23:55:48.0560165Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/const-pat-ice.stderr
2020-02-02T23:55:48.0560466Z To update references, rerun the tests and pass the `--bless` flag
2020-02-02T23:55:48.0560778Z To only update this specific test, also pass `--test-args pattern/const-pat-ice.rs`
2020-02-02T23:55:48.0561053Z error: 1 errors occurred comparing output.
2020-02-02T23:55:48.0561097Z status: exit code: 101
2020-02-02T23:55:48.0561097Z status: exit code: 101
2020-02-02T23:55:48.0563009Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/const-pat-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/auxiliary" "-A" "unused"
2020-02-02T23:55:48.0563586Z ------------------------------------------
2020-02-02T23:55:48.0563660Z 
2020-02-02T23:55:48.0563936Z ------------------------------------------
2020-02-02T23:55:48.0563987Z stderr:
2020-02-02T23:55:48.0563987Z stderr:
2020-02-02T23:55:48.0564229Z ------------------------------------------
2020-02-02T23:55:48.0564638Z thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', /checkout/src/librustc_mir_build/hair/pattern/_match.rs:1647:5
2020-02-02T23:55:48.0564747Z 
2020-02-02T23:55:48.0564818Z error: internal compiler error: unexpected panic
2020-02-02T23:55:48.0564850Z 
2020-02-02T23:55:48.0564897Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-02T23:55:48.0564897Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-02T23:55:48.0564928Z 
2020-02-02T23:55:48.0565325Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-02T23:55:48.0565369Z 
2020-02-02T23:55:48.0566171Z note: rustc 1.42.0-nightly (3d9110bd0 2020-02-02) running on x86_64-unknown-linux-gnu
2020-02-02T23:55:48.0566227Z 
2020-02-02T23:55:48.0566513Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-02T23:55:48.0566572Z 
2020-02-02T23:55:48.0566806Z ------------------------------------------
2020-02-02T23:55:48.0566836Z 
2020-02-02T23:55:48.0566859Z 
---
2020-02-02T23:55:48.0579968Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-02T23:55:48.0580077Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-02T23:55:48.0580111Z 
2020-02-02T23:55:48.0580140Z 
2020-02-02T23:55:48.0582159Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-02T23:55:48.0582440Z 
2020-02-02T23:55:48.0582494Z 
2020-02-02T23:55:48.0583802Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-02T23:55:48.0583865Z Build completed unsuccessfully in 0:58:49
2020-02-02T23:55:48.0583865Z Build completed unsuccessfully in 0:58:49
2020-02-02T23:55:48.0630325Z == clock drift check ==
2020-02-02T23:55:48.0646068Z   local time: Sun Feb  2 23:55:48 UTC 2020
2020-02-02T23:55:48.3559622Z   network time: Sun, 02 Feb 2020 23:55:48 GMT
2020-02-02T23:55:48.3567480Z == end clock drift check ==
2020-02-02T23:55:48.9041350Z 
2020-02-02T23:55:48.9130124Z ##[error]Bash exited with code '1'.
2020-02-02T23:55:48.9139140Z ##[section]Finishing: Run build
2020-02-02T23:55:48.9166756Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-02T23:55:48.9168706Z Task         : Get sources
2020-02-02T23:55:48.9168772Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T23:55:48.9168819Z Version      : 1.0.0
2020-02-02T23:55:48.9168861Z Author       : Microsoft
2020-02-02T23:55:48.9168861Z Author       : Microsoft
2020-02-02T23:55:48.9168929Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-02T23:55:48.9168981Z ==============================================================================
2020-02-02T23:55:49.3228747Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-02T23:55:49.3269988Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-02T23:55:49.3382181Z Cleaning up task key
2020-02-02T23:55:49.3382963Z Start cleaning up orphan processes.
2020-02-02T23:55:49.3488584Z Terminate orphan process: pid (4784) (python)
2020-02-02T23:55:49.3748340Z ##[section]Finishing: Finalize Job
