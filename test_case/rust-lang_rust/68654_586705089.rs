plain
2020-02-16T12:05:42.7284869Z ========================== Starting Command Output ===========================
2020-02-16T12:05:42.7286609Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5868f3fc-f19b-47b6-aed3-86b3250fe4c1.sh
2020-02-16T12:05:42.7286649Z 
2020-02-16T12:05:42.7289435Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T12:05:42.7296195Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-16T12:05:42.7297890Z Task         : Get sources
2020-02-16T12:05:42.7297924Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T12:05:42.7298009Z Version      : 1.0.0
2020-02-16T12:05:42.7298042Z Author       : Microsoft
---
2020-02-16T12:05:45.1005688Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T12:05:45.1251733Z ##[command]git config gc.auto 0
2020-02-16T12:05:45.1315126Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T12:05:45.1365268Z ##[command]git config --get-all http.proxy
2020-02-16T12:05:45.1502642Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68654/merge:refs/remotes/pull/68654/merge
---
2020-02-16T13:04:42.6124607Z .................................................................................................... 1700/9650
2020-02-16T13:04:47.2137834Z ..........................................F.F....................................................... 1800/9650
2020-02-16T13:04:58.5824074Z ..................................i................................................................. 1900/9650
2020-02-16T13:05:05.9394861Z .................................................................................................... 2000/9650
2020-02-16T13:05:19.9183666Z ........................iiiii....................................................................... 2100/9650
2020-02-16T13:05:29.3621584Z .................................................................................................... 2300/9650
2020-02-16T13:05:31.7396217Z .................................................................................................... 2400/9650
2020-02-16T13:05:36.2002416Z .................................................................................................... 2500/9650
2020-02-16T13:05:57.0998486Z .................................................................................................... 2600/9650
---
2020-02-16T13:09:13.7820553Z .................................................................................................... 5600/9650
2020-02-16T13:09:24.3398728Z .......................................................................................i............ 5700/9650
2020-02-16T13:09:31.9438751Z .................................................................................................... 5800/9650
2020-02-16T13:09:36.9768661Z .....................................................................................i.............. 5900/9650
2020-02-16T13:09:46.5084972Z ...............................................................................ii...i..ii........... 6000/9650
2020-02-16T13:09:58.5732267Z i................................................................................................... 6100/9650
2020-02-16T13:10:14.7459408Z .................................................................................................... 6300/9650
2020-02-16T13:10:22.3228966Z .................................................................................................... 6400/9650
2020-02-16T13:10:22.3228966Z .................................................................................................... 6400/9650
2020-02-16T13:10:35.4162293Z .......i..ii........................................................................................ 6500/9650
2020-02-16T13:10:54.6769516Z ...............................................................................................i.... 6700/9650
2020-02-16T13:10:56.8482615Z .................................................................................................... 6800/9650
2020-02-16T13:10:59.0389582Z .................................................................................................... 6900/9650
2020-02-16T13:11:01.4526337Z .....i.............................................................................................. 7000/9650
---
2020-02-16T13:12:35.4494414Z .................................................................................................... 7600/9650
2020-02-16T13:12:40.0430532Z .................................................................................................... 7700/9650
2020-02-16T13:12:45.9865205Z .................................................................................................... 7800/9650
2020-02-16T13:12:52.5166100Z .................................................................................................... 7900/9650
2020-02-16T13:13:01.8235657Z .......................................................................................iiiiiii.i.... 8000/9650
2020-02-16T13:13:17.5142843Z ...........................i......i................................................................. 8200/9650
2020-02-16T13:13:22.3708531Z .................................................................................................... 8300/9650
2020-02-16T13:13:33.5368818Z .................................................................................................... 8400/9650
2020-02-16T13:13:44.9325456Z .................................................................................................... 8500/9650
---
2020-02-16T13:15:41.7972207Z 
2020-02-16T13:15:41.7973206Z ---- [ui] ui/consts/miri_unleashed/mutable_const2.rs stdout ----
2020-02-16T13:15:41.7973544Z diff of stderr:
2020-02-16T13:15:41.7973764Z 
2020-02-16T13:15:41.7974012Z 10 LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-16T13:15:41.7977789Z 12 
2020-02-16T13:15:41.7977789Z 12 
2020-02-16T13:15:41.7978805Z - thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', $SRC_DIR/librustc_errors/lib.rs:LL:CC
2020-02-16T13:15:41.7979482Z + thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', $SRC_DIR/librustc_errors/lib.rs:LL:COL
2020-02-16T13:15:41.7980061Z 15 
2020-02-16T13:15:41.7980296Z 16 error: internal compiler error: unexpected panic
2020-02-16T13:15:41.7980541Z 
2020-02-16T13:15:41.7980738Z 
2020-02-16T13:15:41.7980738Z 
2020-02-16T13:15:41.7980974Z The actual stderr differed from the expected stderr.
2020-02-16T13:15:41.7981572Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/mutable_const2.stderr
2020-02-16T13:15:41.7982141Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T13:15:41.7985284Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const2.rs`
2020-02-16T13:15:41.7986085Z error: 1 errors occurred comparing output.
2020-02-16T13:15:41.7986313Z status: exit code: 101
2020-02-16T13:15:41.7986313Z status: exit code: 101
2020-02-16T13:15:41.7987526Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/auxiliary" "-A" "unused"
2020-02-16T13:15:41.7988403Z ------------------------------------------
2020-02-16T13:15:41.7988667Z 
2020-02-16T13:15:41.7989169Z ------------------------------------------
2020-02-16T13:15:41.7989455Z stderr:
2020-02-16T13:15:41.7989455Z stderr:
2020-02-16T13:15:41.7989920Z ------------------------------------------
2020-02-16T13:15:41.7990229Z warning: skipping const checks
2020-02-16T13:15:41.7990733Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:38
2020-02-16T13:15:41.7991039Z    |
2020-02-16T13:15:41.7991288Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-16T13:15:41.7991758Z 
2020-02-16T13:15:41.7991993Z error: internal compiler error: mutable allocation in constant
2020-02-16T13:15:41.7992500Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:1
2020-02-16T13:15:41.7992800Z    |
2020-02-16T13:15:41.7992800Z    |
2020-02-16T13:15:41.7993034Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-02-16T13:15:41.7995577Z 
2020-02-16T13:15:41.7995577Z 
2020-02-16T13:15:41.7996238Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', /checkout/src/librustc_errors/lib.rs:355:17
2020-02-16T13:15:41.7996807Z 
2020-02-16T13:15:41.7997169Z error: internal compiler error: unexpected panic
2020-02-16T13:15:41.7997429Z 
2020-02-16T13:15:41.7997669Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-16T13:15:41.7997669Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-16T13:15:41.7998000Z 
2020-02-16T13:15:41.7998706Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-16T13:15:41.7999583Z note: rustc 1.43.0-nightly (9713b74f5 2020-02-16) running on x86_64-unknown-linux-gnu
2020-02-16T13:15:41.7999853Z 
2020-02-16T13:15:41.7999853Z 
2020-02-16T13:15:41.8000489Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-16T13:15:41.8002235Z 
2020-02-16T13:15:41.8002793Z ------------------------------------------
2020-02-16T13:15:41.8002971Z 
2020-02-16T13:15:41.8003096Z 
2020-02-16T13:15:41.8003096Z 
2020-02-16T13:15:41.8003526Z ---- [ui] ui/consts/miri_unleashed/mutable_references_ice.rs stdout ----
2020-02-16T13:15:41.8003729Z diff of stderr:
2020-02-16T13:15:41.8003860Z 
2020-02-16T13:15:41.8003995Z 6 
2020-02-16T13:15:41.8004417Z 7 thread 'rustc' panicked at 'assertion failed: `(left != right)`
2020-02-16T13:15:41.8004622Z 8   left: `Const`,
2020-02-16T13:15:41.8005292Z -  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', $SRC_DIR/librustc_mir/interpret/intern.rs:LL:CC
2020-02-16T13:15:41.8006006Z +  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', $SRC_DIR/librustc_mir/interpret/intern.rs:LL:COL
2020-02-16T13:15:41.8006512Z 11 
2020-02-16T13:15:41.8006683Z 12 error: internal compiler error: unexpected panic
2020-02-16T13:15:41.8006810Z 
2020-02-16T13:15:41.8006941Z 
2020-02-16T13:15:41.8006941Z 
2020-02-16T13:15:41.8007082Z The actual stderr differed from the expected stderr.
2020-02-16T13:15:41.8007608Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/mutable_references_ice.stderr
2020-02-16T13:15:41.8008051Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T13:15:41.8008546Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references_ice.rs`
2020-02-16T13:15:41.8008857Z error: 1 errors occurred comparing output.
2020-02-16T13:15:41.8009017Z status: exit code: 101
2020-02-16T13:15:41.8009017Z status: exit code: 101
2020-02-16T13:15:41.8010072Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_ice/auxiliary" "-A" "unused"
2020-02-16T13:15:41.8010716Z ------------------------------------------
2020-02-16T13:15:41.8010901Z 
2020-02-16T13:15:41.8011293Z ------------------------------------------
2020-02-16T13:15:41.8011475Z stderr:
2020-02-16T13:15:41.8011475Z stderr:
2020-02-16T13:15:41.8011861Z ------------------------------------------
2020-02-16T13:15:41.8012043Z warning: skipping const checks
2020-02-16T13:15:41.8012585Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_ice.rs:22:8
2020-02-16T13:15:41.8012836Z    |
2020-02-16T13:15:41.8012990Z LL |     x: &UnsafeCell::new(42), //~ WARN: skipping const checks
2020-02-16T13:15:41.8013396Z 
2020-02-16T13:15:41.8013822Z thread 'rustc' panicked at 'assertion failed: `(left != right)`
2020-02-16T13:15:41.8014031Z   left: `Const`,
2020-02-16T13:15:41.8014031Z   left: `Const`,
2020-02-16T13:15:41.8014687Z  right: `Const`: UnsafeCells are not allowed behind references in constants. This should have been prevented statically by const qualification. If this were allowed one would be able to change a constant at one use site and other use sites could observe that mutation.', /checkout/src/librustc_mir/interpret/intern.rs:173:21
2020-02-16T13:15:41.8015060Z 
2020-02-16T13:15:41.8015203Z error: internal compiler error: unexpected panic
2020-02-16T13:15:41.8015324Z 
2020-02-16T13:15:41.8015495Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-16T13:15:41.8015495Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-16T13:15:41.8015619Z 
2020-02-16T13:15:41.8016134Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-16T13:15:41.8016761Z note: rustc 1.43.0-nightly (9713b74f5 2020-02-16) running on x86_64-unknown-linux-gnu
2020-02-16T13:15:41.8016947Z 
2020-02-16T13:15:41.8016947Z 
2020-02-16T13:15:41.8017443Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-16T13:15:41.8017737Z 
2020-02-16T13:15:41.8018120Z ------------------------------------------
2020-02-16T13:15:41.8018279Z 
2020-02-16T13:15:41.8018399Z 
2020-02-16T13:15:41.8018399Z 
2020-02-16T13:15:41.8018786Z ---- [ui] ui/pattern/const-pat-ice.rs stdout ----
2020-02-16T13:15:41.8018972Z diff of stderr:
2020-02-16T13:15:41.8019093Z 
2020-02-16T13:15:41.8019585Z - thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', $SRC_DIR/librustc_mir_build/hair/pattern/_match.rs:LL:CC
2020-02-16T13:15:41.8020102Z + thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', $SRC_DIR/librustc_mir_build/hair/pattern/_match.rs:LL:COL
2020-02-16T13:15:41.8020471Z 3 
2020-02-16T13:15:41.8020613Z 4 error: internal compiler error: unexpected panic
2020-02-16T13:15:41.8020733Z 
2020-02-16T13:15:41.8020866Z 
2020-02-16T13:15:41.8020866Z 
2020-02-16T13:15:41.8021008Z The actual stderr differed from the expected stderr.
2020-02-16T13:15:41.8021457Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/const-pat-ice.stderr
2020-02-16T13:15:41.8021909Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T13:15:41.8022351Z To only update this specific test, also pass `--test-args pattern/const-pat-ice.rs`
2020-02-16T13:15:41.8022704Z error: 1 errors occurred comparing output.
2020-02-16T13:15:41.8022851Z status: exit code: 101
2020-02-16T13:15:41.8022851Z status: exit code: 101
2020-02-16T13:15:41.8023897Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/const-pat-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/const-pat-ice/auxiliary" "-A" "unused"
2020-02-16T13:15:41.8024600Z ------------------------------------------
2020-02-16T13:15:41.8024768Z 
2020-02-16T13:15:41.8025134Z ------------------------------------------
2020-02-16T13:15:41.8025331Z stderr:
2020-02-16T13:15:41.8025331Z stderr:
2020-02-16T13:15:41.8025799Z ------------------------------------------
2020-02-16T13:15:41.8026369Z thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', /checkout/src/librustc_mir_build/hair/pattern/_match.rs:1647:5
2020-02-16T13:15:41.8026892Z 
2020-02-16T13:15:41.8027040Z error: internal compiler error: unexpected panic
2020-02-16T13:15:41.8027163Z 
2020-02-16T13:15:41.8027325Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-16T13:15:41.8027325Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-16T13:15:41.8027451Z 
2020-02-16T13:15:41.8027954Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-16T13:15:41.8028604Z note: rustc 1.43.0-nightly (9713b74f5 2020-02-16) running on x86_64-unknown-linux-gnu
2020-02-16T13:15:41.8028783Z 
2020-02-16T13:15:41.8028783Z 
2020-02-16T13:15:41.8029276Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-16T13:15:41.8029598Z 
2020-02-16T13:15:41.8029970Z ------------------------------------------
2020-02-16T13:15:41.8030156Z 
2020-02-16T13:15:41.8030295Z 
---
2020-02-16T13:15:41.8032488Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T13:15:41.8032688Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T13:15:41.8032817Z 
2020-02-16T13:15:41.8032936Z 
2020-02-16T13:15:41.8034615Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T13:15:41.8035090Z 
2020-02-16T13:15:41.8035207Z 
2020-02-16T13:15:41.8035364Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T13:15:41.8035510Z Build completed unsuccessfully in 1:03:28
2020-02-16T13:15:41.8035510Z Build completed unsuccessfully in 1:03:28
2020-02-16T13:15:41.8046873Z == clock drift check ==
2020-02-16T13:15:41.8067979Z   local time: Sun Feb 16 13:15:41 UTC 2020
2020-02-16T13:15:42.0957496Z   network time: Sun, 16 Feb 2020 13:15:42 GMT
2020-02-16T13:15:42.0960186Z == end clock drift check ==
2020-02-16T13:15:42.6022151Z 
2020-02-16T13:15:42.6115318Z ##[error]Bash exited with code '1'.
2020-02-16T13:15:42.6127344Z ##[section]Finishing: Run build
2020-02-16T13:15:42.6147601Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-16T13:15:42.6149564Z Task         : Get sources
2020-02-16T13:15:42.6149751Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T13:15:42.6149796Z Version      : 1.0.0
2020-02-16T13:15:42.6149857Z Author       : Microsoft
2020-02-16T13:15:42.6149857Z Author       : Microsoft
2020-02-16T13:15:42.6149903Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-16T13:15:42.6149952Z ==============================================================================
2020-02-16T13:15:43.0139639Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-16T13:15:43.0185763Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-02-16T13:15:43.0300373Z Cleaning up task key
2020-02-16T13:15:43.0301161Z Start cleaning up orphan processes.
2020-02-16T13:15:43.1082176Z Terminate orphan process: pid (4811) (python)
2020-02-16T13:15:43.1108643Z ##[section]Finishing: Finalize Job
