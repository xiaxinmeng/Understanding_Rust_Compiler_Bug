plain
2020-02-11T20:50:13.6707124Z ========================== Starting Command Output ===========================
2020-02-11T20:50:13.6710667Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ffade32f-ac2d-442b-bd52-15c11db7560c.sh
2020-02-11T20:50:13.6711058Z 
2020-02-11T20:50:13.6713882Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T20:50:13.6719811Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-11T20:50:13.6721465Z Task         : Get sources
2020-02-11T20:50:13.6721502Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T20:50:13.6721534Z Version      : 1.0.0
2020-02-11T20:50:13.6721567Z Author       : Microsoft
---
2020-02-11T20:50:14.5492117Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T20:50:14.5633207Z ##[command]git config gc.auto 0
2020-02-11T20:50:14.5756273Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T20:50:14.5819902Z ##[command]git config --get-all http.proxy
2020-02-11T20:50:14.5957451Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68654/merge:refs/remotes/pull/68654/merge
---
2020-02-11T21:48:59.2585131Z .................................................................................................... 1700/9627
2020-02-11T21:49:04.3512869Z ......................................F.F........................................................... 1800/9627
2020-02-11T21:49:16.3646860Z ..............................i..................................................................... 1900/9627
2020-02-11T21:49:24.1666059Z .................................................................................................... 2000/9627
2020-02-11T21:49:38.8242921Z ....................iiiii........................................................................... 2100/9627
2020-02-11T21:49:48.9513515Z .................................................................................................... 2300/9627
2020-02-11T21:49:51.4952890Z .................................................................................................... 2400/9627
2020-02-11T21:49:56.3968838Z .................................................................................................... 2500/9627
2020-02-11T21:50:17.7444830Z .................................................................................................... 2600/9627
---
2020-02-11T21:52:53.4301442Z .......................................................................i...............i............ 4900/9627
2020-02-11T21:53:01.1800832Z .................................................................................................... 5000/9627
2020-02-11T21:53:09.7898834Z .................................................................................................... 5100/9627
2020-02-11T21:53:14.5125521Z .............i...................................................................................... 5200/9627
2020-02-11T21:53:25.6177288Z .......................................................................................ii.ii........ 5300/9627
2020-02-11T21:53:29.5281154Z i...i............................................................................................... 5400/9627
2020-02-11T21:53:41.6386357Z .................................................................................................... 5600/9627
2020-02-11T21:53:50.1809199Z ...........................................................................i........................ 5700/9627
2020-02-11T21:53:57.8216175Z .................................................................................................... 5800/9627
2020-02-11T21:54:04.4358372Z .................................................................................................... 5900/9627
2020-02-11T21:54:04.4358372Z .................................................................................................... 5900/9627
2020-02-11T21:54:14.5455976Z ...................................................................ii...i..ii...........i........... 6000/9627
2020-02-11T21:54:35.8468684Z .................................................................................................... 6200/9627
2020-02-11T21:54:43.2645643Z .................................................................................................... 6300/9627
2020-02-11T21:54:50.6241133Z ...............................................................................................i..ii 6400/9627
2020-02-11T21:55:03.8084941Z .................................................................................................... 6500/9627
---
2020-02-11T21:57:09.4481648Z .................................................................................................... 7600/9627
2020-02-11T21:57:13.6391347Z .................................................................................................... 7700/9627
2020-02-11T21:57:19.4727332Z .................................................................................................... 7800/9627
2020-02-11T21:57:28.0733423Z .................................................................................................... 7900/9627
2020-02-11T21:57:37.1447772Z .....................................................................iiiiiii.i...................... 8000/9627
2020-02-11T21:57:53.4757398Z .........i......i................................................................................... 8200/9627
2020-02-11T21:57:59.1344670Z .................................................................................................... 8300/9627
2020-02-11T21:58:13.3911496Z .................................................................................................... 8400/9627
2020-02-11T21:58:23.4521400Z .................................................................................................... 8500/9627
---
2020-02-11T22:00:22.5835420Z 
2020-02-11T22:00:22.5836236Z ---- [ui] ui/consts/miri_unleashed/mutable_const2.rs stdout ----
2020-02-11T22:00:22.5836311Z diff of stderr:
2020-02-11T22:00:22.5857664Z 
2020-02-11T22:00:22.5857800Z 10 LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-11T22:00:22.5857927Z 12 
2020-02-11T22:00:22.5857927Z 12 
2020-02-11T22:00:22.5858498Z - thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
2020-02-11T22:00:22.5858825Z + thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', $SRC_DIR/librustc_errors/lib.rs:LL:COL
2020-02-11T22:00:22.5858949Z 15 
2020-02-11T22:00:22.5858994Z 16 error: internal compiler error: unexpected panic
2020-02-11T22:00:22.5859039Z 
2020-02-11T22:00:22.5859067Z 
2020-02-11T22:00:22.5859067Z 
2020-02-11T22:00:22.5859112Z The actual stderr differed from the expected stderr.
2020-02-11T22:00:22.5859430Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/mutable_const2.stderr
2020-02-11T22:00:22.5859956Z To update references, rerun the tests and pass the `--bless` flag
2020-02-11T22:00:22.5860339Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const2.rs`
2020-02-11T22:00:22.5860444Z error: 1 errors occurred comparing output.
2020-02-11T22:00:22.5860490Z status: exit code: 101
2020-02-11T22:00:22.5860490Z status: exit code: 101
2020-02-11T22:00:22.5861391Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/auxiliary" "-A" "unused"
2020-02-11T22:00:22.5861742Z ------------------------------------------
2020-02-11T22:00:22.5861794Z 
2020-02-11T22:00:22.5862016Z ------------------------------------------
2020-02-11T22:00:22.5862062Z stderr:
2020-02-11T22:00:22.5862062Z stderr:
2020-02-11T22:00:22.5862271Z ------------------------------------------
2020-02-11T22:00:22.5862331Z warning: skipping const checks
2020-02-11T22:00:22.5862579Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:38
2020-02-11T22:00:22.5862630Z    |
2020-02-11T22:00:22.5862693Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-11T22:00:22.5862777Z 
2020-02-11T22:00:22.5862833Z error: internal compiler error: mutable allocation in constant
2020-02-11T22:00:22.5863097Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:1
2020-02-11T22:00:22.5863144Z    |
2020-02-11T22:00:22.5863144Z    |
2020-02-11T22:00:22.5863205Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-11T22:00:22.5863300Z 
2020-02-11T22:00:22.5863300Z 
2020-02-11T22:00:22.5863603Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', /checkout/src/librustc_errors/lib.rs:355:17
2020-02-11T22:00:22.5863710Z 
2020-02-11T22:00:22.5863754Z error: internal compiler error: unexpected panic
2020-02-11T22:00:22.5863792Z 
2020-02-11T22:00:22.5863837Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-11T22:00:22.5863837Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-11T22:00:22.5863867Z 
2020-02-11T22:00:22.5864287Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-11T22:00:22.5864615Z note: rustc 1.43.0-nightly (10999e967 2020-02-11) running on x86_64-unknown-linux-gnu
2020-02-11T22:00:22.5864651Z 
2020-02-11T22:00:22.5864651Z 
2020-02-11T22:00:22.5865009Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-11T22:00:22.5865079Z 
2020-02-11T22:00:22.5865292Z ------------------------------------------
2020-02-11T22:00:22.5865335Z 
2020-02-11T22:00:22.5865362Z 
2020-02-11T22:00:22.5865362Z 
2020-02-11T22:00:22.5865604Z ---- [ui] ui/consts/miri_unleashed/mutable_references_ice.rs stdout ----
2020-02-11T22:00:22.5865654Z diff of stderr:
2020-02-11T22:00:22.5865699Z 
2020-02-11T22:00:22.5865737Z 6 
2020-02-11T22:00:22.5865976Z 7 thread 'rustc' panicked at 'assertion failed: `(left != right)`
2020-02-11T22:00:22.5866038Z 8   left: `Const`,
2020-02-11T22:00:22.5866520Z -  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', src/librustc_mir/interpret/intern.rs:LL:CC
2020-02-11T22:00:22.5867242Z +  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', $SRC_DIR/librustc_mir/interpret/intern.rs:LL:COL
2020-02-11T22:00:22.5867390Z 11 
2020-02-11T22:00:22.5867436Z 12 error: internal compiler error: unexpected panic
2020-02-11T22:00:22.5867467Z 
2020-02-11T22:00:22.5867492Z 
2020-02-11T22:00:22.5867492Z 
2020-02-11T22:00:22.5867549Z The actual stderr differed from the expected stderr.
2020-02-11T22:00:22.5867907Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/mutable_references_ice.stderr
2020-02-11T22:00:22.5868165Z To update references, rerun the tests and pass the `--bless` flag
2020-02-11T22:00:22.5868467Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references_ice.rs`
2020-02-11T22:00:22.5868548Z error: 1 errors occurred comparing output.
2020-02-11T22:00:22.5868592Z status: exit code: 101
2020-02-11T22:00:22.5868592Z status: exit code: 101
2020-02-11T22:00:22.5869488Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/auxiliary" "-A" "unused"
2020-02-11T22:00:22.5869828Z ------------------------------------------
2020-02-11T22:00:22.5869862Z 
2020-02-11T22:00:22.5870087Z ------------------------------------------
2020-02-11T22:00:22.5870133Z stderr:
2020-02-11T22:00:22.5870133Z stderr:
2020-02-11T22:00:22.5870341Z ------------------------------------------
2020-02-11T22:00:22.5870388Z warning: skipping const checks
2020-02-11T22:00:22.5870651Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs:22:8
2020-02-11T22:00:22.5870701Z    |
2020-02-11T22:00:22.5870749Z LL |     x: &UnsafeCell::new(42), //~ WARN: skipping const checks
2020-02-11T22:00:22.5870835Z 
2020-02-11T22:00:22.5871069Z thread 'rustc' panicked at 'assertion failed: `(left != right)`
2020-02-11T22:00:22.5871138Z   left: `Const`,
2020-02-11T22:00:22.5871138Z   left: `Const`,
2020-02-11T22:00:22.5871639Z  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', /checkout/src/librustc_mir/interpret/intern.rs:173:21
2020-02-11T22:00:22.5871766Z 
2020-02-11T22:00:22.5871809Z error: internal compiler error: unexpected panic
2020-02-11T22:00:22.5871837Z 
2020-02-11T22:00:22.5871893Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-11T22:00:22.5871893Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-11T22:00:22.5871922Z 
2020-02-11T22:00:22.5872240Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-11T22:00:22.5872561Z note: rustc 1.43.0-nightly (10999e967 2020-02-11) running on x86_64-unknown-linux-gnu
2020-02-11T22:00:22.5872686Z 
2020-02-11T22:00:22.5872686Z 
2020-02-11T22:00:22.5873052Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-11T22:00:22.5873200Z 
2020-02-11T22:00:22.5873441Z ------------------------------------------
2020-02-11T22:00:22.5873473Z 
2020-02-11T22:00:22.5873498Z 
2020-02-11T22:00:22.5873498Z 
2020-02-11T22:00:22.5873729Z ---- [ui] ui/pattern/const-pat-ice.rs stdout ----
2020-02-11T22:00:22.5873777Z diff of stderr:
2020-02-11T22:00:22.5873804Z 
2020-02-11T22:00:22.5874111Z - thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir_build/hair/pattern/_match.rs:LL:CC
2020-02-11T22:00:22.5874450Z + thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', $SRC_DIR/librustc_mir_build/hair/pattern/_match.rs:LL:COL
2020-02-11T22:00:22.5874578Z 3 
2020-02-11T22:00:22.5874622Z 4 error: internal compiler error: unexpected panic
2020-02-11T22:00:22.5874653Z 
2020-02-11T22:00:22.5874678Z 
2020-02-11T22:00:22.5874678Z 
2020-02-11T22:00:22.5874721Z The actual stderr differed from the expected stderr.
2020-02-11T22:00:22.5875038Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/const-pat-ice.stderr
2020-02-11T22:00:22.5875282Z To update references, rerun the tests and pass the `--bless` flag
2020-02-11T22:00:22.5875543Z To only update this specific test, also pass `--test-args pattern/const-pat-ice.rs`
2020-02-11T22:00:22.5875622Z error: 1 errors occurred comparing output.
2020-02-11T22:00:22.5875666Z status: exit code: 101
2020-02-11T22:00:22.5875666Z status: exit code: 101
2020-02-11T22:00:22.5876466Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/const-pat-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/auxiliary" "-A" "unused"
2020-02-11T22:00:22.5876792Z ------------------------------------------
2020-02-11T22:00:22.5876826Z 
2020-02-11T22:00:22.5877040Z ------------------------------------------
2020-02-11T22:00:22.5877098Z stderr:
2020-02-11T22:00:22.5877098Z stderr:
2020-02-11T22:00:22.5877307Z ------------------------------------------
2020-02-11T22:00:22.5877632Z thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', /checkout/src/librustc_mir_build/hair/pattern/_match.rs:1647:5
2020-02-11T22:00:22.5877750Z 
2020-02-11T22:00:22.5877794Z error: internal compiler error: unexpected panic
2020-02-11T22:00:22.5877824Z 
2020-02-11T22:00:22.5877880Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-11T22:00:22.5877880Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-11T22:00:22.5877910Z 
2020-02-11T22:00:22.5878232Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-11T22:00:22.5878556Z note: rustc 1.43.0-nightly (10999e967 2020-02-11) running on x86_64-unknown-linux-gnu
2020-02-11T22:00:22.5878592Z 
2020-02-11T22:00:22.5878592Z 
2020-02-11T22:00:22.5878899Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-11T22:00:22.5878974Z 
2020-02-11T22:00:22.5879184Z ------------------------------------------
2020-02-11T22:00:22.5879215Z 
2020-02-11T22:00:22.5879241Z 
---
2020-02-11T22:00:22.5895823Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-11T22:00:22.5895900Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-11T22:00:22.5895956Z 
2020-02-11T22:00:22.5895985Z 
2020-02-11T22:00:22.5897491Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-11T22:00:22.5897740Z 
2020-02-11T22:00:22.5897770Z 
2020-02-11T22:00:22.5897816Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-11T22:00:22.5897882Z Build completed unsuccessfully in 1:04:09
2020-02-11T22:00:22.5897882Z Build completed unsuccessfully in 1:04:09
2020-02-11T22:00:22.5946816Z == clock drift check ==
2020-02-11T22:00:22.5982104Z   local time: Tue Feb 11 22:00:22 UTC 2020
2020-02-11T22:00:23.1457593Z   network time: Tue, 11 Feb 2020 22:00:23 GMT
2020-02-11T22:00:23.1460838Z == end clock drift check ==
2020-02-11T22:00:23.5615233Z 
2020-02-11T22:00:23.5736405Z ##[error]Bash exited with code '1'.
2020-02-11T22:00:23.5749787Z ##[section]Finishing: Run build
2020-02-11T22:00:23.5770396Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-11T22:00:23.5772359Z Task         : Get sources
2020-02-11T22:00:23.5772425Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T22:00:23.5772484Z Version      : 1.0.0
2020-02-11T22:00:23.5772525Z Author       : Microsoft
2020-02-11T22:00:23.5772525Z Author       : Microsoft
2020-02-11T22:00:23.5772590Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-11T22:00:23.5772639Z ==============================================================================
2020-02-11T22:00:23.9985728Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-11T22:00:24.0017499Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-11T22:00:24.0137833Z Cleaning up task key
2020-02-11T22:00:24.0138932Z Start cleaning up orphan processes.
2020-02-11T22:00:24.0255586Z Terminate orphan process: pid (4017) (python)
2020-02-11T22:00:24.0498498Z ##[section]Finishing: Finalize Job
