plain
2020-04-07T15:25:58.2320190Z ========================== Starting Command Output ===========================
2020-04-07T15:25:58.2323776Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f9760e70-d9b3-422e-a09f-704cf8353516.sh
2020-04-07T15:25:58.2324330Z 
2020-04-07T15:25:58.2328976Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T15:25:58.2352111Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70891/merge to s
2020-04-07T15:25:58.2356228Z Task         : Get sources
2020-04-07T15:25:58.2356511Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T15:25:58.2356902Z Version      : 1.0.0
2020-04-07T15:25:58.2357218Z Author       : Microsoft
---
2020-04-07T15:25:59.2232756Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T15:25:59.2239503Z ##[command]git config gc.auto 0
2020-04-07T15:25:59.2243352Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T15:25:59.2246744Z ##[command]git config --get-all http.proxy
2020-04-07T15:25:59.2256012Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70891/merge:refs/remotes/pull/70891/merge
---
2020-04-07T15:27:56.0352851Z Looks like docker image is the same as before, not uploading
2020-04-07T15:28:03.6701880Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T15:28:03.7015297Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T15:28:03.7043370Z == clock drift check ==
2020-04-07T15:28:03.7054466Z   local time: Tue Apr  7 15:28:03 UTC 2020
2020-04-07T15:28:04.0117147Z   network time: Tue, 07 Apr 2020 15:28:04 GMT
2020-04-07T15:28:04.0146882Z Starting sccache server...
2020-04-07T15:28:04.0898566Z configure: processing command line
2020-04-07T15:28:04.0899142Z configure: 
2020-04-07T15:28:04.0900186Z configure: rust.dist-src        := False
---
2020-04-07T15:33:10.0181115Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T15:33:11.5390028Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T15:33:13.1257573Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T15:33:15.3165440Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T15:33:23.1629284Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T15:33:27.3294947Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T15:33:31.8439361Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T15:33:35.9887989Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T15:33:43.8432608Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T15:56:03.3612599Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T15:56:05.1901263Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T15:56:07.3612473Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T15:56:10.0515730Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T15:56:19.2152224Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T15:56:24.0156872Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T15:56:29.5017082Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T15:56:34.8732520Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T15:56:43.7396302Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T16:21:36.8091000Z .................................................................................................... 1700/9879
2020-04-07T16:21:40.5132999Z .................................................................................................... 1800/9879
2020-04-07T16:21:48.9089057Z ..................................................................................................i. 1900/9879
2020-04-07T16:21:56.6566583Z .................................................................................................... 2000/9879
2020-04-07T16:22:02.6904694Z ........................................................................................iiiii....... 2100/9879
2020-04-07T16:22:23.1468532Z .................................................................................................... 2300/9879
2020-04-07T16:22:25.1798064Z .................................................................................................... 2400/9879
2020-04-07T16:22:27.2662422Z .................................................................................................... 2500/9879
2020-04-07T16:22:32.8979675Z .................................................................................................... 2600/9879
---
2020-04-07T16:25:18.6879953Z ..............................................................i...............i..................... 5000/9879
2020-04-07T16:25:25.4378058Z .................................................................................................... 5100/9879
2020-04-07T16:25:32.7624158Z .................................................................................................... 5200/9879
2020-04-07T16:25:37.8231081Z .......i............................................................................................ 5300/9879
2020-04-07T16:25:47.2914604Z ................................................................................................ii.i 5400/9879
2020-04-07T16:25:52.1559697Z i........i...i...................................................................................... 5500/9879
2020-04-07T16:26:00.8290837Z .........................................i.......................................................... 5700/9879
2020-04-07T16:26:10.7887593Z .............................................................ii..................................... 5800/9879
2020-04-07T16:26:18.3571516Z i................................................................................................... 5900/9879
2020-04-07T16:26:23.6693982Z .................................................................................................... 6000/9879
2020-04-07T16:26:23.6693982Z .................................................................................................... 6000/9879
2020-04-07T16:26:33.1655708Z ..............................................................................................ii...i 6100/9879
2020-04-07T16:26:44.6452225Z ..ii...........i.................................................................................... 6200/9879
2020-04-07T16:26:59.6568248Z .................................................................................................... 6400/9879
2020-04-07T16:27:05.2526390Z .................................................................................................... 6500/9879
2020-04-07T16:27:05.2526390Z .................................................................................................... 6500/9879
2020-04-07T16:27:21.6917029Z ........................i..ii....................................................................... 6600/9879
2020-04-07T16:27:42.0455319Z .................................................................................................... 6800/9879
2020-04-07T16:27:44.0951233Z ........................i........................................................................... 6900/9879
2020-04-07T16:27:46.1130000Z .................................................................................................... 7000/9879
2020-04-07T16:27:48.2217881Z ...............................................................i.................................... 7100/9879
---
2020-04-07T16:29:24.1846854Z .................................................................................................... 7800/9879
2020-04-07T16:29:28.4927336Z .................................................................................................... 7900/9879
2020-04-07T16:29:34.2267508Z .................................................................................................... 8000/9879
2020-04-07T16:29:41.4750358Z ............................i....................................................................... 8100/9879
2020-04-07T16:29:49.5766026Z ............................................................................iiiiii.iiii.i........... 8200/9879
2020-04-07T16:30:04.8716521Z .....................i......i....................................................................... 8400/9879
2020-04-07T16:30:09.2915567Z .................................................................................................... 8500/9879
2020-04-07T16:30:19.7184967Z .................................................................................................... 8600/9879
2020-04-07T16:30:31.4578142Z .................................................................................................... 8700/9879
---
2020-04-07T16:32:41.4941123Z Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
2020-04-07T16:32:41.5117752Z Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-07T16:32:41.6863777Z 
2020-04-07T16:32:41.6864406Z running 89 tests
2020-04-07T16:32:49.6283898Z ..FF...FFF..FF..F....F..................F....F.F..F..F.F.FF....FF......F......FF.FFFFFFF.
2020-04-07T16:32:49.6293883Z 
2020-04-07T16:32:49.6294888Z ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
2020-04-07T16:32:49.6295551Z 72 
2020-04-07T16:32:49.6295699Z 73     bb6: {
2020-04-07T16:32:49.6295699Z 73     bb6: {
2020-04-07T16:32:49.6296056Z 74         StorageDead(_6);                 // bb6[0]: scope 4 at $DIR/basic_assignment.rs:23:19: 23:20
2020-04-07T16:32:49.6296930Z -         _0 = const ();                   // bb6[1]: scope 0 at $DIR/basic_assignment.rs:10:11: 24:2
2020-04-07T16:32:49.6297536Z -                                          // ty::Const
2020-04-07T16:32:49.6298007Z -                                          // + ty: ()
2020-04-07T16:32:49.6298545Z -                                          // + val: Value(Scalar(<ZST>))
2020-04-07T16:32:49.6299063Z -                                          // mir::Constant
2020-04-07T16:32:49.6299801Z -                                          // + span: $DIR/basic_assignment.rs:10:11: 24:2
2020-04-07T16:32:49.6300581Z -                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-07T16:32:49.6301100Z +         _0 = ();                         // bb6[1]: scope 0 at $DIR/basic_assignment.rs:10:11: 24:2
2020-04-07T16:32:49.6301893Z 82         drop(_5) -> [return: bb7, unwind: bb3]; // bb6[2]: scope 3 at $DIR/basic_assignment.rs:24:1: 24:2
2020-04-07T16:32:49.6302398Z 84 
2020-04-07T16:32:49.6302496Z 
2020-04-07T16:32:49.6302496Z 
2020-04-07T16:32:49.6303341Z thread '[mir-opt] mir-opt/basic_assignment.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/basic_assignment/rustc.main.SimplifyCfg-initial.after.mir', src/tools/compiletest/src/runtest.rs:3151:25
2020-04-07T16:32:49.6304176Z 
2020-04-07T16:32:49.6304546Z ---- [mir-opt] mir-opt/box_expr.rs stdout ----
2020-04-07T16:32:49.6304546Z ---- [mir-opt] mir-opt/box_expr.rs stdout ----
2020-04-07T16:32:49.6304832Z [ERROR compiletest::runtest] Some("    bb5: {")
2020-04-07T16:32:49.6305440Z thread '[mir-opt] mir-opt/box_expr.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6305802Z Current block:     bb5: {
2020-04-07T16:32:49.6306030Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6306262Z Expected Line: "        _0 = ();"
2020-04-07T16:32:49.6306524Z Test Name: rustc.main.ElaborateDrops.before.mir
2020-04-07T16:32:49.6306867Z ... (elided)
2020-04-07T16:32:49.6307047Z     let mut _0: ();
2020-04-07T16:32:49.6307255Z     let _1: std::boxed::Box<S>;
2020-04-07T16:32:49.6307496Z     let mut _2: std::boxed::Box<S>;
2020-04-07T16:32:49.6307496Z     let mut _2: std::boxed::Box<S>;
2020-04-07T16:32:49.6307717Z     let _3: ();
2020-04-07T16:32:49.6307924Z     let mut _4: std::boxed::Box<S>;
2020-04-07T16:32:49.6308123Z     scope 1 {
2020-04-07T16:32:49.6308288Z         debug x => _1;
2020-04-07T16:32:49.6308451Z     }
2020-04-07T16:32:49.6308585Z     bb0: {
2020-04-07T16:32:49.6308876Z         StorageLive(_1);
2020-04-07T16:32:49.6309081Z         StorageLive(_2);
2020-04-07T16:32:49.6309264Z         _2 = Box(S);
2020-04-07T16:32:49.6309741Z         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
2020-04-07T16:32:49.6310005Z     }
2020-04-07T16:32:49.6310162Z     bb1 (cleanup): {
2020-04-07T16:32:49.6310341Z         resume;
2020-04-07T16:32:49.6310635Z     bb2: {
2020-04-07T16:32:49.6310803Z         _1 = move _2;
2020-04-07T16:32:49.6310803Z         _1 = move _2;
2020-04-07T16:32:49.6311146Z         drop(_2) -> bb4;
2020-04-07T16:32:49.6311317Z     }
2020-04-07T16:32:49.6311474Z     bb3 (cleanup): {
2020-04-07T16:32:49.6311816Z         drop(_2) -> bb1;
2020-04-07T16:32:49.6312120Z     bb4: {
2020-04-07T16:32:49.6312286Z         StorageDead(_2);
2020-04-07T16:32:49.6312590Z         StorageLive(_3);
2020-04-07T16:32:49.6312764Z         StorageLive(_4);
2020-04-07T16:32:49.6312764Z         StorageLive(_4);
2020-04-07T16:32:49.6312935Z         _4 = move _1;
2020-04-07T16:32:49.6313487Z         _3 = const std::mem::drop::<std::boxed::Box<S>>(move _4) -> [return: bb5, unwind: bb7];
2020-04-07T16:32:49.6313946Z     bb5: {
2020-04-07T16:32:49.6314111Z         StorageDead(_4);
2020-04-07T16:32:49.6314308Z         StorageDead(_3);
2020-04-07T16:32:49.6314579Z         _0 = ();
2020-04-07T16:32:49.6314579Z         _0 = ();
2020-04-07T16:32:49.6314926Z         drop(_1) -> bb8;
2020-04-07T16:32:49.6315093Z     }
2020-04-07T16:32:49.6315243Z     bb6 (cleanup): {
2020-04-07T16:32:49.6315573Z         drop(_1) -> bb1;
2020-04-07T16:32:49.6315737Z     }
2020-04-07T16:32:49.6315885Z     bb7 (cleanup): {
2020-04-07T16:32:49.6316214Z         drop(_4) -> bb6;
2020-04-07T16:32:49.6316506Z     bb8: {
2020-04-07T16:32:49.6316669Z         StorageDead(_1);
2020-04-07T16:32:49.6316837Z         return;
2020-04-07T16:32:49.6316988Z     }
---
2020-04-07T16:32:49.6318668Z     let mut _4: std::boxed::Box<S>;
2020-04-07T16:32:49.6318871Z     scope 1 {
2020-04-07T16:32:49.6319050Z         debug x => _1;
2020-04-07T16:32:49.6319200Z     }
2020-04-07T16:32:49.6319333Z     bb0: {
2020-04-07T16:32:49.6319515Z         StorageLive(_1);
2020-04-07T16:32:49.6319698Z         StorageLive(_2);
2020-04-07T16:32:49.6319874Z         _2 = Box(S);
2020-04-07T16:32:49.6320368Z         (*_2) = const S::new() -> [return: bb2, unwind: bb3];
2020-04-07T16:32:49.6320610Z     }
2020-04-07T16:32:49.6320763Z     bb1 (cleanup): {
2020-04-07T16:32:49.6320935Z         resume;
2020-04-07T16:32:49.6321218Z     bb2: {
2020-04-07T16:32:49.6321379Z         _1 = move _2;
2020-04-07T16:32:49.6321379Z         _1 = move _2;
2020-04-07T16:32:49.6321722Z         drop(_2) -> bb4;
2020-04-07T16:32:49.6321871Z     }
2020-04-07T16:32:49.6322023Z     bb3 (cleanup): {
2020-04-07T16:32:49.6322366Z         drop(_2) -> bb1;
2020-04-07T16:32:49.6322644Z     bb4: {
2020-04-07T16:32:49.6322804Z         StorageDead(_2);
2020-04-07T16:32:49.6323343Z         StorageLive(_3);
2020-04-07T16:32:49.6323541Z         StorageLive(_4);
2020-04-07T16:32:49.6323541Z         StorageLive(_4);
2020-04-07T16:32:49.6323718Z         _4 = move _1;
2020-04-07T16:32:49.6324354Z         _3 = const std::mem::drop::<std::boxed::Box<S>>(move _4) -> [return: bb5, unwind: bb7];
2020-04-07T16:32:49.6324799Z     bb5: {
2020-04-07T16:32:49.6324978Z         StorageDead(_4);
2020-04-07T16:32:49.6325161Z         StorageDead(_3);
2020-04-07T16:32:49.6325340Z         _0 = const ();
2020-04-07T16:32:49.6325340Z         _0 = const ();
2020-04-07T16:32:49.6325682Z         drop(_1) -> bb8;
2020-04-07T16:32:49.6325836Z     }
2020-04-07T16:32:49.6325989Z     bb6 (cleanup): {
2020-04-07T16:32:49.6326322Z         drop(_1) -> bb1;
2020-04-07T16:32:49.6326486Z     }
2020-04-07T16:32:49.6326636Z     bb7 (cleanup): {
2020-04-07T16:32:49.6326967Z         drop(_4) -> bb6;
2020-04-07T16:32:49.6327260Z     bb8: {
2020-04-07T16:32:49.6327424Z         StorageDead(_1);
2020-04-07T16:32:49.6327607Z         return;
2020-04-07T16:32:49.6327744Z     }
2020-04-07T16:32:49.6327744Z     }
2020-04-07T16:32:49.6328120Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6328296Z 
2020-04-07T16:32:49.6328675Z ---- [mir-opt] mir-opt/const_allocation.rs stdout ----
2020-04-07T16:32:49.6329070Z 18         _1 = (*_2);                      // bb0[3]: scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-04-07T16:32:49.6329577Z 19         StorageDead(_2);                 // bb0[4]: scope 0 at $DIR/const_allocation.rs:8:8: 8:9
2020-04-07T16:32:49.6330101Z 20         StorageDead(_1);                 // bb0[5]: scope 0 at $DIR/const_allocation.rs:8:8: 8:9
2020-04-07T16:32:49.6330828Z -         _0 = const ();                   // bb0[6]: scope 0 at $DIR/const_allocation.rs:7:11: 9:2
2020-04-07T16:32:49.6331401Z -                                          // ty::Const
2020-04-07T16:32:49.6331885Z -                                          // + ty: ()
2020-04-07T16:32:49.6332402Z -                                          // + val: Value(Scalar(<ZST>))
2020-04-07T16:32:49.6332918Z -                                          // mir::Constant
2020-04-07T16:32:49.6333697Z -                                          // + span: $DIR/const_allocation.rs:7:11: 9:2
2020-04-07T16:32:49.6334392Z -                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-07T16:32:49.6334918Z +         _0 = ();                         // bb0[6]: scope 0 at $DIR/const_allocation.rs:7:11: 9:2
2020-04-07T16:32:49.6335428Z 28         return;                          // bb0[7]: scope 0 at $DIR/const_allocation.rs:9:2: 9:2
2020-04-07T16:32:49.6335886Z 30 }
2020-04-07T16:32:49.6335983Z 
2020-04-07T16:32:49.6335983Z 
2020-04-07T16:32:49.6336882Z thread '[mir-opt] mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation/64bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3151:25
2020-04-07T16:32:49.6337783Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-04-07T16:32:49.6337783Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-04-07T16:32:49.6338194Z 18         _1 = (*_2);                      // bb0[3]: scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-04-07T16:32:49.6338751Z 19         StorageDead(_2);                 // bb0[4]: scope 0 at $DIR/const_allocation2.rs:5:8: 5:9
2020-04-07T16:32:49.6339280Z 20         StorageDead(_1);                 // bb0[5]: scope 0 at $DIR/const_allocation2.rs:5:8: 5:9
2020-04-07T16:32:49.6340023Z -         _0 = const ();                   // bb0[6]: scope 0 at $DIR/const_allocation2.rs:4:11: 6:2
2020-04-07T16:32:49.6340599Z -                                          // ty::Const
2020-04-07T16:32:49.6341080Z -                                          // + ty: ()
2020-04-07T16:32:49.6341596Z -                                          // + val: Value(Scalar(<ZST>))
2020-04-07T16:32:49.6342114Z -                                          // mir::Constant
2020-04-07T16:32:49.6342723Z -                                          // + span: $DIR/const_allocation2.rs:4:11: 6:2
2020-04-07T16:32:49.6343416Z -                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-07T16:32:49.6343949Z +         _0 = ();                         // bb0[6]: scope 0 at $DIR/const_allocation2.rs:4:11: 6:2
2020-04-07T16:32:49.6344463Z 28         return;                          // bb0[7]: scope 0 at $DIR/const_allocation2.rs:6:2: 6:2
2020-04-07T16:32:49.6344923Z 30 }
2020-04-07T16:32:49.6345020Z 
2020-04-07T16:32:49.6345020Z 
2020-04-07T16:32:49.6345840Z thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2/64bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3151:25
2020-04-07T16:32:49.6346708Z ---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
2020-04-07T16:32:49.6346708Z ---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
2020-04-07T16:32:49.6347097Z 18         _1 = (*_2);                      // bb0[3]: scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-04-07T16:32:49.6347607Z 19         StorageDead(_2);                 // bb0[4]: scope 0 at $DIR/const_allocation3.rs:5:8: 5:9
2020-04-07T16:32:49.6348140Z 20         StorageDead(_1);                 // bb0[5]: scope 0 at $DIR/const_allocation3.rs:5:8: 5:9
2020-04-07T16:32:49.6348867Z -         _0 = const ();                   // bb0[6]: scope 0 at $DIR/const_allocation3.rs:4:11: 6:2
2020-04-07T16:32:49.6349579Z -                                          // ty::Const
2020-04-07T16:32:49.6350073Z -                                          // + ty: ()
2020-04-07T16:32:49.6350606Z -                                          // + val: Value(Scalar(<ZST>))
2020-04-07T16:32:49.6351137Z -                                          // mir::Constant
2020-04-07T16:32:49.6351771Z -                                          // + span: $DIR/const_allocation3.rs:4:11: 6:2
2020-04-07T16:32:49.6352594Z -                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-07T16:32:49.6353121Z +         _0 = ();                         // bb0[6]: scope 0 at $DIR/const_allocation3.rs:4:11: 6:2
2020-04-07T16:32:49.6353735Z 28         return;                          // bb0[7]: scope 0 at $DIR/const_allocation3.rs:6:2: 6:2
2020-04-07T16:32:49.6354196Z 30 }
2020-04-07T16:32:49.6354294Z 
2020-04-07T16:32:49.6354294Z 
2020-04-07T16:32:49.6355153Z thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3/64bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3151:25
2020-04-07T16:32:49.6356013Z ---- [mir-opt] mir-opt/const_prop/boxes.rs stdout ----
2020-04-07T16:32:49.6356013Z ---- [mir-opt] mir-opt/const_prop/boxes.rs stdout ----
2020-04-07T16:32:49.6356743Z thread '[mir-opt] mir-opt/const_prop/boxes.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
2020-04-07T16:32:49.6357136Z Expected Line: "     _0 = ();"
2020-04-07T16:32:49.6357382Z Test Name: rustc.main.ConstProp.before.mir
2020-04-07T16:32:49.6357709Z ... (elided)
2020-04-07T16:32:49.6357859Z  bb0: {
2020-04-07T16:32:49.6357999Z ... (elided)
2020-04-07T16:32:49.6358157Z      _4 = Box(i32);
2020-04-07T16:32:49.6358157Z      _4 = Box(i32);
2020-04-07T16:32:49.6358336Z      (*_4) = const 42i32;
2020-04-07T16:32:49.6358530Z      _3 = move _4;
2020-04-07T16:32:49.6358684Z ... (elided)
2020-04-07T16:32:49.6358837Z      _2 = (*_3);
2020-04-07T16:32:49.6359042Z      _1 = Add(move _2, const 0i32);
2020-04-07T16:32:49.6359224Z ... (elided)
2020-04-07T16:32:49.6359636Z      drop(_3) -> [return: bb2, unwind: bb1];
2020-04-07T16:32:49.6359852Z  }
2020-04-07T16:32:49.6359997Z  bb1 (cleanup): {
2020-04-07T16:32:49.6360157Z      resume;
2020-04-07T16:32:49.6360420Z  bb2: {
2020-04-07T16:32:49.6360558Z ... (elided)
2020-04-07T16:32:49.6360707Z      _0 = ();
2020-04-07T16:32:49.6360867Z ... (elided)
---
2020-04-07T16:32:49.6362430Z     let mut _4: std::boxed::Box<i32>;
2020-04-07T16:32:49.6362631Z     scope 1 {
2020-04-07T16:32:49.6362809Z         debug x => _1;
2020-04-07T16:32:49.6389491Z     }
2020-04-07T16:32:49.6389653Z     bb0: {
2020-04-07T16:32:49.6389851Z         StorageLive(_1);
2020-04-07T16:32:49.6390036Z         StorageLive(_2);
2020-04-07T16:32:49.6390218Z         StorageLive(_3);
2020-04-07T16:32:49.6390414Z         StorageLive(_4);
2020-04-07T16:32:49.6390594Z         _4 = Box(i32);
2020-04-07T16:32:49.6390780Z         (*_4) = const 42i32;
2020-04-07T16:32:49.6390965Z         _3 = move _4;
2020-04-07T16:32:49.6391172Z         StorageDead(_4);
2020-04-07T16:32:49.6391347Z         _2 = (*_3);
2020-04-07T16:32:49.6391546Z         _1 = Add(move _2, const 0i32);
2020-04-07T16:32:49.6391767Z         StorageDead(_2);
2020-04-07T16:32:49.6392388Z         drop(_3) -> [return: bb2, unwind: bb1];
2020-04-07T16:32:49.6392614Z     }
2020-04-07T16:32:49.6392787Z     bb1 (cleanup): {
2020-04-07T16:32:49.6392960Z         resume;
2020-04-07T16:32:49.6393244Z     bb2: {
2020-04-07T16:32:49.6393409Z         StorageDead(_3);
2020-04-07T16:32:49.6393588Z         _0 = const ();
2020-04-07T16:32:49.6393766Z         StorageDead(_1);
2020-04-07T16:32:49.6393766Z         StorageDead(_1);
2020-04-07T16:32:49.6393950Z         return;
2020-04-07T16:32:49.6394086Z     }
2020-04-07T16:32:49.6394477Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6394666Z 
2020-04-07T16:32:49.6395030Z ---- [mir-opt] mir-opt/const_prop/cast.rs stdout ----
2020-04-07T16:32:49.6395327Z 39 +                                          // mir::Constant
2020-04-07T16:32:49.6395713Z 40 +                                          // + span: $DIR/cast.rs:6:13: 6:24
2020-04-07T16:32:49.6396178Z 41 +                                          // + literal: Const { ty: u8, val: Value(Scalar(0x2a)) }
2020-04-07T16:32:49.6398492Z -           _0 = const ();                   // bb0[4]: scope 0 at $DIR/cast.rs:3:11: 7:2
2020-04-07T16:32:49.6399135Z -                                            // ty::Const
2020-04-07T16:32:49.6399649Z -                                            // + ty: ()
2020-04-07T16:32:49.6400341Z -                                            // + val: Value(Scalar(<ZST>))
2020-04-07T16:32:49.6401006Z -                                            // mir::Constant
2020-04-07T16:32:49.6401565Z -                                            // + span: $DIR/cast.rs:3:11: 7:2
2020-04-07T16:32:49.6402239Z -                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
2020-04-07T16:32:49.6403799Z +           _0 = ();                         // bb0[4]: scope 0 at $DIR/cast.rs:3:11: 7:2
2020-04-07T16:32:49.6404296Z 49           StorageDead(_2);                 // bb0[5]: scope 1 at $DIR/cast.rs:7:1: 7:2
2020-04-07T16:32:49.6404746Z 50           StorageDead(_1);                 // bb0[6]: scope 0 at $DIR/cast.rs:7:1: 7:2
2020-04-07T16:32:49.6405224Z 51           return;                          // bb0[7]: scope 0 at $DIR/cast.rs:7:2: 7:2
2020-04-07T16:32:49.6405487Z 
2020-04-07T16:32:49.6406400Z thread '[mir-opt] mir-opt/const_prop/cast.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/cast/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3151:25
2020-04-07T16:32:49.6407376Z ---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
2020-04-07T16:32:49.6407376Z ---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
2020-04-07T16:32:49.6407675Z [ERROR compiletest::runtest] Some("bb1: {")
2020-04-07T16:32:49.6408337Z thread '[mir-opt] mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6408701Z Current block: bb1: {
2020-04-07T16:32:49.6408904Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6409125Z Expected Line: "  _0 = ();"
2020-04-07T16:32:49.6409347Z Test Name: rustc.hello.ConstProp.before.mir
2020-04-07T16:32:49.6409672Z ... (elided)
2020-04-07T16:32:49.6409816Z let mut _0: ();
2020-04-07T16:32:49.6409975Z let mut _1: bool;
2020-04-07T16:32:49.6410130Z let mut _2: !;
2020-04-07T16:32:49.6410130Z let mut _2: !;
2020-04-07T16:32:49.6410280Z bb0: {
2020-04-07T16:32:49.6410419Z   StorageLive(_1);
2020-04-07T16:32:49.6410623Z   _1 = const <bool as NeedsDrop>::NEEDS;
2020-04-07T16:32:49.6411075Z   switchInt(_1) -> [false: bb1, otherwise: bb2];
2020-04-07T16:32:49.6411370Z bb1: {
2020-04-07T16:32:49.6411508Z   _0 = ();
2020-04-07T16:32:49.6411649Z   StorageDead(_1);
2020-04-07T16:32:49.6411786Z   return;
2020-04-07T16:32:49.6411786Z   return;
2020-04-07T16:32:49.6411897Z }
2020-04-07T16:32:49.6412022Z bb2: {
2020-04-07T16:32:49.6412162Z   StorageLive(_2);
2020-04-07T16:32:49.6412396Z   const std::rt::begin_panic::<&str>(const "explicit panic");
2020-04-07T16:32:49.6412725Z Actual:
2020-04-07T16:32:49.6412725Z Actual:
2020-04-07T16:32:49.6413000Z fn hello() -> () {
2020-04-07T16:32:49.6413179Z     let mut _0: ();
2020-04-07T16:32:49.6413353Z     let mut _1: bool;
2020-04-07T16:32:49.6413524Z     let mut _2: !;
2020-04-07T16:32:49.6413674Z     bb0: {
2020-04-07T16:32:49.6413845Z         StorageLive(_1);
2020-04-07T16:32:49.6414185Z         _1 = const <bool as NeedsDrop>::NEEDS;
2020-04-07T16:32:49.6414672Z         switchInt(_1) -> [false: bb1, otherwise: bb2];
2020-04-07T16:32:49.6415037Z     bb1: {
2020-04-07T16:32:49.6415197Z         _0 = const ();
2020-04-07T16:32:49.6415392Z         StorageDead(_1);
2020-04-07T16:32:49.6415560Z         return;
2020-04-07T16:32:49.6415560Z         return;
2020-04-07T16:32:49.6415697Z     }
2020-04-07T16:32:49.6415843Z     bb2: {
2020-04-07T16:32:49.6416010Z         StorageLive(_2);
2020-04-07T16:32:49.6416284Z         const std::rt::begin_panic::<&str>(const "explicit panic");
2020-04-07T16:32:49.6416912Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6417089Z 
2020-04-07T16:32:49.6417494Z ---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
2020-04-07T16:32:49.6417494Z ---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
2020-04-07T16:32:49.6418027Z [ERROR compiletest::runtest] Some("bb2: {")
2020-04-07T16:32:49.6418692Z thread '[mir-opt] mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6419183Z Current block: bb2: {
2020-04-07T16:32:49.6419413Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6419635Z Expected Line: "  _0 = ();"
2020-04-07T16:32:49.6419863Z Test Name: rustc.main.ConstProp.before.mir
2020-04-07T16:32:49.6420207Z ... (elided)
2020-04-07T16:32:49.6420362Z let mut _0: ();
2020-04-07T16:32:49.6420533Z let _1: i32;
2020-04-07T16:32:49.6420706Z let mut _2: (i32, bool);
---
2020-04-07T16:32:49.6423541Z       debug z => _8;
2020-04-07T16:32:49.6423677Z     }
2020-04-07T16:32:49.6423785Z   }
2020-04-07T16:32:49.6423903Z }
2020-04-07T16:32:49.6424013Z bb0: {
2020-04-07T16:32:49.6424150Z   StorageLive(_1);
2020-04-07T16:32:49.6424356Z   _2 = CheckedAdd(const 2i32, const 2i32);
2020-04-07T16:32:49.6428063Z   assert(!move (_2.1: bool), "attempt to add with overflow") -> bb1;
2020-04-07T16:32:49.6428428Z bb1: {
2020-04-07T16:32:49.6428428Z bb1: {
2020-04-07T16:32:49.6428791Z   _1 = move (_2.0: i32);
2020-04-07T16:32:49.6428969Z   StorageLive(_3);
2020-04-07T16:32:49.6429121Z   StorageLive(_4);
2020-04-07T16:32:49.6429516Z   _4 = [const 0i32, const 1i32, const 2i32, const 3i32, const 4i32, const 5i32];
2020-04-07T16:32:49.6429785Z   StorageLive(_5);
2020-04-07T16:32:49.6430138Z   _6 = const 6usize;
2020-04-07T16:32:49.6430138Z   _6 = const 6usize;
2020-04-07T16:32:49.6430304Z   _7 = Lt(_5, _6);
2020-04-07T16:32:49.6430941Z   assert(move _7, "index out of bounds: the len is move _6 but the index is _5") -> bb2;
2020-04-07T16:32:49.6431334Z bb2: {
2020-04-07T16:32:49.6431334Z bb2: {
2020-04-07T16:32:49.6431473Z   _3 = _4[_5];
2020-04-07T16:32:49.6431644Z   StorageDead(_5);
2020-04-07T16:32:49.6431807Z   StorageDead(_4);
2020-04-07T16:32:49.6431968Z   StorageLive(_8);
2020-04-07T16:32:49.6432130Z   StorageLive(_9);
2020-04-07T16:32:49.6432375Z   _9 = Point { x: const 12u32, y: const 42u32 };
2020-04-07T16:32:49.6432611Z   _8 = (_9.1: u32);
2020-04-07T16:32:49.6432901Z   StorageDead(_9);
2020-04-07T16:32:49.6435091Z   StorageDead(_8);
2020-04-07T16:32:49.6435254Z   StorageDead(_3);
2020-04-07T16:32:49.6437419Z   StorageDead(_1);
2020-04-07T16:32:49.6437584Z   return;
---
2020-04-07T16:32:49.6442622Z                 debug z => _8;
2020-04-07T16:32:49.6442808Z             }
2020-04-07T16:32:49.6443094Z         }
2020-04-07T16:32:49.6443440Z     }
2020-04-07T16:32:49.6443580Z     bb0: {
2020-04-07T16:32:49.6443752Z         StorageLive(_1);
2020-04-07T16:32:49.6444001Z         _2 = CheckedAdd(const 2i32, const 2i32);
2020-04-07T16:32:49.6444644Z         assert(!move (_2.1: bool), "attempt to add with overflow") -> bb1;
2020-04-07T16:32:49.6445161Z     bb1: {
2020-04-07T16:32:49.6445161Z     bb1: {
2020-04-07T16:32:49.6445348Z         _1 = move (_2.0: i32);
2020-04-07T16:32:49.6445554Z         StorageLive(_3);
2020-04-07T16:32:49.6445737Z         StorageLive(_4);
2020-04-07T16:32:49.6446247Z         _4 = [const 0i32, const 1i32, const 2i32, const 3i32, const 4i32, const 5i32];
2020-04-07T16:32:49.6446548Z         StorageLive(_5);
2020-04-07T16:32:49.6447064Z         _6 = const 6usize;
2020-04-07T16:32:49.6447064Z         _6 = const 6usize;
2020-04-07T16:32:49.6452799Z         _7 = Lt(_5, _6);
2020-04-07T16:32:49.6453750Z         assert(move _7, "index out of bounds: the len is move _6 but the index is _5") -> bb2;
2020-04-07T16:32:49.6454221Z     bb2: {
2020-04-07T16:32:49.6454221Z     bb2: {
2020-04-07T16:32:49.6454381Z         _3 = _4[_5];
2020-04-07T16:32:49.6454574Z         StorageDead(_5);
2020-04-07T16:32:49.6454758Z         StorageDead(_4);
2020-04-07T16:32:49.6454941Z         StorageLive(_8);
2020-04-07T16:32:49.6455140Z         StorageLive(_9);
2020-04-07T16:32:49.6455513Z         _9 = Point { x: const 12u32, y: const 42u32 };
2020-04-07T16:32:49.6455788Z         _8 = (_9.1: u32);
2020-04-07T16:32:49.6455991Z         StorageDead(_9);
2020-04-07T16:32:49.6456418Z         _0 = const ();
2020-04-07T16:32:49.6456609Z         StorageDead(_8);
2020-04-07T16:32:49.6458395Z         StorageDead(_3);
2020-04-07T16:32:49.6458613Z         StorageDead(_1);
2020-04-07T16:32:49.6459113Z     }
2020-04-07T16:32:49.6459707Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6459897Z 
2020-04-07T16:32:49.6460261Z ---- [mir-opt] mir-opt/graphviz.rs stdout ----
2020-04-07T16:32:49.6460261Z ---- [mir-opt] mir-opt/graphviz.rs stdout ----
2020-04-07T16:32:49.6468707Z [ERROR compiletest::runtest] None
2020-04-07T16:32:49.6469554Z thread '[mir-opt] mir-opt/graphviz.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6469910Z Current block: None
2020-04-07T16:32:49.6470801Z Actual Line: "    bb0__0_3 [shape=\"none\", label=<<table border=\"0\" cellborder=\"1\" cellspacing=\"0\"><tr><td bgcolor=\"gray\" align=\"center\" colspan=\"1\">0</td></tr><tr><td align=\"left\" balign=\"left\">_0 = const ()<br/></td></tr><tr><td align=\"left\">goto</td></tr></table>>];"
2020-04-07T16:32:49.6472068Z Expected Line: "    bb0__0_3 [shape=\"none\", label=<<table border=\"0\" cellborder=\"1\" cellspacing=\"0\"><tr><td bgcolor=\"gray\" align=\"center\" colspan=\"1\">0</td></tr><tr><td align=\"left\" balign=\"left\">_0 = ()<br/></td></tr><tr><td align=\"left\">goto</td></tr></table>>];"
2020-04-07T16:32:49.6472733Z Test Name: rustc.main.mir_map.0.dot
2020-04-07T16:32:49.6473091Z ... (elided)
2020-04-07T16:32:49.6473091Z ... (elided)
2020-04-07T16:32:49.6473330Z digraph Mir_0_3 { // The name here MUST be an ASCII identifier.
2020-04-07T16:32:49.6473622Z     graph [fontname="monospace"];
2020-04-07T16:32:49.6473841Z     node [fontname="monospace"];
2020-04-07T16:32:49.6474055Z     edge [fontname="monospace"];
2020-04-07T16:32:49.6474580Z     label=<fn main() -&gt; ()<br align="left"/>>;
2020-04-07T16:32:49.6475198Z     bb0__0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">0</td></tr><tr><td align="left" balign="left">_0 = ()<br/></td></tr><tr><td align="left">goto</td></tr></table>>];
2020-04-07T16:32:49.6476054Z     bb1__0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">1</td></tr><tr><td align="left">resume</td></tr></table>>];
2020-04-07T16:32:49.6477099Z     bb2__0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">2</td></tr><tr><td align="left">return</td></tr></table>>];
2020-04-07T16:32:49.6478283Z     bb0__0_3 -> bb2__0_3 [label=""];
2020-04-07T16:32:49.6478608Z Actual:
2020-04-07T16:32:49.6478608Z Actual:
2020-04-07T16:32:49.6478760Z digraph Mir_0_3 {
2020-04-07T16:32:49.6478953Z     graph [fontname="monospace"];
2020-04-07T16:32:49.6479305Z     node [fontname="monospace"];
2020-04-07T16:32:49.6479514Z     edge [fontname="monospace"];
2020-04-07T16:32:49.6479940Z     label=<fn main() -&gt; ()<br align="left"/>>;
2020-04-07T16:32:49.6480715Z     bb0__0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">0</td></tr><tr><td align="left" balign="left">_0 = const ()<br/></td></tr><tr><td align="left">goto</td></tr></table>>];
2020-04-07T16:32:49.6482064Z     bb1__0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">1</td></tr><tr><td align="left">resume</td></tr></table>>];
2020-04-07T16:32:49.6482854Z     bb2__0_3 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">2</td></tr><tr><td align="left">return</td></tr></table>>];
2020-04-07T16:32:49.6484042Z     bb0__0_3 -> bb2__0_3 [label=""];
2020-04-07T16:32:49.6484592Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6485412Z ---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
2020-04-07T16:32:49.6485412Z ---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
2020-04-07T16:32:49.6485708Z [ERROR compiletest::runtest] Some("bb2: {")
2020-04-07T16:32:49.6486377Z thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6486748Z Current block: bb2: {
2020-04-07T16:32:49.6486958Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6487310Z Expected Line: "  _0 = ();"
2020-04-07T16:32:49.6487522Z Test Name: rustc.main.Inline.before.mir
2020-04-07T16:32:49.6487828Z ... (elided)
2020-04-07T16:32:49.6487989Z let mut _0: ();
2020-04-07T16:32:49.6487989Z let mut _0: ();
2020-04-07T16:32:49.6488314Z let _1: std::boxed::Box<std::vec::Vec<u32>> as UserTypeProjection { base: UserType(0), projs: [] };
2020-04-07T16:32:49.6488694Z let mut _2: std::boxed::Box<std::vec::Vec<u32>>;
2020-04-07T16:32:49.6488925Z let mut _3: ();
2020-04-07T16:32:49.6489339Z   debug _x => _1;
2020-04-07T16:32:49.6489490Z }
2020-04-07T16:32:49.6489754Z bb0: {
2020-04-07T16:32:49.6489904Z   StorageLive(_1);
2020-04-07T16:32:49.6489904Z   StorageLive(_1);
2020-04-07T16:32:49.6490082Z   StorageLive(_2);
2020-04-07T16:32:49.6490280Z   _2 = Box(std::vec::Vec<u32>);
2020-04-07T16:32:49.6490850Z   (*_2) = const std::vec::Vec::<u32>::new() -> [return: bb2, unwind: bb4];
2020-04-07T16:32:49.6491127Z }
2020-04-07T16:32:49.6491268Z bb1 (cleanup): {
2020-04-07T16:32:49.6491428Z   resume;
2020-04-07T16:32:49.6491679Z bb2: {
2020-04-07T16:32:49.6491820Z   _1 = move _2;
2020-04-07T16:32:49.6491979Z   StorageDead(_2);
2020-04-07T16:32:49.6492250Z   _0 = ();
2020-04-07T16:32:49.6492250Z   _0 = ();
2020-04-07T16:32:49.6492722Z   drop(_1) -> [return: bb3, unwind: bb1];
2020-04-07T16:32:49.6493045Z bb3: {
2020-04-07T16:32:49.6493325Z   StorageDead(_1);
2020-04-07T16:32:49.6493481Z   return;
2020-04-07T16:32:49.6493598Z }
2020-04-07T16:32:49.6493598Z }
2020-04-07T16:32:49.6493753Z bb4 (cleanup): {
2020-04-07T16:32:49.6494399Z   _3 = const alloc::alloc::box_free::<std::vec::Vec<u32>>(move (_2.0: std::ptr::Unique<std::vec::Vec<u32>>)) -> bb1;
2020-04-07T16:32:49.6494862Z Actual:
2020-04-07T16:32:49.6495148Z fn main() -> () {
2020-04-07T16:32:49.6495318Z     let mut _0: ();
2020-04-07T16:32:49.6495318Z     let mut _0: ();
2020-04-07T16:32:49.6495688Z     let _1: std::boxed::Box<std::vec::Vec<u32>> as UserTypeProjection { base: UserType(0), projs: [] };
2020-04-07T16:32:49.6496203Z     let mut _2: std::boxed::Box<std::vec::Vec<u32>>;
2020-04-07T16:32:49.6496437Z     let mut _3: ();
2020-04-07T16:32:49.6496762Z         debug _x => _1;
2020-04-07T16:32:49.6496902Z     }
2020-04-07T16:32:49.6497041Z     bb0: {
2020-04-07T16:32:49.6497467Z         StorageLive(_1);
2020-04-07T16:32:49.6497467Z         StorageLive(_1);
2020-04-07T16:32:49.6497652Z         StorageLive(_2);
2020-04-07T16:32:49.6498044Z         _2 = Box(std::vec::Vec<u32>);
2020-04-07T16:32:49.6498687Z         (*_2) = const std::vec::Vec::<u32>::new() -> [return: bb2, unwind: bb4];
2020-04-07T16:32:49.6498971Z     }
2020-04-07T16:32:49.6499126Z     bb1 (cleanup): {
2020-04-07T16:32:49.6499315Z         resume;
2020-04-07T16:32:49.6499586Z     bb2: {
2020-04-07T16:32:49.6499762Z         _1 = move _2;
2020-04-07T16:32:49.6499940Z         StorageDead(_2);
2020-04-07T16:32:49.6500120Z         _0 = const ();
2020-04-07T16:32:49.6500120Z         _0 = const ();
2020-04-07T16:32:49.6501001Z         drop(_1) -> [return: bb3, unwind: bb1];
2020-04-07T16:32:49.6501644Z     bb3: {
2020-04-07T16:32:49.6501843Z         StorageDead(_1);
2020-04-07T16:32:49.6502029Z         return;
2020-04-07T16:32:49.6502167Z     }
2020-04-07T16:32:49.6502167Z     }
2020-04-07T16:32:49.6502321Z     bb4 (cleanup): {
2020-04-07T16:32:49.6503181Z         _3 = const alloc::alloc::box_free::<std::vec::Vec<u32>>(move (_2.0: std::ptr::Unique<std::vec::Vec<u32>>)) -> bb1;
2020-04-07T16:32:49.6504037Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6504229Z 
2020-04-07T16:32:49.6504751Z ---- [mir-opt] mir-opt/inline/inline-specialization.rs stdout ----
2020-04-07T16:32:49.6504751Z ---- [mir-opt] mir-opt/inline/inline-specialization.rs stdout ----
2020-04-07T16:32:49.6505221Z [ERROR compiletest::runtest] Some("bb1: {")
2020-04-07T16:32:49.6505953Z thread '[mir-opt] mir-opt/inline/inline-specialization.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6506346Z Current block: bb1: {
2020-04-07T16:32:49.6506569Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6506814Z Expected Line: "  _0 = ();"
2020-04-07T16:32:49.6507055Z Test Name: rustc.main.Inline.before.mir
2020-04-07T16:32:49.6507393Z ... (elided)
2020-04-07T16:32:49.6507692Z let mut _0: ();
2020-04-07T16:32:49.6507950Z let _1: u32;
2020-04-07T16:32:49.6508083Z scope 1 {
2020-04-07T16:32:49.6508083Z scope 1 {
2020-04-07T16:32:49.6508238Z   debug x => _1;
2020-04-07T16:32:49.6508360Z }
2020-04-07T16:32:49.6508470Z bb0: {
2020-04-07T16:32:49.6508620Z   StorageLive(_1);
2020-04-07T16:32:49.6509027Z   _1 = const <std::vec::Vec<()> as Foo>::bar() -> bb1;
2020-04-07T16:32:49.6509340Z bb1: {
2020-04-07T16:32:49.6509479Z   _0 = ();
2020-04-07T16:32:49.6509621Z   StorageDead(_1);
2020-04-07T16:32:49.6509759Z   return;
---
2020-04-07T16:32:49.6510599Z     let _1: u32;
2020-04-07T16:32:49.6510743Z     scope 1 {
2020-04-07T16:32:49.6510890Z         debug x => _1;
2020-04-07T16:32:49.6511042Z     }
2020-04-07T16:32:49.6511167Z     bb0: {
2020-04-07T16:32:49.6511438Z         StorageLive(_1);
2020-04-07T16:32:49.6512051Z         _1 = const <std::vec::Vec<()> as Foo>::bar() -> bb1;
2020-04-07T16:32:49.6512431Z     bb1: {
2020-04-07T16:32:49.6512608Z         _0 = const ();
2020-04-07T16:32:49.6512787Z         StorageDead(_1);
2020-04-07T16:32:49.6512956Z         return;
2020-04-07T16:32:49.6512956Z         return;
2020-04-07T16:32:49.6513092Z     }
2020-04-07T16:32:49.6513525Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6513704Z 
2020-04-07T16:32:49.6514624Z ---- [mir-opt] mir-opt/issue-38669.rs stdout ----
2020-04-07T16:32:49.6514917Z [ERROR compiletest::runtest] Some("    bb3: {")
2020-04-07T16:32:49.6515498Z thread '[mir-opt] mir-opt/issue-38669.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6515825Z Current block:     bb3: {
2020-04-07T16:32:49.6516049Z Actual Line: "        _3 = const ();"
2020-04-07T16:32:49.6516265Z Expected Line: "        _3 = ();"
2020-04-07T16:32:49.6516670Z Test Name: rustc.main.SimplifyCfg-initial.after.mir
2020-04-07T16:32:49.6517001Z ... (elided)
2020-04-07T16:32:49.6517137Z     bb0: {
2020-04-07T16:32:49.6517291Z         StorageLive(_1);
2020-04-07T16:32:49.6517614Z         _1 = const false;
2020-04-07T16:32:49.6517614Z         _1 = const false;
2020-04-07T16:32:49.6517796Z         FakeRead(ForLet, _1);
2020-04-07T16:32:49.6518147Z         goto -> bb2;
2020-04-07T16:32:49.6518301Z     }
2020-04-07T16:32:49.6518444Z     bb1 (cleanup): {
2020-04-07T16:32:49.6518605Z         resume;
2020-04-07T16:32:49.6518871Z     bb2: {
2020-04-07T16:32:49.6518871Z     bb2: {
2020-04-07T16:32:49.6519245Z         falseUnwind -> [real: bb3, cleanup: bb1];
2020-04-07T16:32:49.6519576Z     bb3: {
2020-04-07T16:32:49.6519730Z         StorageLive(_3);
2020-04-07T16:32:49.6519902Z         StorageLive(_4);
2020-04-07T16:32:49.6520074Z         _4 = _1;
2020-04-07T16:32:49.6520074Z         _4 = _1;
2020-04-07T16:32:49.6520255Z         FakeRead(ForMatchedPlace, _4);
2020-04-07T16:32:49.6520760Z         switchInt(_4) -> [false: bb5, otherwise: bb4];
2020-04-07T16:32:49.6521125Z ... (elided)
2020-04-07T16:32:49.6521260Z     bb5: {
2020-04-07T16:32:49.6521415Z         _3 = ();
2020-04-07T16:32:49.6521580Z         StorageDead(_4);
---
2020-04-07T16:32:49.6526018Z     }
2020-04-07T16:32:49.6526141Z     bb0: {
2020-04-07T16:32:49.6526294Z         StorageLive(_1);
2020-04-07T16:32:49.6526482Z         _1 = const false;
2020-04-07T16:32:49.6526663Z         FakeRead(ForLet, _1);
2020-04-07T16:32:49.6526979Z         goto -> bb2;
2020-04-07T16:32:49.6527129Z     }
2020-04-07T16:32:49.6527269Z     bb1 (cleanup): {
2020-04-07T16:32:49.6527424Z         resume;
2020-04-07T16:32:49.6527689Z     bb2: {
2020-04-07T16:32:49.6527689Z     bb2: {
2020-04-07T16:32:49.6528062Z         falseUnwind -> [real: bb3, cleanup: bb1];
2020-04-07T16:32:49.6528404Z     bb3: {
2020-04-07T16:32:49.6528559Z         StorageLive(_3);
2020-04-07T16:32:49.6528734Z         StorageLive(_4);
2020-04-07T16:32:49.6528907Z         _4 = _1;
2020-04-07T16:32:49.6528907Z         _4 = _1;
2020-04-07T16:32:49.6529089Z         FakeRead(ForMatchedPlace, _4);
2020-04-07T16:32:49.6529516Z         switchInt(_4) -> [false: bb5, otherwise: bb4];
2020-04-07T16:32:49.6529860Z     bb4: {
2020-04-07T16:32:49.6529860Z     bb4: {
2020-04-07T16:32:49.6530370Z         falseEdges -> [real: bb6, imaginary: bb5];
2020-04-07T16:32:49.6530719Z     bb5: {
2020-04-07T16:32:49.6531091Z         _3 = const ();
2020-04-07T16:32:49.6531290Z         StorageDead(_4);
2020-04-07T16:32:49.6531473Z         StorageDead(_3);
---
2020-04-07T16:32:49.6533505Z         return;
2020-04-07T16:32:49.6533634Z     }
2020-04-07T16:32:49.6533987Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6534171Z 
2020-04-07T16:32:49.6534679Z ---- [mir-opt] mir-opt/issue-49232.rs stdout ----
2020-04-07T16:32:49.6534950Z [ERROR compiletest::runtest] Some("    bb6: {")
2020-04-07T16:32:49.6535548Z thread '[mir-opt] mir-opt/issue-49232.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6535875Z Current block:     bb6: {
2020-04-07T16:32:49.6536084Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6536314Z Expected Line: "        _0 = ();"
2020-04-07T16:32:49.6536528Z Test Name: rustc.main.mir_map.0.mir
2020-04-07T16:32:49.6536841Z ... (elided)
2020-04-07T16:32:49.6537118Z fn main() -> (){
2020-04-07T16:32:49.6537274Z     let mut _0: ();
2020-04-07T16:32:49.6537440Z     let mut _1: ();
2020-04-07T16:32:49.6537440Z     let mut _1: ();
2020-04-07T16:32:49.6537701Z     let _2: i32;
2020-04-07T16:32:49.6537878Z     let mut _3: bool;
2020-04-07T16:32:49.6538051Z     let mut _4: !;
2020-04-07T16:32:49.6538226Z     let _5: ();
2020-04-07T16:32:49.6538392Z     let mut _6: &i32;
2020-04-07T16:32:49.6538555Z     scope 1 {
2020-04-07T16:32:49.6538731Z         debug beacon => _2;
2020-04-07T16:32:49.6539002Z     bb0: {
2020-04-07T16:32:49.6539318Z         goto -> bb1;
2020-04-07T16:32:49.6539469Z     }
2020-04-07T16:32:49.6539592Z     bb1: {
2020-04-07T16:32:49.6539592Z     bb1: {
2020-04-07T16:32:49.6539963Z         falseUnwind -> [real: bb3, cleanup: bb4];
2020-04-07T16:32:49.6540300Z     bb2: {
2020-04-07T16:32:49.6540586Z         goto -> bb14;
2020-04-07T16:32:49.6540737Z     }
2020-04-07T16:32:49.6540861Z     bb3: {
2020-04-07T16:32:49.6540861Z     bb3: {
2020-04-07T16:32:49.6541014Z         StorageLive(_2);
2020-04-07T16:32:49.6541199Z         StorageLive(_3);
2020-04-07T16:32:49.6541370Z         _3 = const true;
2020-04-07T16:32:49.6541567Z         FakeRead(ForMatchedPlace, _3);
2020-04-07T16:32:49.6541994Z         switchInt(_3) -> [false: bb5, otherwise: bb6];
2020-04-07T16:32:49.6542214Z     }
2020-04-07T16:32:49.6542353Z     bb4 (cleanup): {
2020-04-07T16:32:49.6542514Z         resume;
2020-04-07T16:32:49.6542784Z     bb5: {
2020-04-07T16:32:49.6542784Z     bb5: {
2020-04-07T16:32:49.6543157Z         falseEdges -> [real: bb7, imaginary: bb6];
2020-04-07T16:32:49.6543487Z     bb6: {
2020-04-07T16:32:49.6543627Z         _0 = ();
2020-04-07T16:32:49.6543915Z         goto -> bb8;
2020-04-07T16:32:49.6544066Z     }
---
2020-04-07T16:32:49.6546831Z     }
2020-04-07T16:32:49.6546958Z     bb11: {
2020-04-07T16:32:49.6547247Z         goto -> bb12;
2020-04-07T16:32:49.6547397Z     }
2020-04-07T16:32:49.6547519Z     bb12: {
2020-04-07T16:32:49.6547684Z         FakeRead(ForLet, _2);
2020-04-07T16:32:49.6547992Z         StorageDead(_3);
2020-04-07T16:32:49.6548189Z         StorageLive(_5);
2020-04-07T16:32:49.6548370Z         StorageLive(_6);
2020-04-07T16:32:49.6548541Z         _6 = &_2;
2020-04-07T16:32:49.6549061Z         _5 = const std::mem::drop::<&i32>(move _6) -> [return: bb13, unwind: bb4];
2020-04-07T16:32:49.6549470Z     bb13: {
2020-04-07T16:32:49.6549699Z         StorageDead(_6);
2020-04-07T16:32:49.6549882Z         StorageDead(_5);
2020-04-07T16:32:49.6550055Z         _1 = ();
---
2020-04-07T16:32:49.6553855Z     let mut _4: !;
2020-04-07T16:32:49.6554066Z     let _5: ();
2020-04-07T16:32:49.6554255Z     let mut _6: &i32;
2020-04-07T16:32:49.6554436Z     scope 1 {
2020-04-07T16:32:49.6554634Z         debug beacon => _2;
2020-04-07T16:32:49.6554945Z     bb0: {
2020-04-07T16:32:49.6555382Z         goto -> bb1;
2020-04-07T16:32:49.6555538Z     }
2020-04-07T16:32:49.6559447Z     bb1: {
2020-04-07T16:32:49.6559447Z     bb1: {
2020-04-07T16:32:49.6560033Z         falseUnwind -> [real: bb3, cleanup: bb4];
2020-04-07T16:32:49.6560425Z     bb2: {
2020-04-07T16:32:49.6560762Z         goto -> bb14;
2020-04-07T16:32:49.6560949Z     }
2020-04-07T16:32:49.6561090Z     bb3: {
2020-04-07T16:32:49.6561090Z     bb3: {
2020-04-07T16:32:49.6561267Z         StorageLive(_2);
2020-04-07T16:32:49.6561480Z         StorageLive(_3);
2020-04-07T16:32:49.6561676Z         _3 = const true;
2020-04-07T16:32:49.6561898Z         FakeRead(ForMatchedPlace, _3);
2020-04-07T16:32:49.6562404Z         switchInt(_3) -> [false: bb5, otherwise: bb6];
2020-04-07T16:32:49.6562763Z     }
2020-04-07T16:32:49.6563235Z     bb4 (cleanup): {
2020-04-07T16:32:49.6563420Z         resume;
2020-04-07T16:32:49.6563714Z     bb5: {
2020-04-07T16:32:49.6563714Z     bb5: {
2020-04-07T16:32:49.6564291Z         falseEdges -> [real: bb7, imaginary: bb6];
2020-04-07T16:32:49.6564647Z     bb6: {
2020-04-07T16:32:49.6564808Z         _0 = const ();
2020-04-07T16:32:49.6565152Z         goto -> bb8;
2020-04-07T16:32:49.6565296Z     }
---
2020-04-07T16:32:49.6569061Z     }
2020-04-07T16:32:49.6569204Z     bb11: {
2020-04-07T16:32:49.6569657Z         goto -> bb12;
2020-04-07T16:32:49.6569932Z     }
2020-04-07T16:32:49.6570067Z     bb12: {
2020-04-07T16:32:49.6570409Z         FakeRead(ForLet, _2);
2020-04-07T16:32:49.6570616Z         StorageDead(_3);
2020-04-07T16:32:49.6570820Z         StorageLive(_5);
2020-04-07T16:32:49.6571008Z         StorageLive(_6);
2020-04-07T16:32:49.6571184Z         _6 = &_2;
2020-04-07T16:32:49.6571726Z         _5 = const std::mem::drop::<&i32>(move _6) -> [return: bb13, unwind: bb4];
2020-04-07T16:32:49.6572276Z     bb13: {
2020-04-07T16:32:49.6572464Z         StorageDead(_6);
2020-04-07T16:32:49.6572653Z         StorageDead(_5);
2020-04-07T16:32:49.6572838Z         _1 = const ();
---
2020-04-07T16:32:49.6574136Z         return;
2020-04-07T16:32:49.6574278Z     }
2020-04-07T16:32:49.6574672Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6574874Z 
2020-04-07T16:32:49.6575245Z ---- [mir-opt] mir-opt/loop_test.rs stdout ----
2020-04-07T16:32:49.6575652Z [ERROR compiletest::runtest] None
2020-04-07T16:32:49.6576354Z thread '[mir-opt] mir-opt/loop_test.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6576683Z Current block: None
2020-04-07T16:32:49.6577054Z Actual Line: "        _1 = const ();"
2020-04-07T16:32:49.6577299Z Expected Line: "       _1 = ();"
2020-04-07T16:32:49.6577781Z Test Name: rustc.main.SimplifyCfg-qualify-consts.after.mir
2020-04-07T16:32:49.6578135Z ... (elided)
2020-04-07T16:32:49.6578296Z ... (elided)
2020-04-07T16:32:49.6578296Z ... (elided)
2020-04-07T16:32:49.6578460Z    bb1 (cleanup): {
2020-04-07T16:32:49.6578628Z        resume;
2020-04-07T16:32:49.6578909Z ... (elided)
2020-04-07T16:32:49.6578909Z ... (elided)
2020-04-07T16:32:49.6579090Z    bb3: { // Entry into the loop
2020-04-07T16:32:49.6579457Z        StorageDead(_2);
2020-04-07T16:32:49.6579638Z        StorageDead(_1);
2020-04-07T16:32:49.6579817Z        StorageLive(_4);
2020-04-07T16:32:49.6580247Z        goto -> bb5;
2020-04-07T16:32:49.6580247Z        goto -> bb5;
2020-04-07T16:32:49.6580526Z    }
2020-04-07T16:32:49.6580659Z ... (elided)
2020-04-07T16:32:49.6580849Z    bb5: { // The loop_block
2020-04-07T16:32:49.6581314Z        falseUnwind -> [real: bb6, cleanup: bb1];
2020-04-07T16:32:49.6581534Z    }
2020-04-07T16:32:49.6581835Z    bb6: { // The loop body (body_block)
2020-04-07T16:32:49.6582056Z        StorageLive(_6);
2020-04-07T16:32:49.6582241Z        _6 = const 1i32;
2020-04-07T16:32:49.6582451Z        FakeRead(ForLet, _6);
2020-04-07T16:32:49.6582647Z        StorageDead(_6);
2020-04-07T16:32:49.6583000Z        goto -> bb5;
2020-04-07T16:32:49.6583396Z ... (elided)
2020-04-07T16:32:49.6583652Z Actual:
2020-04-07T16:32:49.6583944Z fn main() -> () {
2020-04-07T16:32:49.6584136Z     let mut _0: ();
---
2020-04-07T16:32:49.6585957Z     bb0: {
2020-04-07T16:32:49.6586125Z         StorageLive(_1);
2020-04-07T16:32:49.6586307Z         StorageLive(_2);
2020-04-07T16:32:49.6586628Z         _2 = const true;
2020-04-07T16:32:49.6586842Z         FakeRead(ForMatchedPlace, _2);
2020-04-07T16:32:49.6587320Z         switchInt(_2) -> [false: bb3, otherwise: bb2];
2020-04-07T16:32:49.6587562Z     }
2020-04-07T16:32:49.6587718Z     bb1 (cleanup): {
2020-04-07T16:32:49.6588013Z         resume;
2020-04-07T16:32:49.6588313Z     bb2: {
2020-04-07T16:32:49.6588313Z     bb2: {
2020-04-07T16:32:49.6588741Z         falseEdges -> [real: bb4, imaginary: bb3];
2020-04-07T16:32:49.6589125Z     bb3: {
2020-04-07T16:32:49.6589298Z         _1 = const ();
2020-04-07T16:32:49.6589496Z         StorageDead(_2);
2020-04-07T16:32:49.6589707Z         StorageDead(_1);
---
2020-04-07T16:32:49.6590918Z         StorageDead(_2);
2020-04-07T16:32:49.6591246Z         StorageDead(_1);
2020-04-07T16:32:49.6591413Z         return;
2020-04-07T16:32:49.6591552Z     }
2020-04-07T16:32:49.6591703Z     bb5: {
2020-04-07T16:32:49.6592118Z         falseUnwind -> [real: bb6, cleanup: bb1];
2020-04-07T16:32:49.6592489Z     bb6: {
2020-04-07T16:32:49.6592659Z         StorageLive(_6);
2020-04-07T16:32:49.6592847Z         _6 = const 1i32;
2020-04-07T16:32:49.6592847Z         _6 = const 1i32;
2020-04-07T16:32:49.6593044Z         FakeRead(ForLet, _6);
2020-04-07T16:32:49.6593257Z         StorageDead(_6);
2020-04-07T16:32:49.6593589Z         goto -> bb5;
2020-04-07T16:32:49.6594244Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6594417Z 
2020-04-07T16:32:49.6594780Z ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
2020-04-07T16:32:49.6594780Z ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
2020-04-07T16:32:49.6595202Z [ERROR compiletest::runtest] Some(" bb11: {")
2020-04-07T16:32:49.6595992Z thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6596361Z Current block:  bb11: {
2020-04-07T16:32:49.6596604Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6596839Z Expected Line: "     _0 = ();"
2020-04-07T16:32:49.6597105Z Test Name: rustc.full_tested_match.PromoteTemps.after.mir
2020-04-07T16:32:49.6597489Z ... (elided)
2020-04-07T16:32:49.6597633Z  bb0: {
2020-04-07T16:32:49.6597777Z ... (elided)
2020-04-07T16:32:49.6597777Z ... (elided)
2020-04-07T16:32:49.6598040Z      _2 = std::option::Option::<i32>::Some(const 42i32,);
2020-04-07T16:32:49.6598325Z      FakeRead(ForMatchedPlace, _2);
2020-04-07T16:32:49.6598538Z      _3 = discriminant(_2);
2020-04-07T16:32:49.6599239Z      switchInt(move _3) -> [0isize: bb2, 1isize: bb3, otherwise: bb5];
2020-04-07T16:32:49.6599507Z  }
2020-04-07T16:32:49.6599651Z  bb1 (cleanup): {
2020-04-07T16:32:49.6599828Z      resume;
2020-04-07T16:32:49.6599955Z  }
2020-04-07T16:32:49.6600126Z  bb2: {  // pre_binding3 and arm3
2020-04-07T16:32:49.6600347Z      _1 = (const 3i32, const 3i32);
2020-04-07T16:32:49.6600733Z      goto -> bb11;
2020-04-07T16:32:49.6600989Z  bb3: {
2020-04-07T16:32:49.6600989Z  bb3: {
2020-04-07T16:32:49.6601424Z      falseEdges -> [real: bb6, imaginary: bb4]; //pre_binding1
2020-04-07T16:32:49.6601773Z  bb4: {
2020-04-07T16:32:49.6601773Z  bb4: {
2020-04-07T16:32:49.6602207Z      falseEdges -> [real: bb10, imaginary: bb2]; //pre_binding2
2020-04-07T16:32:49.6602554Z  bb5: {
2020-04-07T16:32:49.6602715Z      unreachable;
2020-04-07T16:32:49.6602849Z  }
2020-04-07T16:32:49.6602849Z  }
2020-04-07T16:32:49.6603295Z  bb6: { // binding1 and guard
2020-04-07T16:32:49.6603482Z      StorageLive(_6);
2020-04-07T16:32:49.6603736Z      _11 = const full_tested_match::promoted[0];
2020-04-07T16:32:49.6604011Z      _6 = &(((*_11) as Some).0: i32);
2020-04-07T16:32:49.6604227Z      _4 = &shallow _2;
2020-04-07T16:32:49.6604421Z      StorageLive(_7);
2020-04-07T16:32:49.6604895Z      _7 = const guard() -> [return: bb7, unwind: bb1];
2020-04-07T16:32:49.6605116Z  }
2020-04-07T16:32:49.6605280Z  bb7: { // end of guard
2020-04-07T16:32:49.6605718Z      switchInt(move _7) -> [false: bb9, otherwise: bb8];
2020-04-07T16:32:49.6605943Z  }
2020-04-07T16:32:49.6606094Z  bb8: { // arm1
2020-04-07T16:32:49.6606262Z      StorageDead(_7);
2020-04-07T16:32:49.6606454Z      FakeRead(ForMatchGuard, _4);
2020-04-07T16:32:49.6606685Z      FakeRead(ForGuardBinding, _6);
2020-04-07T16:32:49.6606882Z      StorageLive(_5);
2020-04-07T16:32:49.6607086Z      _5 = ((_2 as Some).0: i32);
2020-04-07T16:32:49.6607289Z      StorageLive(_8);
2020-04-07T16:32:49.6607465Z      _8 = _5;
2020-04-07T16:32:49.6607644Z      _1 = (const 1i32, move _8);
2020-04-07T16:32:49.6607840Z      StorageDead(_8);
2020-04-07T16:32:49.6608027Z      StorageDead(_5);
2020-04-07T16:32:49.6608198Z      StorageDead(_6);
2020-04-07T16:32:49.6608511Z      goto -> bb11;
2020-04-07T16:32:49.6608662Z  }
2020-04-07T16:32:49.6608815Z  bb9: { // to pre_binding2
2020-04-07T16:32:49.6609005Z      StorageDead(_7);
2020-04-07T16:32:49.6609176Z      StorageDead(_6);
2020-04-07T16:32:49.6609501Z      goto -> bb4;
2020-04-07T16:32:49.6609635Z  }
2020-04-07T16:32:49.6609768Z  bb10: { // arm2
2020-04-07T16:32:49.6609947Z      StorageLive(_9);
2020-04-07T16:32:49.6610150Z      _9 = ((_2 as Some).0: i32);
2020-04-07T16:32:49.6610355Z      StorageLive(_10);
2020-04-07T16:32:49.6610533Z      _10 = _9;
2020-04-07T16:32:49.6610717Z      _1 = (const 2i32, move _10);
2020-04-07T16:32:49.6610911Z      StorageDead(_10);
2020-04-07T16:32:49.6611099Z      StorageDead(_9);
2020-04-07T16:32:49.6611413Z      goto -> bb11;
2020-04-07T16:32:49.6611668Z  bb11: {
2020-04-07T16:32:49.6611838Z      StorageDead(_2);
2020-04-07T16:32:49.6612014Z      StorageDead(_1);
2020-04-07T16:32:49.6612174Z      _0 = ();
---
2020-04-07T16:32:49.6617200Z     }
2020-04-07T16:32:49.6617340Z     scope 3 {
2020-04-07T16:32:49.6617524Z         debug y => _9;
2020-04-07T16:32:49.6617682Z     }
2020-04-07T16:32:49.6617820Z     bb0: {
2020-04-07T16:32:49.6618115Z         StorageLive(_1);
2020-04-07T16:32:49.6618299Z         StorageLive(_2);
2020-04-07T16:32:49.6618562Z         _2 = std::option::Option::<i32>::Some(const 42i32,);
2020-04-07T16:32:49.6618864Z         FakeRead(ForMatchedPlace, _2);
2020-04-07T16:32:49.6619080Z         _3 = discriminant(_2);
2020-04-07T16:32:49.6619635Z         switchInt(move _3) -> [0isize: bb2, 1isize: bb3, otherwise: bb5];
2020-04-07T16:32:49.6619919Z     }
2020-04-07T16:32:49.6620075Z     bb1 (cleanup): {
2020-04-07T16:32:49.6620246Z         resume;
2020-04-07T16:32:49.6620626Z     bb2: {
2020-04-07T16:32:49.6620808Z         _1 = (const 3i32, const 3i32);
2020-04-07T16:32:49.6621140Z         goto -> bb11;
2020-04-07T16:32:49.6621413Z     }
2020-04-07T16:32:49.6621413Z     }
2020-04-07T16:32:49.6621545Z     bb3: {
2020-04-07T16:32:49.6621943Z         falseEdges -> [real: bb6, imaginary: bb4];
2020-04-07T16:32:49.6622300Z     bb4: {
2020-04-07T16:32:49.6622300Z     bb4: {
2020-04-07T16:32:49.6622698Z         falseEdges -> [real: bb10, imaginary: bb2];
2020-04-07T16:32:49.6623054Z     bb5: {
2020-04-07T16:32:49.6623211Z         unreachable;
2020-04-07T16:32:49.6623354Z     }
2020-04-07T16:32:49.6623500Z     bb6: {
2020-04-07T16:32:49.6623500Z     bb6: {
2020-04-07T16:32:49.6623665Z         StorageLive(_6);
2020-04-07T16:32:49.6624157Z         _11 = const full_tested_match::promoted[0];
2020-04-07T16:32:49.6624481Z         _6 = &(((*_11) as Some).0: i32);
2020-04-07T16:32:49.6624726Z         _4 = &shallow _2;
2020-04-07T16:32:49.6624924Z         StorageLive(_7);
2020-04-07T16:32:49.6625413Z         _7 = const guard() -> [return: bb7, unwind: bb1];
2020-04-07T16:32:49.6625807Z     bb7: {
2020-04-07T16:32:49.6625807Z     bb7: {
2020-04-07T16:32:49.6626265Z         switchInt(move _7) -> [false: bb9, otherwise: bb8];
2020-04-07T16:32:49.6626664Z     bb8: {
2020-04-07T16:32:49.6626834Z         StorageDead(_7);
2020-04-07T16:32:49.6627067Z         FakeRead(ForMatchGuard, _4);
2020-04-07T16:32:49.6627067Z         FakeRead(ForMatchGuard, _4);
2020-04-07T16:32:49.6627302Z         FakeRead(ForGuardBinding, _6);
2020-04-07T16:32:49.6627516Z         StorageLive(_5);
2020-04-07T16:32:49.6627888Z         _5 = ((_2 as Some).0: i32);
2020-04-07T16:32:49.6628221Z         StorageLive(_8);
2020-04-07T16:32:49.6628495Z         _8 = _5;
2020-04-07T16:32:49.6629120Z         _1 = (const 1i32, move _8);
2020-04-07T16:32:49.6629330Z         StorageDead(_8);
2020-04-07T16:32:49.6629512Z         StorageDead(_5);
2020-04-07T16:32:49.6629693Z         StorageDead(_6);
2020-04-07T16:32:49.6630095Z         goto -> bb11;
2020-04-07T16:32:49.6630377Z     bb9: {
2020-04-07T16:32:49.6630557Z         StorageDead(_7);
2020-04-07T16:32:49.6630746Z         StorageDead(_6);
2020-04-07T16:32:49.6631071Z         goto -> bb4;
2020-04-07T16:32:49.6631071Z         goto -> bb4;
2020-04-07T16:32:49.6631228Z     }
2020-04-07T16:32:49.6631362Z     bb10: {
2020-04-07T16:32:49.6631527Z         StorageLive(_9);
2020-04-07T16:32:49.6631886Z         _9 = ((_2 as Some).0: i32);
2020-04-07T16:32:49.6632105Z         StorageLive(_10);
2020-04-07T16:32:49.6632386Z         _10 = _9;
2020-04-07T16:32:49.6632566Z         _1 = (const 2i32, move _10);
2020-04-07T16:32:49.6632771Z         StorageDead(_10);
2020-04-07T16:32:49.6632943Z         StorageDead(_9);
2020-04-07T16:32:49.6633283Z         goto -> bb11;
2020-04-07T16:32:49.6633562Z     bb11: {
2020-04-07T16:32:49.6633718Z         StorageDead(_2);
2020-04-07T16:32:49.6633902Z         StorageDead(_1);
2020-04-07T16:32:49.6634070Z         _0 = const ();
2020-04-07T16:32:49.6634070Z         _0 = const ();
2020-04-07T16:32:49.6634226Z         return;
2020-04-07T16:32:49.6634370Z     }
2020-04-07T16:32:49.6634719Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6634956Z 
2020-04-07T16:32:49.6635310Z ---- [mir-opt] mir-opt/match_test.rs stdout ----
2020-04-07T16:32:49.6635583Z [ERROR compiletest::runtest] Some("   bb14: {")
2020-04-07T16:32:49.6637212Z thread '[mir-opt] mir-opt/match_test.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6637588Z Current block:    bb14: {
2020-04-07T16:32:49.6637830Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6638061Z Expected Line: "       _0 = ();"
2020-04-07T16:32:49.6638493Z Test Name: rustc.main.SimplifyCfg-initial.after.mir
2020-04-07T16:32:49.6638859Z ... (elided)
2020-04-07T16:32:49.6639003Z    bb0: {
2020-04-07T16:32:49.6639145Z ... (elided)
2020-04-07T16:32:49.6639145Z ... (elided)
2020-04-07T16:32:49.6639589Z        switchInt(move _6) -> [false: bb4, otherwise: bb1];
2020-04-07T16:32:49.6640029Z    bb1: {
2020-04-07T16:32:49.6640029Z    bb1: {
2020-04-07T16:32:49.6640208Z        _7 = Lt(_1, const 10i32);
2020-04-07T16:32:49.6640641Z        switchInt(move _7) -> [false: bb4, otherwise: bb2];
2020-04-07T16:32:49.6640996Z    bb2: {
2020-04-07T16:32:49.6640996Z    bb2: {
2020-04-07T16:32:49.6641367Z        falseEdges -> [real: bb9, imaginary: bb6];
2020-04-07T16:32:49.6641690Z    bb3: {
2020-04-07T16:32:49.6641844Z        _3 = const 3i32;
2020-04-07T16:32:49.6642148Z        goto -> bb14;
2020-04-07T16:32:49.6642280Z    }
2020-04-07T16:32:49.6642280Z    }
2020-04-07T16:32:49.6642416Z    bb4: {
2020-04-07T16:32:49.6642582Z        _4 = Le(const 10i32, _1);
2020-04-07T16:32:49.6643258Z        switchInt(move _4) -> [false: bb7, otherwise: bb5];
2020-04-07T16:32:49.6643613Z    bb5: {
2020-04-07T16:32:49.6643613Z    bb5: {
2020-04-07T16:32:49.6643773Z        _5 = Le(_1, const 20i32);
2020-04-07T16:32:49.6644269Z        switchInt(move _5) -> [false: bb7, otherwise: bb6];
2020-04-07T16:32:49.6644601Z    bb6: {
2020-04-07T16:32:49.6644601Z    bb6: {
2020-04-07T16:32:49.6644989Z        falseEdges -> [real: bb12, imaginary: bb8];
2020-04-07T16:32:49.6645300Z    bb7: {
2020-04-07T16:32:49.6645300Z    bb7: {
2020-04-07T16:32:49.6645684Z        switchInt(_1) -> [-1i32: bb8, otherwise: bb3];
2020-04-07T16:32:49.6646023Z    bb8: {
2020-04-07T16:32:49.6646023Z    bb8: {
2020-04-07T16:32:49.6646394Z        falseEdges -> [real: bb13, imaginary: bb3];
2020-04-07T16:32:49.6646725Z    bb9: {
2020-04-07T16:32:49.6646874Z        _8 = &shallow _1;
2020-04-07T16:32:49.6647059Z        StorageLive(_9);
2020-04-07T16:32:49.6647215Z        _9 = _2;
2020-04-07T16:32:49.6647215Z        _9 = _2;
2020-04-07T16:32:49.6647622Z        switchInt(move _9) -> [false: bb11, otherwise: bb10];
2020-04-07T16:32:49.6647971Z    bb10: {
2020-04-07T16:32:49.6648124Z        StorageDead(_9);
2020-04-07T16:32:49.6648310Z        FakeRead(ForMatchGuard, _8);
2020-04-07T16:32:49.6648513Z        _3 = const 0i32;
2020-04-07T16:32:49.6648513Z        _3 = const 0i32;
2020-04-07T16:32:49.6648814Z        goto -> bb14;
2020-04-07T16:32:49.6648946Z    }
2020-04-07T16:32:49.6649081Z    bb11: {
2020-04-07T16:32:49.6649233Z        StorageDead(_9);
2020-04-07T16:32:49.6649621Z        falseEdges -> [real: bb3, imaginary: bb6];
2020-04-07T16:32:49.6649949Z    bb12: {
2020-04-07T16:32:49.6650095Z        _3 = const 1i32;
2020-04-07T16:32:49.6650409Z        goto -> bb14;
2020-04-07T16:32:49.6650542Z    }
---
2020-04-07T16:32:49.6655726Z         scope 2 {
2020-04-07T16:32:49.6655890Z             debug b => _2;
2020-04-07T16:32:49.6656042Z         }
2020-04-07T16:32:49.6656175Z     }
2020-04-07T16:32:49.6656299Z     bb0: {
2020-04-07T16:32:49.6656453Z         StorageLive(_1);
2020-04-07T16:32:49.6656640Z         _1 = const 3i32;
2020-04-07T16:32:49.6656820Z         FakeRead(ForLet, _1);
2020-04-07T16:32:49.6656998Z         StorageLive(_2);
2020-04-07T16:32:49.6657169Z         _2 = const true;
2020-04-07T16:32:49.6657360Z         FakeRead(ForLet, _2);
2020-04-07T16:32:49.6657539Z         StorageLive(_3);
2020-04-07T16:32:49.6657735Z         FakeRead(ForMatchedPlace, _1);
2020-04-07T16:32:49.6657954Z         _6 = Le(const 0i32, _1);
2020-04-07T16:32:49.6658424Z         switchInt(move _6) -> [false: bb4, otherwise: bb1];
2020-04-07T16:32:49.6658781Z     bb1: {
2020-04-07T16:32:49.6658781Z     bb1: {
2020-04-07T16:32:49.6658946Z         _7 = Lt(_1, const 10i32);
2020-04-07T16:32:49.6659378Z         switchInt(move _7) -> [false: bb4, otherwise: bb2];
2020-04-07T16:32:49.6659866Z     bb2: {
2020-04-07T16:32:49.6659866Z     bb2: {
2020-04-07T16:32:49.6660264Z         falseEdges -> [real: bb9, imaginary: bb6];
2020-04-07T16:32:49.6660627Z     bb3: {
2020-04-07T16:32:49.6660791Z         _3 = const 3i32;
2020-04-07T16:32:49.6661116Z         goto -> bb14;
2020-04-07T16:32:49.6661277Z     }
2020-04-07T16:32:49.6661277Z     }
2020-04-07T16:32:49.6661405Z     bb4: {
2020-04-07T16:32:49.6661580Z         _4 = Le(const 10i32, _1);
2020-04-07T16:32:49.6662053Z         switchInt(move _4) -> [false: bb7, otherwise: bb5];
2020-04-07T16:32:49.6662421Z     bb5: {
2020-04-07T16:32:49.6662421Z     bb5: {
2020-04-07T16:32:49.6662615Z         _5 = Le(_1, const 20i32);
2020-04-07T16:32:49.6663171Z         switchInt(move _5) -> [false: bb7, otherwise: bb6];
2020-04-07T16:32:49.6663504Z     bb6: {
2020-04-07T16:32:49.6663504Z     bb6: {
2020-04-07T16:32:49.6663900Z         falseEdges -> [real: bb12, imaginary: bb8];
2020-04-07T16:32:49.6664221Z     bb7: {
2020-04-07T16:32:49.6664221Z     bb7: {
2020-04-07T16:32:49.6664620Z         switchInt(_1) -> [-1i32: bb8, otherwise: bb3];
2020-04-07T16:32:49.6664951Z     bb8: {
2020-04-07T16:32:49.6664951Z     bb8: {
2020-04-07T16:32:49.6665338Z         falseEdges -> [real: bb13, imaginary: bb3];
2020-04-07T16:32:49.6665664Z     bb9: {
2020-04-07T16:32:49.6665834Z         _8 = &shallow _1;
2020-04-07T16:32:49.6666007Z         StorageLive(_9);
2020-04-07T16:32:49.6666165Z         _9 = _2;
2020-04-07T16:32:49.6666165Z         _9 = _2;
2020-04-07T16:32:49.6666571Z         switchInt(move _9) -> [false: bb11, otherwise: bb10];
2020-04-07T16:32:49.6666932Z     bb10: {
2020-04-07T16:32:49.6667086Z         StorageDead(_9);
2020-04-07T16:32:49.6667292Z         FakeRead(ForMatchGuard, _8);
2020-04-07T16:32:49.6667481Z         _3 = const 0i32;
2020-04-07T16:32:49.6667481Z         _3 = const 0i32;
2020-04-07T16:32:49.6667785Z         goto -> bb14;
2020-04-07T16:32:49.6668026Z     }
2020-04-07T16:32:49.6668149Z     bb11: {
2020-04-07T16:32:49.6668302Z         StorageDead(_9);
2020-04-07T16:32:49.6668736Z         falseEdges -> [real: bb3, imaginary: bb6];
2020-04-07T16:32:49.6669061Z     bb12: {
2020-04-07T16:32:49.6669217Z         _3 = const 1i32;
2020-04-07T16:32:49.6669538Z         goto -> bb14;
2020-04-07T16:32:49.6669672Z     }
---
2020-04-07T16:32:49.6671447Z         return;
2020-04-07T16:32:49.6671576Z     }
2020-04-07T16:32:49.6671966Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6672134Z 
2020-04-07T16:32:49.6672496Z ---- [mir-opt] mir-opt/packed-struct-drop-aligned.rs stdout ----
2020-04-07T16:32:49.6672792Z [ERROR compiletest::runtest] Some("    bb4: {")
2020-04-07T16:32:49.6673407Z thread '[mir-opt] mir-opt/packed-struct-drop-aligned.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6673766Z Current block:     bb4: {
2020-04-07T16:32:49.6673991Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6674208Z Expected Line: "        _0 = ();"
2020-04-07T16:32:49.6674620Z Test Name: rustc.main.SimplifyCfg-elaborate-drops.after.mir
2020-04-07T16:32:49.6674960Z ... (elided)
2020-04-07T16:32:49.6675231Z fn main() -> () {
2020-04-07T16:32:49.6675393Z     let mut _0: ();
2020-04-07T16:32:49.6675581Z     let mut _1: Packed;
---
2020-04-07T16:32:49.6676511Z     let mut _6: Aligned;
2020-04-07T16:32:49.6676686Z     scope 1 {
2020-04-07T16:32:49.6676840Z         debug x => _1;
2020-04-07T16:32:49.6676978Z     }
2020-04-07T16:32:49.6677102Z     bb0: {
2020-04-07T16:32:49.6677270Z         StorageLive(_1);
2020-04-07T16:32:49.6677423Z ... (elided)
2020-04-07T16:32:49.6677585Z         _1 = Packed(move _2,);
2020-04-07T16:32:49.6677762Z ... (elided)
2020-04-07T16:32:49.6677915Z         StorageLive(_6);
2020-04-07T16:32:49.6678115Z         _6 = move (_1.0: Aligned);
2020-04-07T16:32:49.6678549Z         drop(_6) -> [return: bb4, unwind: bb3];
2020-04-07T16:32:49.6678742Z     }
2020-04-07T16:32:49.6678882Z     bb1 (cleanup): {
2020-04-07T16:32:49.6679055Z         resume;
2020-04-07T16:32:49.6679301Z     bb2: {
2020-04-07T16:32:49.6679454Z         StorageDead(_1);
2020-04-07T16:32:49.6679626Z         return;
2020-04-07T16:32:49.6679753Z     }
2020-04-07T16:32:49.6679753Z     }
2020-04-07T16:32:49.6679901Z     bb3 (cleanup): {
2020-04-07T16:32:49.6680112Z         (_1.0: Aligned) = move _4;
2020-04-07T16:32:49.6680450Z         drop(_1) -> bb1;
2020-04-07T16:32:49.6680732Z     bb4: {
2020-04-07T16:32:49.6680886Z         StorageDead(_6);
2020-04-07T16:32:49.6680886Z         StorageDead(_6);
2020-04-07T16:32:49.6681080Z         (_1.0: Aligned) = move _4;
2020-04-07T16:32:49.6681271Z         StorageDead(_4);
2020-04-07T16:32:49.6681444Z         _0 = ();
2020-04-07T16:32:49.6681815Z         drop(_1) -> [return: bb2, unwind: bb1];
2020-04-07T16:32:49.6682127Z }
2020-04-07T16:32:49.6682237Z Actual:
2020-04-07T16:32:49.6682503Z fn main() -> () {
2020-04-07T16:32:49.6682675Z     let mut _0: ();
---
2020-04-07T16:32:49.6683951Z     let mut _6: Aligned;
2020-04-07T16:32:49.6684244Z     scope 1 {
2020-04-07T16:32:49.6684398Z         debug x => _1;
2020-04-07T16:32:49.6684537Z     }
2020-04-07T16:32:49.6684675Z     bb0: {
2020-04-07T16:32:49.6684829Z         StorageLive(_1);
2020-04-07T16:32:49.6685000Z         StorageLive(_2);
2020-04-07T16:32:49.6685184Z         StorageLive(_3);
2020-04-07T16:32:49.6685373Z         _3 = Droppy(const 0usize,);
2020-04-07T16:32:49.6685573Z         _2 = Aligned(move _3,);
2020-04-07T16:32:49.6685769Z         StorageDead(_3);
2020-04-07T16:32:49.6685949Z         _1 = Packed(move _2,);
2020-04-07T16:32:49.6686128Z         StorageDead(_2);
2020-04-07T16:32:49.6686297Z         StorageLive(_4);
2020-04-07T16:32:49.6686481Z         StorageLive(_5);
2020-04-07T16:32:49.6686725Z         _5 = Droppy(const 0usize,);
2020-04-07T16:32:49.6686932Z         _4 = Aligned(move _5,);
2020-04-07T16:32:49.6687131Z         StorageDead(_5);
2020-04-07T16:32:49.6687301Z         StorageLive(_6);
2020-04-07T16:32:49.6687498Z         _6 = move (_1.0: Aligned);
2020-04-07T16:32:49.6688133Z         drop(_6) -> [return: bb4, unwind: bb3];
2020-04-07T16:32:49.6688339Z     }
2020-04-07T16:32:49.6688492Z     bb1 (cleanup): {
2020-04-07T16:32:49.6688677Z         resume;
2020-04-07T16:32:49.6688947Z     bb2: {
2020-04-07T16:32:49.6689112Z         StorageDead(_1);
2020-04-07T16:32:49.6689294Z         return;
2020-04-07T16:32:49.6689431Z     }
2020-04-07T16:32:49.6689431Z     }
2020-04-07T16:32:49.6689584Z     bb3 (cleanup): {
2020-04-07T16:32:49.6689809Z         (_1.0: Aligned) = move _4;
2020-04-07T16:32:49.6690166Z         drop(_1) -> bb1;
2020-04-07T16:32:49.6690468Z     bb4: {
2020-04-07T16:32:49.6690633Z         StorageDead(_6);
2020-04-07T16:32:49.6690633Z         StorageDead(_6);
2020-04-07T16:32:49.6690842Z         (_1.0: Aligned) = move _4;
2020-04-07T16:32:49.6691047Z         StorageDead(_4);
2020-04-07T16:32:49.6691241Z         _0 = const ();
2020-04-07T16:32:49.6691742Z         drop(_1) -> [return: bb2, unwind: bb1];
2020-04-07T16:32:49.6692292Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6692453Z 
2020-04-07T16:32:49.6692815Z ---- [mir-opt] mir-opt/no-spurious-drop-after-call.rs stdout ----
2020-04-07T16:32:49.6692815Z ---- [mir-opt] mir-opt/no-spurious-drop-after-call.rs stdout ----
2020-04-07T16:32:49.6693110Z [ERROR compiletest::runtest] Some("   bb3: {")
2020-04-07T16:32:49.6693717Z thread '[mir-opt] mir-opt/no-spurious-drop-after-call.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6694077Z Current block:    bb3: {
2020-04-07T16:32:49.6694299Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6694513Z Expected Line: "       _0 = ();"
2020-04-07T16:32:49.6694741Z Test Name: rustc.main.ElaborateDrops.before.mir
2020-04-07T16:32:49.6695075Z ... (elided)
2020-04-07T16:32:49.6695214Z    bb2: {
2020-04-07T16:32:49.6695366Z        StorageDead(_3);
2020-04-07T16:32:49.6695366Z        StorageDead(_3);
2020-04-07T16:32:49.6695896Z        _1 = const std::mem::drop::<std::string::String>(move _2) -> [return: bb3, unwind: bb4];
2020-04-07T16:32:49.6696313Z    bb3: {
2020-04-07T16:32:49.6696478Z        StorageDead(_2);
2020-04-07T16:32:49.6696646Z        StorageDead(_4);
2020-04-07T16:32:49.6696814Z        StorageDead(_1);
---
2020-04-07T16:32:49.6697652Z fn main() -> () {
2020-04-07T16:32:49.6697935Z     let mut _0: ();
2020-04-07T16:32:49.6698107Z     let _1: ();
2020-04-07T16:32:49.6698332Z     let mut _2: std::string::String;
2020-04-07T16:32:49.6699614Z     let mut _3: &str;
2020-04-07T16:32:49.6699817Z     let _4: &str;
2020-04-07T16:32:49.6699997Z     bb0: {
2020-04-07T16:32:49.6700161Z         StorageLive(_1);
2020-04-07T16:32:49.6700355Z         StorageLive(_2);
2020-04-07T16:32:49.6700552Z         StorageLive(_3);
2020-04-07T16:32:49.6700734Z         StorageLive(_4);
2020-04-07T16:32:49.6701018Z         _4 = const "";
2020-04-07T16:32:49.6701183Z         _3 = &(*_4);
2020-04-07T16:32:49.6702089Z         _2 = const <str as std::string::ToString>::to_string(move _3) -> bb2;
2020-04-07T16:32:49.6702355Z     }
2020-04-07T16:32:49.6702509Z     bb1 (cleanup): {
2020-04-07T16:32:49.6702697Z         resume;
2020-04-07T16:32:49.6702966Z     bb2: {
2020-04-07T16:32:49.6703146Z         StorageDead(_3);
2020-04-07T16:32:49.6703146Z         StorageDead(_3);
2020-04-07T16:32:49.6703700Z         _1 = const std::mem::drop::<std::string::String>(move _2) -> [return: bb3, unwind: bb4];
2020-04-07T16:32:49.6704157Z     bb3: {
2020-04-07T16:32:49.6704322Z         StorageDead(_2);
2020-04-07T16:32:49.6704504Z         StorageDead(_4);
2020-04-07T16:32:49.6704685Z         StorageDead(_1);
2020-04-07T16:32:49.6704685Z         StorageDead(_1);
2020-04-07T16:32:49.6704880Z         _0 = const ();
2020-04-07T16:32:49.6705238Z         return;
2020-04-07T16:32:49.6705376Z     }
2020-04-07T16:32:49.6705535Z     bb4 (cleanup): {
2020-04-07T16:32:49.6705883Z         drop(_2) -> bb1;
2020-04-07T16:32:49.6706379Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6706549Z 
2020-04-07T16:32:49.6706937Z ---- [mir-opt] mir-opt/simplify-locals-removes-unused-consts.rs stdout ----
2020-04-07T16:32:49.6706937Z ---- [mir-opt] mir-opt/simplify-locals-removes-unused-consts.rs stdout ----
2020-04-07T16:32:49.6707233Z [ERROR compiletest::runtest] Some("bb2: {")
2020-04-07T16:32:49.6708010Z thread '[mir-opt] mir-opt/simplify-locals-removes-unused-consts.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6708401Z Current block: bb2: {
2020-04-07T16:32:49.6708615Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6708973Z Expected Line: "  return;"
2020-04-07T16:32:49.6710757Z Test Name: rustc.main.SimplifyLocals.before.mir
2020-04-07T16:32:49.6711172Z ... (elided)
2020-04-07T16:32:49.6711349Z let mut _0: ();
2020-04-07T16:32:49.6711537Z let mut _1: ((), ());
2020-04-07T16:32:49.6711740Z let mut _2: ();
---
2020-04-07T16:32:49.6713408Z let mut _10: u8;
2020-04-07T16:32:49.6713684Z let mut _11: Temp;
2020-04-07T16:32:49.6713843Z scope 1 {
2020-04-07T16:32:49.6713954Z }
2020-04-07T16:32:49.6714065Z bb0: {
2020-04-07T16:32:49.6714216Z   StorageLive(_1);
2020-04-07T16:32:49.6714368Z   StorageLive(_2);
2020-04-07T16:32:49.6714517Z   _2 = const ();
2020-04-07T16:32:49.6714667Z   StorageLive(_3);
2020-04-07T16:32:49.6714830Z   _3 = const ();
2020-04-07T16:32:49.6715026Z   _1 = const {transmute(()): ((), ())};
2020-04-07T16:32:49.6715222Z   StorageDead(_3);
2020-04-07T16:32:49.6715387Z   StorageDead(_2);
2020-04-07T16:32:49.6715669Z   StorageDead(_1);
2020-04-07T16:32:49.6715831Z   StorageLive(_4);
2020-04-07T16:32:49.6716005Z   StorageLive(_6);
2020-04-07T16:32:49.6716164Z   _6 = const ();
2020-04-07T16:32:49.6716322Z   StorageLive(_7);
2020-04-07T16:32:49.6716499Z   _7 = const ();
2020-04-07T16:32:49.6716658Z   StorageDead(_7);
2020-04-07T16:32:49.6716819Z   StorageDead(_6);
2020-04-07T16:32:49.6717516Z   _4 = const use_zst(const {transmute(()): ((), ())}) -> bb1;
2020-04-07T16:32:49.6718009Z bb1: {
2020-04-07T16:32:49.6718162Z   StorageDead(_4);
2020-04-07T16:32:49.6718350Z   StorageLive(_8);
2020-04-07T16:32:49.6718526Z   StorageLive(_10);
2020-04-07T16:32:49.6718526Z   StorageLive(_10);
2020-04-07T16:32:49.6718705Z   StorageLive(_11);
2020-04-07T16:32:49.6718952Z   _11 = const {transmute(0x28) : Temp};
2020-04-07T16:32:49.6719181Z   _10 = const 40u8;
2020-04-07T16:32:49.6719358Z   StorageDead(_10);
2020-04-07T16:32:49.6719883Z   _8 = const use_u8(const 42u8) -> bb2;
2020-04-07T16:32:49.6720204Z bb2: {
2020-04-07T16:32:49.6720356Z   StorageDead(_11);
2020-04-07T16:32:49.6720540Z   StorageDead(_8);
2020-04-07T16:32:49.6720694Z   return;
---
2020-04-07T16:32:49.6723857Z     let mut _10: u8;
2020-04-07T16:32:49.6724048Z     let mut _11: Temp;
2020-04-07T16:32:49.6724208Z     scope 1 {
2020-04-07T16:32:49.6724334Z     }
2020-04-07T16:32:49.6724458Z     bb0: {
2020-04-07T16:32:49.6724627Z         StorageLive(_1);
2020-04-07T16:32:49.6724894Z         StorageLive(_2);
2020-04-07T16:32:49.6725074Z         _2 = const ();
2020-04-07T16:32:49.6725258Z         StorageLive(_3);
2020-04-07T16:32:49.6725426Z         _3 = const ();
2020-04-07T16:32:49.6725641Z         _1 = const {transmute(()): ((), ())};
2020-04-07T16:32:49.6725879Z         StorageDead(_3);
2020-04-07T16:32:49.6726052Z         StorageDead(_2);
2020-04-07T16:32:49.6726221Z         StorageDead(_1);
2020-04-07T16:32:49.6726407Z         StorageLive(_4);
2020-04-07T16:32:49.6726577Z         StorageLive(_6);
2020-04-07T16:32:49.6726744Z         _6 = const ();
2020-04-07T16:32:49.6726911Z         StorageLive(_7);
2020-04-07T16:32:49.6727092Z         _7 = const ();
2020-04-07T16:32:49.6727259Z         StorageDead(_7);
2020-04-07T16:32:49.6727429Z         StorageDead(_6);
2020-04-07T16:32:49.6728099Z         _4 = const use_zst(const {transmute(()): ((), ())}) -> bb1;
2020-04-07T16:32:49.6728479Z     bb1: {
2020-04-07T16:32:49.6728663Z         StorageDead(_4);
2020-04-07T16:32:49.6728846Z         StorageLive(_8);
2020-04-07T16:32:49.6729030Z         StorageLive(_10);
2020-04-07T16:32:49.6729030Z         StorageLive(_10);
2020-04-07T16:32:49.6729230Z         StorageLive(_11);
2020-04-07T16:32:49.6729466Z         _11 = const {transmute(0x28): Temp};
2020-04-07T16:32:49.6729703Z         _10 = const 40u8;
2020-04-07T16:32:49.6729887Z         StorageDead(_10);
2020-04-07T16:32:49.6730298Z         _8 = const use_u8(const 42u8) -> bb2;
2020-04-07T16:32:49.6730613Z     bb2: {
2020-04-07T16:32:49.6730795Z         StorageDead(_11);
2020-04-07T16:32:49.6731085Z         StorageDead(_8);
2020-04-07T16:32:49.6731253Z         _0 = const ();
2020-04-07T16:32:49.6731253Z         _0 = const ();
2020-04-07T16:32:49.6731421Z         return;
2020-04-07T16:32:49.6731548Z     }
2020-04-07T16:32:49.6731897Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6732060Z 
2020-04-07T16:32:49.6732409Z ---- [mir-opt] mir-opt/storage_ranges.rs stdout ----
2020-04-07T16:32:49.6732643Z [ERROR compiletest::runtest] None
2020-04-07T16:32:49.6733200Z thread '[mir-opt] mir-opt/storage_ranges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6733532Z Current block: None
2020-04-07T16:32:49.6733727Z Actual Line: "        _2 = const ();"
2020-04-07T16:32:49.6733946Z Expected Line: "        _2 = ();"
2020-04-07T16:32:49.6734168Z Test Name: rustc.main.nll.0.mir
2020-04-07T16:32:49.6734462Z ... (elided)
2020-04-07T16:32:49.6734611Z     bb0: {
2020-04-07T16:32:49.6734765Z         StorageLive(_1);
2020-04-07T16:32:49.6734935Z         _1 = const 0i32;
2020-04-07T16:32:49.6734935Z         _1 = const 0i32;
2020-04-07T16:32:49.6735129Z         FakeRead(ForLet, _1);
2020-04-07T16:32:49.6735309Z         StorageLive(_2);
2020-04-07T16:32:49.6735478Z         StorageLive(_3);
2020-04-07T16:32:49.6735647Z         StorageLive(_4);
2020-04-07T16:32:49.6735831Z         StorageLive(_5);
2020-04-07T16:32:49.6735989Z         _5 = _1;
2020-04-07T16:32:49.6736219Z         _4 = std::option::Option::<i32>::Some(move _5,);
2020-04-07T16:32:49.6736477Z         StorageDead(_5);
2020-04-07T16:32:49.6736637Z         _3 = &_4;
2020-04-07T16:32:49.6736806Z         FakeRead(ForLet, _3);
2020-04-07T16:32:49.6737145Z         StorageDead(_4);
2020-04-07T16:32:49.6737416Z         StorageDead(_3);
2020-04-07T16:32:49.6737601Z         StorageDead(_2);
2020-04-07T16:32:49.6737772Z         StorageLive(_6);
2020-04-07T16:32:49.6737772Z         StorageLive(_6);
2020-04-07T16:32:49.6737943Z         _6 = const 1i32;
2020-04-07T16:32:49.6738121Z         FakeRead(ForLet, _6);
2020-04-07T16:32:49.6738460Z         StorageDead(_6);
2020-04-07T16:32:49.6738629Z         StorageDead(_1);
2020-04-07T16:32:49.6738799Z         return;
2020-04-07T16:32:49.6738930Z      }
2020-04-07T16:32:49.6738930Z      }
2020-04-07T16:32:49.6739048Z Actual:
2020-04-07T16:32:49.6739354Z | Free Region Mapping
2020-04-07T16:32:49.6739745Z | '_#0r | Global | ['_#0r, '_#1r]
2020-04-07T16:32:49.6740072Z | '_#1r | Local | ['_#1r]
2020-04-07T16:32:49.6740458Z | Inferred Region Values
2020-04-07T16:32:49.6740458Z | Inferred Region Values
2020-04-07T16:32:49.6740860Z | '_#0r | U0 | {bb0[0..=22], '_#0r, '_#1r}
2020-04-07T16:32:49.6741225Z | '_#1r | U0 | {bb0[0..=22], '_#1r}
2020-04-07T16:32:49.6741583Z | '_#2r | U0 | {bb0[10..=11]}
2020-04-07T16:32:49.6741905Z | '_#3r | U0 | {bb0[11]}
2020-04-07T16:32:49.6742184Z | Inference Constraints
2020-04-07T16:32:49.6742184Z | Inference Constraints
2020-04-07T16:32:49.6742506Z | '_#0r live at {bb0[0..=22]}
2020-04-07T16:32:49.6742838Z | '_#1r live at {bb0[0..=22]}
2020-04-07T16:32:49.6743168Z | '_#2r live at {bb0[10]}
2020-04-07T16:32:49.6743477Z | '_#3r live at {bb0[11]}
2020-04-07T16:32:49.6743852Z | '_#2r: '_#3r due to Assignment at Single(bb0[10])
2020-04-07T16:32:49.6744294Z fn main() -> () {
2020-04-07T16:32:49.6744452Z     let mut _0: ();
2020-04-07T16:32:49.6744611Z     let _1: i32;
2020-04-07T16:32:49.6744778Z     let _2: ();
---
2020-04-07T16:32:49.6746567Z         scope 3 {
2020-04-07T16:32:49.6746731Z             debug c => _6;
2020-04-07T16:32:49.6746883Z         }
2020-04-07T16:32:49.6747016Z     }
2020-04-07T16:32:49.6747140Z     bb0: {
2020-04-07T16:32:49.6747294Z         StorageLive(_1);
2020-04-07T16:32:49.6747480Z         _1 = const 0i32;
2020-04-07T16:32:49.6747658Z         FakeRead(ForLet, _1);
2020-04-07T16:32:49.6747962Z         StorageLive(_2);
2020-04-07T16:32:49.6748159Z         StorageLive(_3);
2020-04-07T16:32:49.6748341Z         StorageLive(_4);
2020-04-07T16:32:49.6748521Z         StorageLive(_5);
2020-04-07T16:32:49.6748688Z         _5 = _1;
2020-04-07T16:32:49.6748950Z         _4 = std::option::Option::<i32>::Some(move _5,);
2020-04-07T16:32:49.6749205Z         StorageDead(_5);
2020-04-07T16:32:49.6749375Z         _3 = &_4;
2020-04-07T16:32:49.6749567Z         FakeRead(ForLet, _3);
2020-04-07T16:32:49.6749756Z         _2 = const ();
2020-04-07T16:32:49.6749937Z         StorageDead(_4);
2020-04-07T16:32:49.6750132Z         StorageDead(_3);
2020-04-07T16:32:49.6750312Z         StorageDead(_2);
2020-04-07T16:32:49.6750494Z         StorageLive(_6);
2020-04-07T16:32:49.6750686Z         _6 = const 1i32;
2020-04-07T16:32:49.6750877Z         FakeRead(ForLet, _6);
2020-04-07T16:32:49.6751171Z         _0 = const ();
2020-04-07T16:32:49.6751352Z         StorageDead(_6);
2020-04-07T16:32:49.6751522Z         StorageDead(_1);
2020-04-07T16:32:49.6751808Z     }
2020-04-07T16:32:49.6752183Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6752395Z 
2020-04-07T16:32:49.6752758Z ---- [mir-opt] mir-opt/uniform_array_move_out.rs stdout ----
2020-04-07T16:32:49.6752758Z ---- [mir-opt] mir-opt/uniform_array_move_out.rs stdout ----
2020-04-07T16:32:49.6753021Z [ERROR compiletest::runtest] None
2020-04-07T16:32:49.6753740Z thread '[mir-opt] mir-opt/uniform_array_move_out.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6754096Z Current block: None
2020-04-07T16:32:49.6754430Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6754659Z Expected Line: "     _0 = ();"
2020-04-07T16:32:49.6754904Z Test Name: rustc.move_out_from_end.mir_map.0.mir
2020-04-07T16:32:49.6755260Z ... (elided)
2020-04-07T16:32:49.6755260Z ... (elided)
2020-04-07T16:32:49.6755428Z      _6 = move _1[1 of 2];
2020-04-07T16:32:49.6755750Z Actual:
2020-04-07T16:32:49.6756104Z fn move_out_from_end() -> () {
2020-04-07T16:32:49.6756292Z     let mut _0: ();
2020-04-07T16:32:49.6756292Z     let mut _0: ();
2020-04-07T16:32:49.6756624Z     let _1: [std::boxed::Box<i32>; 2];
2020-04-07T16:32:49.6756862Z     let mut _2: std::boxed::Box<i32>;
2020-04-07T16:32:49.6757098Z     let mut _3: std::boxed::Box<i32>;
2020-04-07T16:32:49.6757421Z     let mut _4: std::boxed::Box<i32>;
2020-04-07T16:32:49.6757667Z     let mut _5: std::boxed::Box<i32>;
2020-04-07T16:32:49.6758023Z         debug a => _1;
2020-04-07T16:32:49.6758225Z         let _6: std::boxed::Box<i32>;
2020-04-07T16:32:49.6758425Z         scope 2 {
2020-04-07T16:32:49.6758425Z         scope 2 {
2020-04-07T16:32:49.6758823Z             debug _y => _6;
2020-04-07T16:32:49.6759116Z     }
2020-04-07T16:32:49.6759237Z     bb0: {
2020-04-07T16:32:49.6759406Z         StorageLive(_1);
2020-04-07T16:32:49.6759577Z         StorageLive(_2);
2020-04-07T16:32:49.6759577Z         StorageLive(_2);
2020-04-07T16:32:49.6759746Z         StorageLive(_3);
2020-04-07T16:32:49.6759927Z         _3 = Box(i32);
2020-04-07T16:32:49.6760099Z         (*_3) = const 1i32;
2020-04-07T16:32:49.6760270Z         _2 = move _3;
2020-04-07T16:32:49.6760773Z         drop(_3) -> [return: bb4, unwind: bb2];
2020-04-07T16:32:49.6760967Z     }
2020-04-07T16:32:49.6761109Z     bb1 (cleanup): {
2020-04-07T16:32:49.6761269Z         resume;
2020-04-07T16:32:49.6761420Z     }
2020-04-07T16:32:49.6761564Z     bb2 (cleanup): {
2020-04-07T16:32:49.6761881Z         drop(_2) -> bb1;
2020-04-07T16:32:49.6762033Z     }
2020-04-07T16:32:49.6762172Z     bb3 (cleanup): {
2020-04-07T16:32:49.6762617Z         drop(_3) -> bb2;
2020-04-07T16:32:49.6763058Z     bb4: {
2020-04-07T16:32:49.6763227Z         StorageDead(_3);
2020-04-07T16:32:49.6763409Z         StorageLive(_4);
2020-04-07T16:32:49.6763607Z         StorageLive(_5);
2020-04-07T16:32:49.6763607Z         StorageLive(_5);
2020-04-07T16:32:49.6763786Z         _5 = Box(i32);
2020-04-07T16:32:49.6763971Z         (*_5) = const 2i32;
2020-04-07T16:32:49.6764170Z         _4 = move _5;
2020-04-07T16:32:49.6764614Z         drop(_5) -> [return: bb7, unwind: bb5];
2020-04-07T16:32:49.6764826Z     }
2020-04-07T16:32:49.6764995Z     bb5 (cleanup): {
2020-04-07T16:32:49.6765330Z         drop(_4) -> bb2;
2020-04-07T16:32:49.6765479Z     }
2020-04-07T16:32:49.6765643Z     bb6 (cleanup): {
2020-04-07T16:32:49.6765978Z         drop(_5) -> bb5;
2020-04-07T16:32:49.6766263Z     bb7: {
2020-04-07T16:32:49.6766437Z         StorageDead(_5);
2020-04-07T16:32:49.6766437Z         StorageDead(_5);
2020-04-07T16:32:49.6766627Z         _1 = [move _2, move _4];
2020-04-07T16:32:49.6767048Z         drop(_4) -> [return: bb8, unwind: bb2];
2020-04-07T16:32:49.6767403Z     bb8: {
2020-04-07T16:32:49.6767566Z         StorageDead(_4);
2020-04-07T16:32:49.6767566Z         StorageDead(_4);
2020-04-07T16:32:49.6767991Z         drop(_2) -> [return: bb9, unwind: bb1];
2020-04-07T16:32:49.6768333Z     bb9: {
2020-04-07T16:32:49.6768498Z         StorageDead(_2);
2020-04-07T16:32:49.6768498Z         StorageDead(_2);
2020-04-07T16:32:49.6768703Z         FakeRead(ForLet, _1);
2020-04-07T16:32:49.6768894Z         StorageLive(_6);
2020-04-07T16:32:49.6769085Z         _6 = move _1[1 of 2];
2020-04-07T16:32:49.6769288Z         _0 = const ();
2020-04-07T16:32:49.6769701Z         drop(_6) -> [return: bb12, unwind: bb10];
2020-04-07T16:32:49.6769910Z     }
2020-04-07T16:32:49.6770077Z     bb10 (cleanup): {
2020-04-07T16:32:49.6770414Z         drop(_1) -> bb1;
2020-04-07T16:32:49.6770562Z     }
2020-04-07T16:32:49.6770727Z     bb11 (cleanup): {
2020-04-07T16:32:49.6771064Z         drop(_6) -> bb10;
2020-04-07T16:32:49.6771354Z     bb12: {
2020-04-07T16:32:49.6771690Z         StorageDead(_6);
2020-04-07T16:32:49.6771690Z         StorageDead(_6);
2020-04-07T16:32:49.6772137Z         drop(_1) -> [return: bb13, unwind: bb1];
2020-04-07T16:32:49.6772492Z     bb13: {
2020-04-07T16:32:49.6772658Z         StorageDead(_1);
2020-04-07T16:32:49.6773084Z         goto -> bb14;
2020-04-07T16:32:49.6773234Z     }
2020-04-07T16:32:49.6773234Z     }
2020-04-07T16:32:49.6773359Z     bb14: {
2020-04-07T16:32:49.6773501Z         return;
2020-04-07T16:32:49.6773644Z     }
2020-04-07T16:32:49.6773987Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6774149Z 
2020-04-07T16:32:49.6774486Z ---- [mir-opt] mir-opt/uninhabited-enum.rs stdout ----
2020-04-07T16:32:49.6774734Z [ERROR compiletest::runtest] None
2020-04-07T16:32:49.6775377Z thread '[mir-opt] mir-opt/uninhabited-enum.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6775711Z Current block: None
2020-04-07T16:32:49.6775922Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6776145Z Expected Line: "    StorageDead(_2);"
2020-04-07T16:32:49.6776396Z Test Name: rustc.process_void.SimplifyLocals.after.mir
2020-04-07T16:32:49.6776738Z ... (elided)
2020-04-07T16:32:49.6776866Z bb0: {
2020-04-07T16:32:49.6777007Z     StorageLive(_2);
2020-04-07T16:32:49.6777176Z     _2 = &(*_1);
2020-04-07T16:32:49.6777176Z     _2 = &(*_1);
2020-04-07T16:32:49.6777329Z     StorageDead(_2);
2020-04-07T16:32:49.6777592Z     return;
2020-04-07T16:32:49.6777731Z }
2020-04-07T16:32:49.6777848Z Actual:
2020-04-07T16:32:49.6778237Z fn process_void(_1: *const Void) -> () {
2020-04-07T16:32:49.6778637Z     let mut _0: ();
2020-04-07T16:32:49.6778813Z     let _2: &Void;
2020-04-07T16:32:49.6778989Z     scope 1 {
2020-04-07T16:32:49.6779165Z         debug _input => _2;
---
2020-04-07T16:32:49.6780623Z         return;
2020-04-07T16:32:49.6780759Z     }
2020-04-07T16:32:49.6781235Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6781396Z 
2020-04-07T16:32:49.6781777Z ---- [mir-opt] mir-opt/uninhabited_enum_branching.rs stdout ----
2020-04-07T16:32:49.6782049Z [ERROR compiletest::runtest] Some("bb7: {")
2020-04-07T16:32:49.6782654Z thread '[mir-opt] mir-opt/uninhabited_enum_branching.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6783014Z Current block: bb7: {
2020-04-07T16:32:49.6783208Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6783412Z Expected Line: "  _0 = ();"
2020-04-07T16:32:49.6783659Z Test Name: rustc.main.UninhabitedEnumBranching.before.mir
2020-04-07T16:32:49.6784977Z ... (elided)
2020-04-07T16:32:49.6785282Z let mut _0: ();
2020-04-07T16:32:49.6785458Z let _1: &str;
2020-04-07T16:32:49.6785627Z let mut _2: Test1;
2020-04-07T16:32:49.6785627Z let mut _2: Test1;
2020-04-07T16:32:49.6785806Z let mut _3: isize;
2020-04-07T16:32:49.6785990Z let _4: &str;
2020-04-07T16:32:49.6786148Z let _5: &str;
2020-04-07T16:32:49.6786305Z let _6: &str;
2020-04-07T16:32:49.6786488Z let mut _7: Test2;
2020-04-07T16:32:49.6786666Z let mut _8: isize;
2020-04-07T16:32:49.6786832Z let _9: &str;
2020-04-07T16:32:49.6786992Z bb0: {
2020-04-07T16:32:49.6787142Z   StorageLive(_1);
2020-04-07T16:32:49.6787309Z   StorageLive(_2);
2020-04-07T16:32:49.6787606Z   _2 = Test1::C;
2020-04-07T16:32:49.6787812Z   _3 = discriminant(_2);
2020-04-07T16:32:49.6788663Z   switchInt(move _3) -> [0isize: bb2, 1isize: bb3, otherwise: bb1];
2020-04-07T16:32:49.6789051Z bb1: {
2020-04-07T16:32:49.6789197Z   StorageLive(_5);
2020-04-07T16:32:49.6789361Z   _5 = const "C";
2020-04-07T16:32:49.6789530Z   _1 = &(*_5);
2020-04-07T16:32:49.6789530Z   _1 = &(*_5);
2020-04-07T16:32:49.6789685Z   StorageDead(_5);
2020-04-07T16:32:49.6789989Z   goto -> bb4;
2020-04-07T16:32:49.6790307Z }
2020-04-07T16:32:49.6790425Z bb2: {
2020-04-07T16:32:49.6790581Z   _1 = const "A(Empty)";
2020-04-07T16:32:49.6800332Z   goto -> bb4;
2020-04-07T16:32:49.6800651Z bb3: {
2020-04-07T16:32:49.6800799Z   StorageLive(_4);
2020-04-07T16:32:49.6800799Z   StorageLive(_4);
2020-04-07T16:32:49.6800990Z   _4 = const "B(Empty)";
2020-04-07T16:32:49.6801158Z   _1 = &(*_4);
2020-04-07T16:32:49.6801314Z   StorageDead(_4);
2020-04-07T16:32:49.6801779Z   goto -> bb4;
2020-04-07T16:32:49.6802023Z bb4: {
2020-04-07T16:32:49.6802164Z   StorageDead(_2);
2020-04-07T16:32:49.6802336Z   StorageDead(_1);
2020-04-07T16:32:49.6802491Z   StorageLive(_6);
2020-04-07T16:32:49.6802491Z   StorageLive(_6);
2020-04-07T16:32:49.6806376Z   StorageLive(_7);
2020-04-07T16:32:49.6806583Z   _7 = Test2::D;
2020-04-07T16:32:49.6806941Z   _8 = discriminant(_7);
2020-04-07T16:32:49.6807581Z   switchInt(move _8) -> [4isize: bb6, otherwise: bb5];
2020-04-07T16:32:49.6807934Z bb5: {
2020-04-07T16:32:49.6808076Z   StorageLive(_9);
2020-04-07T16:32:49.6808243Z   _9 = const "E";
2020-04-07T16:32:49.6808416Z   _6 = &(*_9);
---
2020-04-07T16:32:49.6810448Z   return;
2020-04-07T16:32:49.6810582Z }
2020-04-07T16:32:49.6810699Z Actual:
2020-04-07T16:32:49.6810984Z fn main() -> () {
2020-04-07T16:32:49.6811169Z     let mut _0: ();
2020-04-07T16:32:49.6811350Z     let _1: &str;
2020-04-07T16:32:49.6811629Z     let mut _2: Test1;
2020-04-07T16:32:49.6811807Z     let mut _3: isize;
2020-04-07T16:32:49.6811993Z     let _4: &str;
2020-04-07T16:32:49.6812153Z     let _5: &str;
2020-04-07T16:32:49.6812316Z     let _6: &str;
2020-04-07T16:32:49.6812498Z     let mut _7: Test2;
2020-04-07T16:32:49.6812675Z     let mut _8: isize;
2020-04-07T16:32:49.6812842Z     let _9: &str;
2020-04-07T16:32:49.6813004Z     bb0: {
2020-04-07T16:32:49.6813160Z         StorageLive(_1);
2020-04-07T16:32:49.6813330Z         StorageLive(_2);
2020-04-07T16:32:49.6813522Z         _2 = Test1::C;
2020-04-07T16:32:49.6813708Z         _3 = discriminant(_2);
2020-04-07T16:32:49.6814188Z         switchInt(move _3) -> [0isize: bb2, 1isize: bb3, otherwise: bb1];
2020-04-07T16:32:49.6814579Z     bb1: {
2020-04-07T16:32:49.6814732Z         StorageLive(_5);
2020-04-07T16:32:49.6814901Z         _5 = const "C";
2020-04-07T16:32:49.6815080Z         _1 = &(*_5);
2020-04-07T16:32:49.6815080Z         _1 = &(*_5);
2020-04-07T16:32:49.6815250Z         StorageDead(_5);
2020-04-07T16:32:49.6815554Z         goto -> bb4;
2020-04-07T16:32:49.6815703Z     }
2020-04-07T16:32:49.6815823Z     bb2: {
2020-04-07T16:32:49.6815983Z         _1 = const "A(Empty)";
2020-04-07T16:32:49.6816313Z         goto -> bb4;
2020-04-07T16:32:49.6816568Z     bb3: {
2020-04-07T16:32:49.6816721Z         StorageLive(_4);
2020-04-07T16:32:49.6816721Z         StorageLive(_4);
2020-04-07T16:32:49.6816916Z         _4 = const "B(Empty)";
2020-04-07T16:32:49.6817090Z         _1 = &(*_4);
2020-04-07T16:32:49.6817254Z         StorageDead(_4);
2020-04-07T16:32:49.6817567Z         goto -> bb4;
2020-04-07T16:32:49.6817826Z     bb4: {
2020-04-07T16:32:49.6817995Z         StorageDead(_2);
2020-04-07T16:32:49.6818166Z         StorageDead(_1);
2020-04-07T16:32:49.6818335Z         StorageLive(_6);
2020-04-07T16:32:49.6818335Z         StorageLive(_6);
2020-04-07T16:32:49.6818503Z         StorageLive(_7);
2020-04-07T16:32:49.6818694Z         _7 = Test2::D;
2020-04-07T16:32:49.6818881Z         _8 = discriminant(_7);
2020-04-07T16:32:49.6819311Z         switchInt(move _8) -> [4isize: bb6, otherwise: bb5];
2020-04-07T16:32:49.6819672Z     bb5: {
2020-04-07T16:32:49.6819826Z         StorageLive(_9);
2020-04-07T16:32:49.6820131Z         _9 = const "E";
2020-04-07T16:32:49.6820294Z         _6 = &(*_9);
---
2020-04-07T16:32:49.6822463Z         return;
2020-04-07T16:32:49.6822591Z     }
2020-04-07T16:32:49.6822939Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6823184Z 
2020-04-07T16:32:49.6823572Z ---- [mir-opt] mir-opt/unreachable.rs stdout ----
2020-04-07T16:32:49.6823841Z [ERROR compiletest::runtest] Some("     bb2: {")
2020-04-07T16:32:49.6824416Z thread '[mir-opt] mir-opt/unreachable.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6824765Z Current block:      bb2: {
2020-04-07T16:32:49.6824974Z Actual Line: "        _0 = const ();"
2020-04-07T16:32:49.6825192Z Expected Line: "         _0 = ();"
2020-04-07T16:32:49.6825451Z Test Name: rustc.main.UnreachablePropagation.before.mir
2020-04-07T16:32:49.6825781Z ... (elided)
2020-04-07T16:32:49.6825933Z      bb0: {
2020-04-07T16:32:49.6826092Z          StorageLive(_1);
2020-04-07T16:32:49.6826436Z          _1 = const empty() -> bb1;
2020-04-07T16:32:49.6826436Z          _1 = const empty() -> bb1;
2020-04-07T16:32:49.6826613Z      }
2020-04-07T16:32:49.6826741Z      bb1: {
2020-04-07T16:32:49.6826908Z          _2 = discriminant(_1);
2020-04-07T16:32:49.6827341Z          switchInt(move _2) -> [1isize: bb3, otherwise: bb2];
2020-04-07T16:32:49.6827699Z      bb2: {
2020-04-07T16:32:49.6827843Z          _0 = ();
2020-04-07T16:32:49.6828023Z          StorageDead(_1);
2020-04-07T16:32:49.6828185Z          return;
2020-04-07T16:32:49.6828185Z          return;
2020-04-07T16:32:49.6828319Z      }
2020-04-07T16:32:49.6828460Z      bb3: {
2020-04-07T16:32:49.6828618Z          StorageLive(_3);
2020-04-07T16:32:49.6828837Z          _3 = move ((_1 as Some).0: Empty);
2020-04-07T16:32:49.6829055Z          StorageLive(_4);
2020-04-07T16:32:49.6829243Z          StorageLive(_5);
2020-04-07T16:32:49.6829415Z          StorageLive(_6);
2020-04-07T16:32:49.6829587Z          _6 = const true;
2020-04-07T16:32:49.6830012Z          switchInt(_6) -> [false: bb4, otherwise: bb5];
2020-04-07T16:32:49.6830354Z      bb4: {
2020-04-07T16:32:49.6830527Z          _4 = const 42i32;
2020-04-07T16:32:49.6830690Z          _5 = ();
2020-04-07T16:32:49.6830983Z          goto -> bb6;
---
2020-04-07T16:32:49.6832995Z      }
2020-04-07T16:32:49.6833104Z  }
2020-04-07T16:32:49.6833230Z Actual:
2020-04-07T16:32:49.6833498Z fn main() -> () {
2020-04-07T16:32:49.6833656Z     let mut _0: ();
2020-04-07T16:32:49.6833865Z     let mut _1: std::option::Option<Empty>;
2020-04-07T16:32:49.6834095Z     let mut _2: isize;
2020-04-07T16:32:49.6834266Z     let _3: Empty;
2020-04-07T16:32:49.6834425Z     let _5: ();
2020-04-07T16:32:49.6834606Z     let mut _6: bool;
2020-04-07T16:32:49.6834776Z     let mut _7: !;
2020-04-07T16:32:49.6835102Z         debug _x => _3;
2020-04-07T16:32:49.6835281Z         let mut _4: i32;
2020-04-07T16:32:49.6835450Z         scope 2 {
2020-04-07T16:32:49.6835450Z         scope 2 {
2020-04-07T16:32:49.6835614Z             debug _y => _4;
2020-04-07T16:32:49.6835995Z     }
2020-04-07T16:32:49.6836119Z     bb0: {
2020-04-07T16:32:49.6836291Z         StorageLive(_1);
2020-04-07T16:32:49.6836667Z         _1 = const empty() -> bb1;
2020-04-07T16:32:49.6836667Z         _1 = const empty() -> bb1;
2020-04-07T16:32:49.6836822Z     }
2020-04-07T16:32:49.6836958Z     bb1: {
2020-04-07T16:32:49.6837123Z         _2 = discriminant(_1);
2020-04-07T16:32:49.6837550Z         switchInt(move _2) -> [1isize: bb3, otherwise: bb2];
2020-04-07T16:32:49.6837912Z     bb2: {
2020-04-07T16:32:49.6838064Z         _0 = const ();
2020-04-07T16:32:49.6838232Z         StorageDead(_1);
2020-04-07T16:32:49.6838406Z         return;
2020-04-07T16:32:49.6838406Z         return;
2020-04-07T16:32:49.6838534Z     }
2020-04-07T16:32:49.6838657Z     bb3: {
2020-04-07T16:32:49.6838901Z         StorageLive(_3);
2020-04-07T16:32:49.6839126Z         _3 = move ((_1 as Some).0: Empty);
2020-04-07T16:32:49.6839342Z         StorageLive(_4);
2020-04-07T16:32:49.6839529Z         StorageLive(_5);
2020-04-07T16:32:49.6839699Z         StorageLive(_6);
2020-04-07T16:32:49.6839873Z         _6 = const true;
2020-04-07T16:32:49.6840323Z         switchInt(_6) -> [false: bb4, otherwise: bb5];
2020-04-07T16:32:49.6840651Z     bb4: {
2020-04-07T16:32:49.6840806Z         _4 = const 42i32;
2020-04-07T16:32:49.6840990Z         _5 = const ();
2020-04-07T16:32:49.6841288Z         goto -> bb6;
---
2020-04-07T16:32:49.6843265Z         unreachable;
2020-04-07T16:32:49.6843415Z     }
2020-04-07T16:32:49.6843801Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6843971Z 
2020-04-07T16:32:49.6844322Z ---- [mir-opt] mir-opt/unreachable_asm.rs stdout ----
2020-04-07T16:32:49.6844556Z [ERROR compiletest::runtest] None
2020-04-07T16:32:49.6845109Z thread '[mir-opt] mir-opt/unreachable_asm.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6845442Z Current block: None
2020-04-07T16:32:49.6845631Z Actual Line: "        _5 = const ();"
2020-04-07T16:32:49.6845844Z Expected Line: "         _5 = ();"
2020-04-07T16:32:49.6846088Z Test Name: rustc.main.UnreachablePropagation.before.mir
2020-04-07T16:32:49.6846432Z ... (elided)
2020-04-07T16:32:49.6846570Z      bb4: {
2020-04-07T16:32:49.6846745Z          _4 = const 42i32;
2020-04-07T16:32:49.6846914Z          _5 = ();
2020-04-07T16:32:49.6846914Z          _5 = ();
2020-04-07T16:32:49.6847212Z          goto -> bb6;
2020-04-07T16:32:49.6847364Z      }
2020-04-07T16:32:49.6847489Z      bb5: {
2020-04-07T16:32:49.6847649Z          _4 = const 21i32;
2020-04-07T16:32:49.6847826Z          _5 = ();
2020-04-07T16:32:49.6848122Z          goto -> bb6;
2020-04-07T16:32:49.6848263Z      }
2020-04-07T16:32:49.6848390Z      bb6: {
2020-04-07T16:32:49.6848562Z          StorageDead(_6);
2020-04-07T16:32:49.6848736Z          StorageDead(_5);
2020-04-07T16:32:49.6848909Z          StorageLive(_7);
2020-04-07T16:32:49.6849402Z          llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []);
2020-04-07T16:32:49.6850022Z          StorageDead(_7);
2020-04-07T16:32:49.6850211Z          StorageLive(_8);
2020-04-07T16:32:49.6850378Z          unreachable;
2020-04-07T16:32:49.6850516Z      }
2020-04-07T16:32:49.6850516Z      }
2020-04-07T16:32:49.6850640Z  }
2020-04-07T16:32:49.6850756Z Actual:
2020-04-07T16:32:49.6851028Z fn main() -> () {
2020-04-07T16:32:49.6851185Z     let mut _0: ();
2020-04-07T16:32:49.6851409Z     let mut _1: std::option::Option<Empty>;
2020-04-07T16:32:49.6851623Z     let mut _2: isize;
2020-04-07T16:32:49.6851995Z     let _3: Empty;
2020-04-07T16:32:49.6852174Z     let _5: ();
2020-04-07T16:32:49.6852340Z     let mut _6: bool;
2020-04-07T16:32:49.6852506Z     let _7: ();
2020-04-07T16:32:49.6852679Z     let mut _8: !;
2020-04-07T16:32:49.6852986Z         debug _x => _3;
2020-04-07T16:32:49.6853178Z         let mut _4: i32;
2020-04-07T16:32:49.6853509Z         scope 2 {
2020-04-07T16:32:49.6853509Z         scope 2 {
2020-04-07T16:32:49.6853677Z             debug _y => _4;
2020-04-07T16:32:49.6854012Z             }
2020-04-07T16:32:49.6854147Z         }
2020-04-07T16:32:49.6854389Z     }
2020-04-07T16:32:49.6854537Z     bb0: {
2020-04-07T16:32:49.6854537Z     bb0: {
2020-04-07T16:32:49.6854701Z         StorageLive(_1);
2020-04-07T16:32:49.6855260Z         _1 = const empty() -> bb1;
2020-04-07T16:32:49.6855454Z     }
2020-04-07T16:32:49.6855584Z     bb1: {
2020-04-07T16:32:49.6855758Z         _2 = discriminant(_1);
2020-04-07T16:32:49.6856254Z         switchInt(move _2) -> [1isize: bb3, otherwise: bb2];
2020-04-07T16:32:49.6856644Z     bb2: {
2020-04-07T16:32:49.6856804Z         _0 = const ();
2020-04-07T16:32:49.6856999Z         StorageDead(_1);
2020-04-07T16:32:49.6857166Z         return;
2020-04-07T16:32:49.6857166Z         return;
2020-04-07T16:32:49.6857302Z     }
2020-04-07T16:32:49.6857447Z     bb3: {
2020-04-07T16:32:49.6857715Z         StorageLive(_3);
2020-04-07T16:32:49.6857929Z         _3 = move ((_1 as Some).0: Empty);
2020-04-07T16:32:49.6858156Z         StorageLive(_4);
2020-04-07T16:32:49.6858327Z         StorageLive(_5);
2020-04-07T16:32:49.6858495Z         StorageLive(_6);
2020-04-07T16:32:49.6858664Z         _6 = const true;
2020-04-07T16:32:49.6859087Z         switchInt(_6) -> [false: bb4, otherwise: bb5];
2020-04-07T16:32:49.6859419Z     bb4: {
2020-04-07T16:32:49.6859590Z         _4 = const 42i32;
2020-04-07T16:32:49.6859759Z         _5 = const ();
2020-04-07T16:32:49.6860057Z         goto -> bb6;
2020-04-07T16:32:49.6860057Z         goto -> bb6;
2020-04-07T16:32:49.6860206Z     }
2020-04-07T16:32:49.6860330Z     bb5: {
2020-04-07T16:32:49.6860482Z         _4 = const 21i32;
2020-04-07T16:32:49.6860666Z         _5 = const ();
2020-04-07T16:32:49.6860964Z         goto -> bb6;
2020-04-07T16:32:49.6861097Z     }
2020-04-07T16:32:49.6861218Z     bb6: {
2020-04-07T16:32:49.6861382Z         StorageDead(_6);
2020-04-07T16:32:49.6861547Z         StorageDead(_5);
2020-04-07T16:32:49.6861714Z         StorageLive(_7);
2020-04-07T16:32:49.6862204Z         llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []);
2020-04-07T16:32:49.6862671Z         _7 = const ();
2020-04-07T16:32:49.6862839Z         StorageDead(_7);
2020-04-07T16:32:49.6863028Z         StorageLive(_8);
2020-04-07T16:32:49.6863327Z     }
2020-04-07T16:32:49.6863749Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6863911Z 
2020-04-07T16:32:49.6864254Z ---- [mir-opt] mir-opt/unreachable_asm_2.rs stdout ----
2020-04-07T16:32:49.6864254Z ---- [mir-opt] mir-opt/unreachable_asm_2.rs stdout ----
2020-04-07T16:32:49.6864528Z [ERROR compiletest::runtest] Some("     bb4: {")
2020-04-07T16:32:49.6865713Z thread '[mir-opt] mir-opt/unreachable_asm_2.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6866229Z Current block:      bb4: {
2020-04-07T16:32:49.6866470Z Actual Line: "        _8 = const ();"
2020-04-07T16:32:49.6866741Z Expected Line: "         _8 = ();"
2020-04-07T16:32:49.6867021Z Test Name: rustc.main.UnreachablePropagation.before.mir
2020-04-07T16:32:49.6867412Z ... (elided)
2020-04-07T16:32:49.6867570Z      bb3: {
2020-04-07T16:32:49.6867727Z ... (elided)
2020-04-07T16:32:49.6867727Z ... (elided)
2020-04-07T16:32:49.6868266Z          switchInt(_6) -> [false: bb4, otherwise: bb5];
2020-04-07T16:32:49.6868653Z      bb4: {
2020-04-07T16:32:49.6868849Z          StorageLive(_8);
2020-04-07T16:32:49.6868849Z          StorageLive(_8);
2020-04-07T16:32:49.6869516Z          llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []);
2020-04-07T16:32:49.6870475Z          StorageDead(_8);
2020-04-07T16:32:49.6870662Z          _4 = const 42i32;
2020-04-07T16:32:49.6870835Z          _5 = ();
2020-04-07T16:32:49.6871189Z          goto -> bb6;
2020-04-07T16:32:49.6871189Z          goto -> bb6;
2020-04-07T16:32:49.6871353Z      }
2020-04-07T16:32:49.6871494Z          bb5: {
2020-04-07T16:32:49.6871669Z          StorageLive(_7);
2020-04-07T16:32:49.6872194Z          llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []);
2020-04-07T16:32:49.6872957Z          StorageDead(_7);
2020-04-07T16:32:49.6873170Z          _4 = const 21i32;
2020-04-07T16:32:49.6873344Z          _5 = ();
2020-04-07T16:32:49.6873692Z          goto -> bb6;
---
2020-04-07T16:32:49.6874964Z      }
2020-04-07T16:32:49.6875089Z  }
2020-04-07T16:32:49.6875200Z Actual:
2020-04-07T16:32:49.6875473Z fn main() -> () {
2020-04-07T16:32:49.6875645Z     let mut _0: ();
2020-04-07T16:32:49.6875855Z     let mut _1: std::option::Option<Empty>;
2020-04-07T16:32:49.6876068Z     let mut _2: isize;
2020-04-07T16:32:49.6876254Z     let _3: Empty;
2020-04-07T16:32:49.6876414Z     let _5: ();
2020-04-07T16:32:49.6876580Z     let mut _6: bool;
2020-04-07T16:32:49.6876758Z     let _7: ();
2020-04-07T16:32:49.6876916Z     let _8: ();
2020-04-07T16:32:49.6877076Z     let mut _9: !;
2020-04-07T16:32:49.6877397Z         debug _x => _3;
2020-04-07T16:32:49.6877575Z         let mut _4: i32;
2020-04-07T16:32:49.6877744Z         scope 2 {
2020-04-07T16:32:49.6877744Z         scope 2 {
2020-04-07T16:32:49.6877926Z             debug _y => _4;
2020-04-07T16:32:49.6878247Z             }
2020-04-07T16:32:49.6878410Z             scope 4 {
2020-04-07T16:32:49.6878559Z             }
2020-04-07T16:32:49.6878691Z         }
2020-04-07T16:32:49.6878691Z         }
2020-04-07T16:32:49.6878810Z     }
2020-04-07T16:32:49.6878950Z     bb0: {
2020-04-07T16:32:49.6879103Z         StorageLive(_1);
2020-04-07T16:32:49.6879447Z         _1 = const empty() -> bb1;
2020-04-07T16:32:49.6879618Z     }
2020-04-07T16:32:49.6879739Z     bb1: {
2020-04-07T16:32:49.6879898Z         _2 = discriminant(_1);
2020-04-07T16:32:49.6880345Z         switchInt(move _2) -> [1isize: bb3, otherwise: bb2];
2020-04-07T16:32:49.6880820Z     bb2: {
2020-04-07T16:32:49.6880991Z         _0 = const ();
2020-04-07T16:32:49.6881167Z         StorageDead(_1);
2020-04-07T16:32:49.6881329Z         return;
2020-04-07T16:32:49.6881329Z         return;
2020-04-07T16:32:49.6881460Z     }
2020-04-07T16:32:49.6881602Z     bb3: {
2020-04-07T16:32:49.6881765Z         StorageLive(_3);
2020-04-07T16:32:49.6881994Z         _3 = move ((_1 as Some).0: Empty);
2020-04-07T16:32:49.6882237Z         StorageLive(_4);
2020-04-07T16:32:49.6882418Z         StorageLive(_5);
2020-04-07T16:32:49.6882599Z         StorageLive(_6);
2020-04-07T16:32:49.6882792Z         _6 = const true;
2020-04-07T16:32:49.6883595Z         switchInt(_6) -> [false: bb4, otherwise: bb5];
2020-04-07T16:32:49.6892274Z     bb4: {
2020-04-07T16:32:49.6892445Z         StorageLive(_8);
2020-04-07T16:32:49.6892445Z         StorageLive(_8);
2020-04-07T16:32:49.6892953Z         llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []);
2020-04-07T16:32:49.6893478Z         _8 = const ();
2020-04-07T16:32:49.6893659Z         StorageDead(_8);
2020-04-07T16:32:49.6893844Z         _4 = const 42i32;
2020-04-07T16:32:49.6894027Z         _5 = const ();
2020-04-07T16:32:49.6898053Z         goto -> bb6;
2020-04-07T16:32:49.6898553Z     bb5: {
2020-04-07T16:32:49.6898744Z         StorageLive(_7);
2020-04-07T16:32:49.6898744Z         StorageLive(_7);
2020-04-07T16:32:49.6899253Z         llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []);
2020-04-07T16:32:49.6899754Z         _7 = const ();
2020-04-07T16:32:49.6899949Z         StorageDead(_7);
2020-04-07T16:32:49.6900135Z         _4 = const 21i32;
2020-04-07T16:32:49.6900477Z         _5 = const ();
2020-04-07T16:32:49.6900905Z         goto -> bb6;
2020-04-07T16:32:49.6901189Z     bb6: {
2020-04-07T16:32:49.6901370Z         StorageDead(_6);
2020-04-07T16:32:49.6901559Z         StorageDead(_5);
2020-04-07T16:32:49.6901846Z         StorageLive(_9);
2020-04-07T16:32:49.6901846Z         StorageLive(_9);
2020-04-07T16:32:49.6902037Z         unreachable;
2020-04-07T16:32:49.6902204Z     }
2020-04-07T16:32:49.6902633Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6902815Z 
2020-04-07T16:32:49.6903336Z ---- [mir-opt] mir-opt/unreachable_diverging.rs stdout ----
2020-04-07T16:32:49.6903640Z [ERROR compiletest::runtest] Some("     bb4: {")
2020-04-07T16:32:49.6904482Z thread '[mir-opt] mir-opt/unreachable_diverging.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6904881Z Current block:      bb4: {
2020-04-07T16:32:49.6905106Z Actual Line: "        _5 = const ();"
2020-04-07T16:32:49.6905340Z Expected Line: "         _5 = ();"
2020-04-07T16:32:49.6905615Z Test Name: rustc.main.UnreachablePropagation.before.mir
2020-04-07T16:32:49.6905967Z ... (elided)
2020-04-07T16:32:49.6906115Z      bb3: {
2020-04-07T16:32:49.6906299Z          StorageLive(_4);
2020-04-07T16:32:49.6906299Z          StorageLive(_4);
2020-04-07T16:32:49.6906539Z          _4 = move ((_2 as Some).0: Empty);
2020-04-07T16:32:49.6906773Z          StorageLive(_5);
2020-04-07T16:32:49.6906973Z          StorageLive(_6);
2020-04-07T16:32:49.6907146Z          _6 = _1;
2020-04-07T16:32:49.6907576Z          switchInt(_6) -> [false: bb4, otherwise: bb5];
2020-04-07T16:32:49.6908068Z      bb4: {
2020-04-07T16:32:49.6908213Z          _5 = ();
2020-04-07T16:32:49.6908518Z          goto -> bb6;
2020-04-07T16:32:49.6908656Z      }
---
2020-04-07T16:32:49.6910256Z      }
2020-04-07T16:32:49.6910365Z  }
2020-04-07T16:32:49.6910476Z Actual:
2020-04-07T16:32:49.6910746Z fn main() -> () {
2020-04-07T16:32:49.6910920Z     let mut _0: ();
2020-04-07T16:32:49.6911081Z     let _1: bool;
2020-04-07T16:32:49.6911292Z     let mut _2: std::option::Option<Empty>;
2020-04-07T16:32:49.6911524Z     let mut _3: isize;
2020-04-07T16:32:49.6911694Z     let _5: ();
2020-04-07T16:32:49.6911859Z     let mut _6: bool;
2020-04-07T16:32:49.6912044Z     let mut _7: !;
2020-04-07T16:32:49.6912349Z         debug x => _1;
2020-04-07T16:32:49.6912534Z         let _4: Empty;
2020-04-07T16:32:49.6912699Z         scope 2 {
2020-04-07T16:32:49.6912867Z             debug bomb => _4;
---
2020-04-07T16:32:49.6913624Z         _1 = const true;
2020-04-07T16:32:49.6913795Z         StorageLive(_2);
2020-04-07T16:32:49.6914136Z         _2 = const empty() -> bb1;
2020-04-07T16:32:49.6914306Z     }
2020-04-07T16:32:49.6914431Z     bb1: {
2020-04-07T16:32:49.6914592Z         _3 = discriminant(_2);
2020-04-07T16:32:49.6915035Z         switchInt(move _3) -> [1isize: bb3, otherwise: bb2];
2020-04-07T16:32:49.6915597Z     bb2: {
2020-04-07T16:32:49.6915759Z         _0 = const ();
2020-04-07T16:32:49.6915955Z         StorageDead(_1);
2020-04-07T16:32:49.6916138Z         StorageDead(_2);
2020-04-07T16:32:49.6916138Z         StorageDead(_2);
2020-04-07T16:32:49.6916306Z         return;
2020-04-07T16:32:49.6916457Z     }
2020-04-07T16:32:49.6916587Z     bb3: {
2020-04-07T16:32:49.6916751Z         StorageLive(_4);
2020-04-07T16:32:49.6916993Z         _4 = move ((_2 as Some).0: Empty);
2020-04-07T16:32:49.6917223Z         StorageLive(_5);
2020-04-07T16:32:49.6917405Z         StorageLive(_6);
2020-04-07T16:32:49.6917574Z         _6 = _1;
2020-04-07T16:32:49.6918040Z         switchInt(_6) -> [false: bb4, otherwise: bb5];
2020-04-07T16:32:49.6918388Z     bb4: {
2020-04-07T16:32:49.6918635Z         _5 = const ();
2020-04-07T16:32:49.6919077Z         goto -> bb6;
2020-04-07T16:32:49.6919212Z     }
---
2020-04-07T16:32:49.6920789Z         unreachable;
2020-04-07T16:32:49.6920933Z     }
2020-04-07T16:32:49.6921303Z }', src/tools/compiletest/src/runtest.rs:3246:13
2020-04-07T16:32:49.6921491Z 
2020-04-07T16:32:49.6921842Z ---- [mir-opt] mir-opt/while-storage.rs stdout ----
2020-04-07T16:32:49.6922114Z [ERROR compiletest::runtest] Some("bb1: {")
2020-04-07T16:32:49.6922737Z thread '[mir-opt] mir-opt/while-storage.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-04-07T16:32:49.6923276Z Current block: bb1: {
2020-04-07T16:32:49.6923910Z Actual Line: "        switchInt(_2) -> [false: bb2, otherwise: bb3];"
2020-04-07T16:32:49.6924458Z Expected Line: "    switchInt(_2) -> [false: bb6, otherwise: bb2];"
2020-04-07T16:32:49.6924763Z Test Name: rustc.while_loop.PreCodegen.after.mir
2020-04-07T16:32:49.6925087Z ... (elided)
2020-04-07T16:32:49.6925230Z bb0: {
2020-04-07T16:32:49.6925372Z     StorageLive(_2);
2020-04-07T16:32:49.6925531Z     StorageLive(_3);
2020-04-07T16:32:49.6925531Z     StorageLive(_3);
2020-04-07T16:32:49.6925695Z     _3 = _1;
2020-04-07T16:32:49.6926025Z     _2 = const get_bool(move _3) -> bb1;
2020-04-07T16:32:49.6926307Z bb1: {
2020-04-07T16:32:49.6926447Z     StorageDead(_3);
2020-04-07T16:32:49.6926447Z     StorageDead(_3);
2020-04-07T16:32:49.6926830Z     switchInt(_2) -> [false: bb6, otherwise: bb2];
2020-04-07T16:32:49.6927150Z bb2: {
2020-04-07T16:32:49.6927409Z      StorageLive(_4);
2020-04-07T16:32:49.6927583Z      StorageLive(_5);
2020-04-07T16:32:49.6927758Z      _5 = _1;
2020-04-07T16:32:49.6927758Z      _5 = _1;
2020-04-07T16:32:49.6928118Z      _4 = const get_bool(move _5) -> bb3;
2020-04-07T16:32:49.6928425Z bb3: {
2020-04-07T16:32:49.6928577Z      StorageDead(_5);
2020-04-07T16:32:49.6928577Z      StorageDead(_5);
2020-04-07T16:32:49.6928990Z      switchInt(_4) -> [false: bb4, otherwise: bb5];
2020-04-07T16:32:49.6929330Z bb4: {
2020-04-07T16:32:49.6929479Z      StorageDead(_4);
2020-04-07T16:32:49.6929651Z      StorageDead(_2);
2020-04-07T16:32:49.6930174Z      goto -> bb0;
---
2020-04-07T16:32:49.6931348Z  bb6: {
2020-04-07T16:32:49.6931503Z      StorageDead(_2);
2020-04-07T16:32:49.6931665Z      return;
2020-04-07T16:32:49.6931795Z  }
2020-04-07T16:32:49.6931933Z Actual:
2020-04-07T16:32:49.6932276Z fn while_loop(_1: bool) -> () {
2020-04-07T16:32:49.6932481Z     debug c => _1;
2020-04-07T16:32:49.6932677Z     let mut _0: ();
2020-04-07T16:32:49.6932876Z     let mut _2: bool;
2020-04-07T16:32:49.6933070Z     let mut _3: bool;
2020-04-07T16:32:49.6933279Z     let mut _4: bool;
2020-04-07T16:32:49.6933471Z     let mut _5: bool;
2020-04-07T16:32:49.6933645Z     bb0: {
2020-04-07T16:32:49.6934069Z         StorageLive(_2);
2020-04-07T16:32:49.6934557Z         StorageLive(_3);
2020-04-07T16:32:49.6934723Z         _3 = _1;
2020-04-07T16:32:49.6935141Z         _2 = const get_bool(move _3) -> bb1;
2020-04-07T16:32:49.6935474Z     bb1: {
2020-04-07T16:32:49.6935639Z         StorageDead(_3);
2020-04-07T16:32:49.6935639Z         StorageDead(_3);
2020-04-07T16:32:49.6936085Z         switchInt(_2) -> [false: bb2, otherwise: bb3];
2020-04-07T16:32:49.6936432Z     bb2: {
2020-04-07T16:32:49.6936604Z         _0 = const ();
2020-04-07T16:32:49.6937065Z         goto -> bb7;
2020-04-07T16:32:49.6937213Z     }
2020-04-07T16:32:49.6937213Z     }
2020-04-07T16:32:49.6937347Z     bb3: {
2020-04-07T16:32:49.6937534Z         StorageLive(_4);
2020-04-07T16:32:49.6937721Z         StorageLive(_5);
2020-04-07T16:32:49.6937984Z         _5 = _1;
2020-04-07T16:32:49.6938424Z         _4 = const get_bool(move _5) -> bb4;
2020-04-07T16:32:49.6938755Z     bb4: {
2020-04-07T16:32:49.6938941Z         StorageDead(_5);
2020-04-07T16:32:49.6938941Z         StorageDead(_5);
2020-04-07T16:32:49.6939390Z         switchInt(_4) -> [false: bb5, otherwise: bb6];
2020-04-07T16:32:49.6939764Z     bb5: {
2020-04-07T16:32:49.6939928Z         StorageDead(_4);
2020-04-07T16:32:49.6940115Z         StorageDead(_2);
2020-04-07T16:32:49.6940443Z         goto -> bb0;
---
2020-04-07T16:32:49.6957388Z test result: FAILED. 60 passed; 29 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-07T16:32:49.6957653Z 
2020-04-07T16:32:49.6957748Z 
2020-04-07T16:32:49.6957842Z 
2020-04-07T16:32:49.6962064Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-07T16:32:49.6965499Z 
2020-04-07T16:32:49.6965627Z 
2020-04-07T16:32:49.6966352Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-07T16:32:49.6966849Z Build completed unsuccessfully in 1:03:12
2020-04-07T16:32:49.6966849Z Build completed unsuccessfully in 1:03:12
2020-04-07T16:32:49.6967114Z == clock drift check ==
2020-04-07T16:32:49.6967365Z   local time: Tue Apr  7 16:32:49 UTC 2020
2020-04-07T16:32:49.7149461Z   network time: Tue, 07 Apr 2020 16:32:49 GMT
2020-04-07T16:32:51.9307041Z 
2020-04-07T16:32:51.9307041Z 
2020-04-07T16:32:51.9393927Z ##[error]Bash exited with code '1'.
2020-04-07T16:32:51.9410151Z ##[section]Finishing: Run build
2020-04-07T16:32:51.9467841Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70891/merge to s
2020-04-07T16:32:51.9474351Z Task         : Get sources
2020-04-07T16:32:51.9474860Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T16:32:51.9475187Z Version      : 1.0.0
2020-04-07T16:32:51.9475439Z Author       : Microsoft
2020-04-07T16:32:51.9475439Z Author       : Microsoft
2020-04-07T16:32:51.9475796Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T16:32:51.9476211Z ==============================================================================
2020-04-07T16:32:52.2587805Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T16:32:52.2630209Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70891/merge to s
2020-04-07T16:32:52.2716452Z Cleaning up task key
2020-04-07T16:32:52.2717742Z Start cleaning up orphan processes.
2020-04-07T16:32:52.2935557Z Terminate orphan process: pid (3299) (python)
2020-04-07T16:32:52.3146686Z ##[section]Finishing: Finalize Job
