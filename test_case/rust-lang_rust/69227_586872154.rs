plain
2020-02-17T07:13:11.7713571Z ========================== Starting Command Output ===========================
2020-02-17T07:13:11.7716490Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cf25030b-1175-4e12-a72b-8850ef5f347f.sh
2020-02-17T07:13:11.7716558Z 
2020-02-17T07:13:11.7720775Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-17T07:13:11.7727347Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69227/merge to s
2020-02-17T07:13:11.7729117Z Task         : Get sources
2020-02-17T07:13:11.7729148Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-17T07:13:11.7729179Z Version      : 1.0.0
2020-02-17T07:13:11.7729228Z Author       : Microsoft
---
2020-02-17T07:13:12.6542690Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-17T07:13:12.6628831Z ##[command]git config gc.auto 0
2020-02-17T07:13:12.6708195Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-17T07:13:12.6763478Z ##[command]git config --get-all http.proxy
2020-02-17T07:13:12.6912707Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69227/merge:refs/remotes/pull/69227/merge
---
2020-02-17T08:14:59.7613931Z .................................................................................................... 1700/9650
2020-02-17T08:15:04.6064228Z ...........................................FF....................................................... 1800/9650
2020-02-17T08:15:16.8020464Z ..................................i................................................................. 1900/9650
2020-02-17T08:15:24.6200244Z .................................................................................................... 2000/9650
2020-02-17T08:15:39.4075463Z ........................iiiii....................................................................... 2100/9650
2020-02-17T08:15:49.3000593Z .................................................................................................... 2300/9650
2020-02-17T08:15:51.6904351Z .................................................................................................... 2400/9650
2020-02-17T08:15:56.3324062Z .................................................................................................... 2500/9650
2020-02-17T08:16:17.7497594Z .................................................................................................... 2600/9650
---
2020-02-17T08:19:45.1845267Z .................................................................................................... 5600/9650
2020-02-17T08:19:56.0868874Z .......................................................................................i............ 5700/9650
2020-02-17T08:20:04.2353288Z .................................................................................................... 5800/9650
2020-02-17T08:20:09.6031101Z .....................................................................................i.............. 5900/9650
2020-02-17T08:20:19.7622974Z ...............................................................................ii...i..ii........... 6000/9650
2020-02-17T08:20:32.5791458Z i................................................................................................... 6100/9650
2020-02-17T08:20:49.3424031Z .................................................................................................... 6300/9650
2020-02-17T08:20:57.1950847Z .................................................................................................... 6400/9650
2020-02-17T08:20:57.1950847Z .................................................................................................... 6400/9650
2020-02-17T08:21:13.3347252Z .......i..ii........................................................................................ 6500/9650
2020-02-17T08:21:33.9166739Z ...............................................................................................i.... 6700/9650
2020-02-17T08:21:36.1438885Z .................................................................................................... 6800/9650
2020-02-17T08:21:38.3379074Z .................................................................................................... 6900/9650
2020-02-17T08:21:40.7666183Z .....i.............................................................................................. 7000/9650
---
2020-02-17T08:23:20.0980065Z .................................................................................................... 7600/9650
2020-02-17T08:23:24.7497705Z .................................................................................................... 7700/9650
2020-02-17T08:23:30.9091214Z .................................................................................................... 7800/9650
2020-02-17T08:23:37.5756926Z .................................................................................................... 7900/9650
2020-02-17T08:23:47.3892685Z .......................................................................................iiiiiii.i.... 8000/9650
2020-02-17T08:24:03.8082551Z ...........................i......i................................................................. 8200/9650
2020-02-17T08:24:08.8176892Z .................................................................................................... 8300/9650
2020-02-17T08:24:20.6106662Z .................................................................................................... 8400/9650
2020-02-17T08:24:32.7587583Z .................................................................................................... 8500/9650
---
2020-02-17T08:26:34.9216027Z 
2020-02-17T08:26:34.9216236Z - warning: skipping const checks
2020-02-17T08:26:34.9216549Z -   --> $DIR/mutable_const2.rs:15:38
2020-02-17T08:26:34.9216707Z -    |
2020-02-17T08:26:34.9216945Z - LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-17T08:26:34.9217314Z - 
2020-02-17T08:26:34.9217528Z - error: internal compiler error: mutable allocation in constant
2020-02-17T08:26:34.9217715Z -   --> $DIR/mutable_const2.rs:15:1
2020-02-17T08:26:34.9217865Z -    |
2020-02-17T08:26:34.9217865Z -    |
2020-02-17T08:26:34.9218331Z - LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-17T08:26:34.9218933Z - 
2020-02-17T08:26:34.9218933Z - 
2020-02-17T08:26:34.9219385Z 13 thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
2020-02-17T08:26:34.9219516Z 15 
2020-02-17T08:26:34.9219541Z 
2020-02-17T08:26:34.9219599Z 22 note: rustc VERSION running on TARGET
2020-02-17T08:26:34.9219637Z 23 
2020-02-17T08:26:34.9219637Z 23 
2020-02-17T08:26:34.9219675Z 24 note: compiler flags: FLAGS
2020-02-17T08:26:34.9220251Z + 
2020-02-17T08:26:34.9220297Z + warning: skipping const checks
2020-02-17T08:26:34.9220531Z +   --> $DIR/mutable_const2.rs:15:38
2020-02-17T08:26:34.9220579Z +    |
2020-02-17T08:26:34.9220733Z + LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-17T08:26:34.9220848Z + 
2020-02-17T08:26:34.9220911Z + error: internal compiler error: mutable allocation in constant
2020-02-17T08:26:34.9221138Z +   --> $DIR/mutable_const2.rs:15:1
2020-02-17T08:26:34.9221186Z +    |
2020-02-17T08:26:34.9221186Z +    |
2020-02-17T08:26:34.9221349Z + LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-17T08:26:34.9221452Z 25 
2020-02-17T08:26:34.9221510Z 26 
2020-02-17T08:26:34.9221538Z 
2020-02-17T08:26:34.9221565Z 
2020-02-17T08:26:34.9221565Z 
2020-02-17T08:26:34.9221612Z The actual stderr differed from the expected stderr.
2020-02-17T08:26:34.9221961Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/mutable_const2.stderr
2020-02-17T08:26:34.9222223Z To update references, rerun the tests and pass the `--bless` flag
2020-02-17T08:26:34.9222776Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const2.rs`
2020-02-17T08:26:34.9222989Z error: 1 errors occurred comparing output.
2020-02-17T08:26:34.9223035Z status: exit code: 101
2020-02-17T08:26:34.9223035Z status: exit code: 101
2020-02-17T08:26:34.9223983Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/auxiliary"
2020-02-17T08:26:34.9224344Z ------------------------------------------
2020-02-17T08:26:34.9224379Z 
2020-02-17T08:26:34.9224616Z ------------------------------------------
2020-02-17T08:26:34.9224665Z stderr:
2020-02-17T08:26:34.9224665Z stderr:
2020-02-17T08:26:34.9224904Z ------------------------------------------
2020-02-17T08:26:34.9225216Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
2020-02-17T08:26:34.9225337Z 
2020-02-17T08:26:34.9225382Z error: internal compiler error: unexpected panic
2020-02-17T08:26:34.9225412Z 
2020-02-17T08:26:34.9225458Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-17T08:26:34.9225458Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-17T08:26:34.9225507Z 
2020-02-17T08:26:34.9225902Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-17T08:26:34.9225942Z 
2020-02-17T08:26:34.9226246Z note: rustc 1.43.0-nightly (a027f5f92 2020-02-17) running on x86_64-unknown-linux-gnu
2020-02-17T08:26:34.9226292Z 
2020-02-17T08:26:34.9226651Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-17T08:26:34.9226768Z warning: skipping const checks
2020-02-17T08:26:34.9227028Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:38
2020-02-17T08:26:34.9227077Z    |
2020-02-17T08:26:34.9227077Z    |
2020-02-17T08:26:34.9227149Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-17T08:26:34.9227235Z 
2020-02-17T08:26:34.9227302Z error: internal compiler error: mutable allocation in constant
2020-02-17T08:26:34.9227562Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:1
2020-02-17T08:26:34.9227611Z    |
2020-02-17T08:26:34.9227611Z    |
2020-02-17T08:26:34.9227689Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-17T08:26:34.9227787Z 
2020-02-17T08:26:34.9227814Z 
2020-02-17T08:26:34.9228057Z ------------------------------------------
2020-02-17T08:26:34.9228090Z 
---
2020-02-17T08:26:34.9229597Z -    |        ^^^^^^^^^^^^^^^^^^^^
2020-02-17T08:26:34.9229790Z - 
2020-02-17T08:26:34.9230035Z 7 thread 'rustc' panicked at 'assertion failed: `(left != right)`
2020-02-17T08:26:34.9230086Z 8   left: `Const`,
2020-02-17T08:26:34.9230706Z 9  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', src/librustc_mir/interpret/intern.rs:LL:CC
2020-02-17T08:26:34.9230876Z 18 note: rustc VERSION running on TARGET
2020-02-17T08:26:34.9230939Z 19 
2020-02-17T08:26:34.9230987Z 20 note: compiler flags: FLAGS
2020-02-17T08:26:34.9231029Z + 
---
2020-02-17T08:26:34.9231665Z 22 
2020-02-17T08:26:34.9231692Z 
2020-02-17T08:26:34.9231737Z 
2020-02-17T08:26:34.9231785Z The actual stderr differed from the expected stderr.
2020-02-17T08:26:34.9232137Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/mutable_references_ice.stderr
2020-02-17T08:26:34.9232419Z To update references, rerun the tests and pass the `--bless` flag
2020-02-17T08:26:34.9232713Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references_ice.rs`
2020-02-17T08:26:34.9232812Z error: 1 errors occurred comparing output.
2020-02-17T08:26:34.9232859Z status: exit code: 101
2020-02-17T08:26:34.9232859Z status: exit code: 101
2020-02-17T08:26:34.9233836Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/auxiliary"
2020-02-17T08:26:34.9234192Z ------------------------------------------
2020-02-17T08:26:34.9234243Z 
2020-02-17T08:26:34.9234465Z ------------------------------------------
2020-02-17T08:26:34.9234513Z stderr:
2020-02-17T08:26:34.9234513Z stderr:
2020-02-17T08:26:34.9234725Z ------------------------------------------
2020-02-17T08:26:34.9234988Z thread 'rustc' panicked at 'assertion failed: `(left != right)`
2020-02-17T08:26:34.9235042Z   left: `Const`,
2020-02-17T08:26:34.9235568Z  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', src/librustc_mir/interpret/intern.rs:173:21
2020-02-17T08:26:34.9235692Z 
2020-02-17T08:26:34.9235757Z error: internal compiler error: unexpected panic
2020-02-17T08:26:34.9235787Z 
2020-02-17T08:26:34.9235834Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-17T08:26:34.9235834Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-17T08:26:34.9235864Z 
2020-02-17T08:26:34.9236211Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-17T08:26:34.9236249Z 
2020-02-17T08:26:34.9236532Z note: rustc 1.43.0-nightly (a027f5f92 2020-02-17) running on x86_64-unknown-linux-gnu
2020-02-17T08:26:34.9236585Z 
2020-02-17T08:26:34.9236942Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-17T08:26:34.9237182Z warning: skipping const checks
2020-02-17T08:26:34.9237872Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs:22:8
2020-02-17T08:26:34.9237972Z    |
2020-02-17T08:26:34.9237972Z    |
2020-02-17T08:26:34.9238015Z LL |     x: &UnsafeCell::new(42), //~ WARN: skipping const checks
2020-02-17T08:26:34.9238102Z 
2020-02-17T08:26:34.9238126Z 
2020-02-17T08:26:34.9238516Z ------------------------------------------
2020-02-17T08:26:34.9238562Z 
---
2020-02-17T08:26:34.9242684Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-17T08:26:34.9242789Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-17T08:26:34.9253951Z 
2020-02-17T08:26:34.9254028Z 
2020-02-17T08:26:34.9260128Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-17T08:26:34.9260466Z 
2020-02-17T08:26:34.9260493Z 
2020-02-17T08:26:34.9270980Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-17T08:26:34.9271082Z Build completed unsuccessfully in 1:07:08
2020-02-17T08:26:34.9271082Z Build completed unsuccessfully in 1:07:08
2020-02-17T08:26:34.9410735Z == clock drift check ==
2020-02-17T08:26:34.9431091Z   local time: Mon Feb 17 08:26:34 UTC 2020
2020-02-17T08:26:35.2382407Z   network time: Mon, 17 Feb 2020 08:26:35 GMT
2020-02-17T08:26:35.2388799Z == end clock drift check ==
2020-02-17T08:26:35.6650845Z 
2020-02-17T08:26:35.6748817Z ##[error]Bash exited with code '1'.
2020-02-17T08:26:35.6761284Z ##[section]Finishing: Run build
2020-02-17T08:26:35.6783154Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69227/merge to s
2020-02-17T08:26:35.6784879Z Task         : Get sources
2020-02-17T08:26:35.6784923Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-17T08:26:35.6784966Z Version      : 1.0.0
2020-02-17T08:26:35.6785022Z Author       : Microsoft
2020-02-17T08:26:35.6785022Z Author       : Microsoft
2020-02-17T08:26:35.6785067Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-17T08:26:35.6785114Z ==============================================================================
2020-02-17T08:26:36.1406694Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-17T08:26:36.1450993Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69227/merge to s
2020-02-17T08:26:36.1562204Z Cleaning up task key
2020-02-17T08:26:36.1562967Z Start cleaning up orphan processes.
2020-02-17T08:26:36.1693585Z Terminate orphan process: pid (4567) (python)
2020-02-17T08:26:36.1926380Z ##[section]Finishing: Finalize Job
