plain
2020-04-02T23:12:48.7135410Z ========================== Starting Command Output ===========================
2020-04-02T23:12:48.7137912Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4eddcd9e-8f7e-4686-bbb4-328d9185ab40.sh
2020-04-02T23:12:48.7138312Z 
2020-04-02T23:12:48.7141790Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-02T23:12:48.7159920Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-02T23:12:48.7162638Z Task         : Get sources
2020-04-02T23:12:48.7162892Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T23:12:48.7163136Z Version      : 1.0.0
2020-04-02T23:12:48.7163315Z Author       : Microsoft
---
2020-04-02T23:12:50.0077954Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-02T23:12:50.0084378Z ##[command]git config gc.auto 0
2020-04-02T23:12:50.0087778Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-02T23:12:50.0090819Z ##[command]git config --get-all http.proxy
2020-04-02T23:12:50.0097667Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70721/merge:refs/remotes/pull/70721/merge
---
2020-04-02T23:14:53.3375739Z Looks like docker image is the same as before, not uploading
2020-04-02T23:14:59.3299645Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T23:14:59.3552706Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T23:14:59.3577699Z == clock drift check ==
2020-04-02T23:14:59.3595266Z   local time: Thu Apr  2 23:14:59 UTC 2020
2020-04-02T23:14:59.6502532Z   network time: Thu, 02 Apr 2020 23:14:59 GMT
2020-04-02T23:14:59.6534639Z Starting sccache server...
2020-04-02T23:14:59.7196973Z configure: processing command line
2020-04-02T23:14:59.7197568Z configure: 
2020-04-02T23:14:59.7198609Z configure: rust.dist-src        := False
---
2020-04-02T23:19:14.9428558Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T23:19:16.1478922Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T23:19:17.4765129Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T23:19:18.3630813Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T23:19:25.9083925Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T23:19:27.6497886Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T23:19:31.4833782Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T23:19:34.8210322Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T23:19:42.4393328Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T23:38:17.5652556Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T23:38:19.1624456Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T23:38:20.9896616Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T23:38:22.2658887Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T23:38:32.2372987Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T23:38:34.3550807Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T23:38:39.1378306Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T23:38:43.9557274Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T23:38:54.2773533Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-03T00:01:55.1309717Z .................................................................................................... 1700/9870
2020-04-03T00:01:58.7908019Z .................................................................................................... 1800/9870
2020-04-03T00:02:06.7507518Z ...............................................................................................i.... 1900/9870
2020-04-03T00:02:13.7197810Z .................................................................................................... 2000/9870
2020-04-03T00:02:19.7048460Z .....................................................................................iiiii.......... 2100/9870
2020-04-03T00:02:40.1928324Z .................................................................................................... 2300/9870
2020-04-03T00:02:42.2403296Z .................................................................................................... 2400/9870
2020-04-03T00:02:44.3942212Z .................................................................................................... 2500/9870
2020-04-03T00:02:50.1178848Z .................................................................................................... 2600/9870
---
2020-04-03T00:05:33.3493989Z ...........................................................i...............i........................ 5000/9870
2020-04-03T00:05:40.4203875Z .................................................................................................... 5100/9870
2020-04-03T00:05:47.7283320Z .................................................................................................... 5200/9870
2020-04-03T00:05:52.5326947Z ....i............................................................................................... 5300/9870
2020-04-03T00:06:02.1458188Z ..........................................................................................ii.ii..... 5400/9870
2020-04-03T00:06:06.0255441Z ...i...i............................................................................................ 5500/9870
2020-04-03T00:06:14.0359349Z ...................................i................................................................ 5700/9870
2020-04-03T00:06:23.1071620Z .......................................................ii....................................i...... 5800/9870
2020-04-03T00:06:30.0595656Z .................................................................................................... 5900/9870
2020-04-03T00:06:34.7977301Z .................................................................................................... 6000/9870
2020-04-03T00:06:34.7977301Z .................................................................................................... 6000/9870
2020-04-03T00:06:43.5867433Z .......................................................................................ii...i..ii... 6100/9870
2020-04-03T00:07:02.8659185Z .................................................................................................... 6300/9870
2020-04-03T00:07:07.3389724Z .................................................................................................... 6400/9870
2020-04-03T00:07:10.0139579Z .................................................................................................... 6500/9870
2020-04-03T00:07:10.0139579Z .................................................................................................... 6500/9870
2020-04-03T00:07:21.5226335Z .................i..ii.............................................................................. 6600/9870
2020-04-03T00:07:40.6460437Z .................................................................................................... 6800/9870
2020-04-03T00:07:42.6533830Z .................i.................................................................................. 6900/9870
2020-04-03T00:07:44.6186363Z .................................................................................................... 7000/9870
2020-04-03T00:07:46.6969356Z ........................................................i........................................... 7100/9870
---
2020-04-03T00:09:17.4108680Z .................................................................................................... 7800/9870
2020-04-03T00:09:21.4468218Z .................................................................................................... 7900/9870
2020-04-03T00:09:26.7783885Z .................................................................................................... 8000/9870
2020-04-03T00:09:34.2841537Z ....................i............................................................................... 8100/9870
2020-04-03T00:09:42.0208622Z .....................................................................iiiiiiiiii.i................... 8200/9870
2020-04-03T00:09:56.3871365Z .............i......i............................................................................... 8400/9870
2020-04-03T00:10:00.7717910Z .................................................................................................... 8500/9870
2020-04-03T00:10:11.2018339Z .................................................................................................... 8600/9870
2020-04-03T00:10:22.0178269Z .................................................................................................... 8700/9870
---
2020-04-03T00:12:34.5597127Z ---- [mir-opt] mir-opt/simplify_try.rs stdout ----
2020-04-03T00:12:34.5598776Z 21           debug err => _6;                 // in scope 2 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-03T00:12:34.5599142Z 22           scope 3 {
2020-04-03T00:12:34.5599348Z 23               scope 7 {
2020-04-03T00:12:34.5600009Z -                   debug t => _6;           // in scope 7 at $SRC_DIR/libcore/convert/mod.rs:LL:COL: 566:14
2020-04-03T00:12:34.5600549Z +                   debug t => _6;           // in scope 7 at $SRC_DIR/libcore/convert/mod.rs:LL:COL: 565:14
2020-04-03T00:12:34.5601110Z 26               scope 8 {
2020-04-03T00:12:34.5601110Z 26               scope 8 {
2020-04-03T00:12:34.5601474Z 27                   debug v => _6;           // in scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1555:20
2020-04-03T00:12:34.5601772Z 
2020-04-03T00:12:34.5602661Z thread '[mir-opt] mir-opt/simplify_try.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_try/rustc.try_identity.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3141:25
2020-04-03T00:12:34.5603594Z 
2020-04-03T00:12:34.5603694Z 
2020-04-03T00:12:34.5603843Z failures:
2020-04-03T00:12:34.5604216Z     [mir-opt] mir-opt/simplify_try.rs
2020-04-03T00:12:34.5604216Z     [mir-opt] mir-opt/simplify_try.rs
2020-04-03T00:12:34.5604381Z 
2020-04-03T00:12:34.5604880Z test result: FAILED. 87 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-03T00:12:34.5605166Z 
2020-04-03T00:12:34.5605652Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-03T00:12:34.5610402Z 
2020-04-03T00:12:34.5610550Z 
2020-04-03T00:12:34.5614617Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-03T00:12:34.5617289Z 
2020-04-03T00:12:34.5617385Z 
2020-04-03T00:12:34.5617873Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-03T00:12:34.5618226Z Build completed unsuccessfully in 0:56:14
2020-04-03T00:12:34.5618226Z Build completed unsuccessfully in 0:56:14
2020-04-03T00:12:34.5654943Z == clock drift check ==
2020-04-03T00:12:34.5671212Z   local time: Fri Apr  3 00:12:34 UTC 2020
2020-04-03T00:12:34.7333207Z   network time: Fri, 03 Apr 2020 00:12:34 GMT
2020-04-03T00:12:36.8669887Z 
2020-04-03T00:12:36.8669887Z 
2020-04-03T00:12:36.8745667Z ##[error]Bash exited with code '1'.
2020-04-03T00:12:36.8758100Z ##[section]Finishing: Run build
2020-04-03T00:12:36.8802718Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-03T00:12:36.8806954Z Task         : Get sources
2020-04-03T00:12:36.8807241Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T00:12:36.8807516Z Version      : 1.0.0
2020-04-03T00:12:36.8807700Z Author       : Microsoft
2020-04-03T00:12:36.8807700Z Author       : Microsoft
2020-04-03T00:12:36.8807991Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-03T00:12:36.8808338Z ==============================================================================
2020-04-03T00:12:37.1856825Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-03T00:12:37.1905355Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-03T00:12:37.1989067Z Cleaning up task key
2020-04-03T00:12:37.1990215Z Start cleaning up orphan processes.
2020-04-03T00:12:37.2271869Z Terminate orphan process: pid (4107) (python)
2020-04-03T00:12:37.2425044Z ##[section]Finishing: Finalize Job
