plain
2020-04-07T10:53:07.7655249Z ========================== Starting Command Output ===========================
2020-04-07T10:53:07.7658338Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/13617fc4-f61e-45f2-aa86-d2682ab8e51d.sh
2020-04-07T10:53:07.7658642Z 
2020-04-07T10:53:07.7663148Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T10:53:07.7683985Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-07T10:53:07.7687811Z Task         : Get sources
2020-04-07T10:53:07.7688399Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T10:53:07.7688738Z Version      : 1.0.0
2020-04-07T10:53:07.7688951Z Author       : Microsoft
---
2020-04-07T10:53:08.7621174Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T10:53:08.7626876Z ##[command]git config gc.auto 0
2020-04-07T10:53:08.7630569Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T10:53:08.7634030Z ##[command]git config --get-all http.proxy
2020-04-07T10:53:08.7640139Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70721/merge:refs/remotes/pull/70721/merge
---
2020-04-07T10:55:09.9103679Z Looks like docker image is the same as before, not uploading
2020-04-07T10:55:17.1135348Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T10:55:17.1468472Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T10:55:17.1502684Z == clock drift check ==
2020-04-07T10:55:17.1509590Z   local time: Tue Apr  7 10:55:17 UTC 2020
2020-04-07T10:55:17.4428884Z   network time: Tue, 07 Apr 2020 10:55:17 GMT
2020-04-07T10:55:17.4450680Z Starting sccache server...
2020-04-07T10:55:17.5312382Z configure: processing command line
2020-04-07T10:55:17.5312653Z configure: 
2020-04-07T10:55:17.5313562Z configure: rust.dist-src        := False
---
2020-04-07T11:00:26.0135873Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T11:00:27.4821063Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T11:00:29.4035725Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T11:00:30.1961376Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T11:00:39.0336174Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T11:00:41.4953021Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T11:00:45.9239016Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T11:00:50.0850007Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T11:00:59.3099612Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T11:23:30.7466838Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T11:23:32.5393814Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T11:23:34.5777626Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T11:23:35.8951153Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T11:23:47.0813924Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T11:23:49.8523906Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T11:23:55.2927473Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T11:24:00.8593674Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T11:24:11.9083942Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T11:50:24.4082866Z .................................................................................................... 1700/9879
2020-04-07T11:50:28.5968424Z .................................................................................................... 1800/9879
2020-04-07T11:50:38.3673356Z ..................................................................................................i. 1900/9879
2020-04-07T11:50:46.8045112Z .................................................................................................... 2000/9879
2020-04-07T11:50:53.5087562Z ........................................................................................iiiii....... 2100/9879
2020-04-07T11:51:15.5076162Z .................................................................................................... 2300/9879
2020-04-07T11:51:17.8200625Z .................................................................................................... 2400/9879
2020-04-07T11:51:20.1622295Z .................................................................................................... 2500/9879
2020-04-07T11:51:26.6763201Z .................................................................................................... 2600/9879
---
2020-04-07T11:54:22.5578796Z ..............................................................i...............i..................... 5000/9879
2020-04-07T11:54:29.9309805Z .................................................................................................... 5100/9879
2020-04-07T11:54:37.4826325Z .................................................................................................... 5200/9879
2020-04-07T11:54:42.7933941Z .......i............................................................................................ 5300/9879
2020-04-07T11:54:52.7949605Z ................................................................................................ii.i 5400/9879
2020-04-07T11:54:57.7116957Z i........i...i...................................................................................... 5500/9879
2020-04-07T11:55:06.3208625Z .........................................i.......................................................... 5700/9879
2020-04-07T11:55:16.2857552Z .............................................................ii..................................... 5800/9879
2020-04-07T11:55:23.4866602Z i................................................................................................... 5900/9879
2020-04-07T11:55:28.6695152Z .................................................................................................... 6000/9879
2020-04-07T11:55:28.6695152Z .................................................................................................... 6000/9879
2020-04-07T11:55:38.5369590Z ..............................................................................................ii...i 6100/9879
2020-04-07T11:55:50.3692461Z ..ii...........i.................................................................................... 6200/9879
2020-04-07T11:56:05.7323738Z .................................................................................................... 6400/9879
2020-04-07T11:56:08.8526975Z .................................................................................................... 6500/9879
2020-04-07T11:56:08.8526975Z .................................................................................................... 6500/9879
2020-04-07T11:56:21.5187824Z ........................i..ii....................................................................... 6600/9879
2020-04-07T11:56:42.5573270Z .................................................................................................... 6800/9879
2020-04-07T11:56:44.6773125Z ........................i........................................................................... 6900/9879
2020-04-07T11:56:46.8281898Z .................................................................................................... 7000/9879
2020-04-07T11:56:49.0492122Z ...............................................................i.................................... 7100/9879
---
2020-04-07T11:58:29.1015537Z .................................................................................................... 7800/9879
2020-04-07T11:58:34.5095297Z .................................................................................................... 7900/9879
2020-04-07T11:58:40.1202246Z .................................................................................................... 8000/9879
2020-04-07T11:58:47.7865900Z ............................i....................................................................... 8100/9879
2020-04-07T11:58:56.5254142Z ............................................................................iiiiii.iiii.i........... 8200/9879
2020-04-07T11:59:12.9996841Z .....................i......i....................................................................... 8400/9879
2020-04-07T11:59:17.8980878Z .................................................................................................... 8500/9879
2020-04-07T11:59:28.8897489Z .................................................................................................... 8600/9879
2020-04-07T11:59:41.3688773Z .................................................................................................... 8700/9879
---
2020-04-07T12:01:59.4203895Z running 89 tests
2020-04-07T12:02:07.1433542Z ............................................................................F............
2020-04-07T12:02:07.1437697Z failures:
2020-04-07T12:02:07.1438219Z 
2020-04-07T12:02:07.1439153Z ---- [mir-opt] mir-opt/simplify_try.rs stdout ----
2020-04-07T12:02:07.1439801Z 24                   debug t => _6;           // in scope 7 at $SRC_DIR/libcore/convert/mod.rs:LL:COL: 566:14
2020-04-07T12:02:07.1440420Z 26               scope 8 {
2020-04-07T12:02:07.1440420Z 26               scope 8 {
2020-04-07T12:02:07.1441097Z -                   debug v => _6;           // in scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1557:20
2020-04-07T12:02:07.1441647Z +                   debug v => _6;           // in scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1555:20
2020-04-07T12:02:07.1442168Z 28                   let mut _12: i32;        // in scope 8 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-07T12:02:07.1442714Z 30           }
2020-04-07T12:02:07.1442844Z 
2020-04-07T12:02:07.1442991Z 35           }
2020-04-07T12:02:07.1443173Z 36       }
2020-04-07T12:02:07.1443173Z 36       }
2020-04-07T12:02:07.1443345Z 37       scope 6 {
2020-04-07T12:02:07.1443963Z -           debug self => _1;                // in scope 6 at $SRC_DIR/libcore/result.rs:LL:COL: 1547:24
2020-04-07T12:02:07.1444500Z +           debug self => _1;                // in scope 6 at $SRC_DIR/libcore/result.rs:LL:COL: 1545:24
2020-04-07T12:02:07.1444988Z 40   
2020-04-07T12:02:07.1445154Z 41       bb0: {
2020-04-07T12:02:07.1445308Z 
2020-04-07T12:02:07.1445467Z 55   
2020-04-07T12:02:07.1445467Z 55   
2020-04-07T12:02:07.1445629Z 56       bb2: {
2020-04-07T12:02:07.1446344Z 57 -         _6 = ((_1 as Err).0: i32);       // bb2[0]: scope 0 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-07T12:02:07.1447216Z - -         ((_0 as Err).0: i32) = move _6;  // bb2[1]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1558:15
2020-04-07T12:02:07.1448326Z - -         discriminant(_0) = 1;            // bb2[2]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1558:15
2020-04-07T12:02:07.1449206Z - +         _0 = move _1;                    // bb2[0]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1558:15
2020-04-07T12:02:07.1450044Z - +         nop;                             // bb2[1]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1558:15
2020-04-07T12:02:07.1450913Z - +         nop;                             // bb2[2]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1558:15
2020-04-07T12:02:07.1451772Z + -         ((_0 as Err).0: i32) = move _6;  // bb2[1]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1556:15
2020-04-07T12:02:07.1452628Z + -         discriminant(_0) = 1;            // bb2[2]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1556:15
2020-04-07T12:02:07.1453248Z + +         _0 = move _1;                    // bb2[0]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1556:15
2020-04-07T12:02:07.1453849Z + +         nop;                             // bb2[1]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1556:15
2020-04-07T12:02:07.1454451Z + +         nop;                             // bb2[2]: scope 8 at $SRC_DIR/libcore/result.rs:LL:COL: 1556:15
2020-04-07T12:02:07.1455282Z 63           goto -> bb3;                     // bb2[3]: scope 0 at $DIR/simplify_try.rs:6:14: 6:15
2020-04-07T12:02:07.1455786Z 65   
2020-04-07T12:02:07.1455919Z 
2020-04-07T12:02:07.1455919Z 
2020-04-07T12:02:07.1456860Z thread '[mir-opt] mir-opt/simplify_try.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_try/rustc.try_identity.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3151:25
2020-04-07T12:02:07.1457921Z 
2020-04-07T12:02:07.1458024Z 
2020-04-07T12:02:07.1458161Z failures:
2020-04-07T12:02:07.1458552Z     [mir-opt] mir-opt/simplify_try.rs
2020-04-07T12:02:07.1458552Z     [mir-opt] mir-opt/simplify_try.rs
2020-04-07T12:02:07.1458739Z 
2020-04-07T12:02:07.1459448Z test result: FAILED. 88 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-07T12:02:07.1459728Z 
2020-04-07T12:02:07.1460456Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-07T12:02:07.1460755Z 
2020-04-07T12:02:07.1460861Z 
2020-04-07T12:02:07.1464945Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-07T12:02:07.1467740Z 
2020-04-07T12:02:07.1467844Z 
2020-04-07T12:02:07.1468416Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-07T12:02:07.1468803Z Build completed unsuccessfully in 1:05:16
2020-04-07T12:02:07.1468803Z Build completed unsuccessfully in 1:05:16
2020-04-07T12:02:07.1523394Z == clock drift check ==
2020-04-07T12:02:07.1537498Z   local time: Tue Apr  7 12:02:07 UTC 2020
2020-04-07T12:02:07.4489383Z   network time: Tue, 07 Apr 2020 12:02:07 GMT
2020-04-07T12:02:09.6163659Z 
2020-04-07T12:02:09.6163659Z 
2020-04-07T12:02:09.6243471Z ##[error]Bash exited with code '1'.
2020-04-07T12:02:09.6270167Z ##[section]Finishing: Run build
2020-04-07T12:02:09.6370710Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-07T12:02:09.6375978Z Task         : Get sources
2020-04-07T12:02:09.6376395Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T12:02:09.6376743Z Version      : 1.0.0
2020-04-07T12:02:09.6376995Z Author       : Microsoft
2020-04-07T12:02:09.6376995Z Author       : Microsoft
2020-04-07T12:02:09.6377357Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T12:02:09.6377793Z ==============================================================================
2020-04-07T12:02:09.9702359Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T12:02:09.9746302Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-07T12:02:09.9837103Z Cleaning up task key
2020-04-07T12:02:09.9838413Z Start cleaning up orphan processes.
2020-04-07T12:02:10.0020803Z Terminate orphan process: pid (3970) (python)
2020-04-07T12:02:10.0191148Z ##[section]Finishing: Finalize Job
