plain
2020-03-17T18:07:57.8725250Z ========================== Starting Command Output ===========================
2020-03-17T18:07:57.8727980Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4bc62554-6d80-4d47-899e-95e4929918fd.sh
2020-03-17T18:07:57.8728315Z 
2020-03-17T18:07:57.8733401Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T18:07:57.8753214Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-17T18:07:57.8757125Z Task         : Get sources
2020-03-17T18:07:57.8757413Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T18:07:57.8757688Z Version      : 1.0.0
2020-03-17T18:07:57.8757926Z Author       : Microsoft
---
2020-03-17T18:07:58.9045834Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T18:07:58.9051157Z ##[command]git config gc.auto 0
2020-03-17T18:07:58.9055098Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T18:07:58.9058424Z ##[command]git config --get-all http.proxy
2020-03-17T18:07:58.9064117Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70043/merge:refs/remotes/pull/70043/merge
---
2020-03-17T19:03:56.8665819Z .................................................................................................... 1100/9795
2020-03-17T19:04:04.7874776Z .....................................i.............................................................. 1200/9795
2020-03-17T19:04:11.8587228Z .................................................................................................... 1300/9795
2020-03-17T19:04:15.8177705Z .................................................................................................... 1400/9795
2020-03-17T19:04:21.6076099Z .........FFF............F..................F...................................F.......F...........F 1500/9795
2020-03-17T19:04:27.3195452Z .F...................F.....F..................F..FF................................................. 1600/9795
2020-03-17T19:04:32.3112545Z ............F...................................................FF...F.F...F.......F.....FF..F.F.... 1700/9795
2020-03-17T19:04:36.2979150Z .....F...FF....................F...............................................F..........FF..FFFF.. 1800/9795
2020-03-17T19:04:47.1420610Z ....FF....................................................................i......................... 1900/9795
2020-03-17T19:04:53.2941190Z .................................................................................................... 2000/9795
2020-03-17T19:05:00.9802948Z ................................................................iiiii............................... 2100/9795
2020-03-17T19:05:18.2022285Z .............F...................................................................................... 2300/9795
2020-03-17T19:05:20.3839368Z .................................................F.................................................. 2400/9795
2020-03-17T19:05:23.3123371Z .................................................................................................... 2500/9795
2020-03-17T19:05:43.1486484Z .................................................................................................... 2600/9795
---
2020-03-17T19:08:16.4285416Z ....................................i...............i............................................... 5000/9795
2020-03-17T19:08:25.1125648Z ...................................................................................................F 5100/9795
2020-03-17T19:08:31.2008451Z .......................................................F.......................i.................... 5200/9795
2020-03-17T19:08:36.4825393Z .................................................................................................... 5300/9795
2020-03-17T19:08:46.2497415Z ............................................................ii.ii........i...i...................... 5400/9795
2020-03-17T19:08:50.2715087Z ...................................................................................................i 5500/9795
2020-03-17T19:09:03.1108641Z .................................................................................................... 5700/9795
2020-03-17T19:09:09.5311816Z .....................................................i.............................................. 5800/9795
2020-03-17T19:09:15.7360094Z .................................................................................................... 5900/9795
2020-03-17T19:09:24.9241260Z .....................................................F.............................................. 6000/9795
2020-03-17T19:09:24.9241260Z .....................................................F.............................................. 6000/9795
2020-03-17T19:09:30.8539396Z ...............................................ii...i..ii...........i............................... 6100/9795
2020-03-17T19:09:50.3214506Z .................................................................................................... 6300/9795
2020-03-17T19:09:56.9653022Z .................................................................................................... 6400/9795
2020-03-17T19:09:56.9653022Z .................................................................................................... 6400/9795
2020-03-17T19:10:05.1715545Z .............................................................................i..ii.................. 6500/9795
2020-03-17T19:10:28.8071570Z .................................................................................................... 6700/9795
2020-03-17T19:10:37.6219418Z ...........................................................................i........................ 6800/9795
2020-03-17T19:10:39.6335118Z .................................................................................................... 6900/9795
2020-03-17T19:10:41.6468942Z .................................................................................................... 7000/9795
---
2020-03-17T19:12:21.3743589Z .................................................................................................... 7800/9795
2020-03-17T19:12:26.4166600Z .................................................................................................... 7900/9795
2020-03-17T19:12:32.4614671Z ............................................................i....................................... 8000/9795
2020-03-17T19:12:42.3891233Z .................................................................................................... 8100/9795
2020-03-17T19:12:47.9714932Z .........iiiiiiiiii.i........F...................................................................... 8200/9795
2020-03-17T19:13:01.2254489Z .................................................................................................... 8400/9795
2020-03-17T19:13:08.6894242Z .................................................................................................... 8500/9795
2020-03-17T19:13:22.4167017Z .................................................................................................... 8600/9795
2020-03-17T19:13:28.8447140Z .................................................................................................... 8700/9795
---
2020-03-17T19:15:17.1369630Z ---- [ui] ui/array-slice-vec/subslice-patterns-const-eval.rs stdout ----
2020-03-17T19:15:17.1370100Z 
2020-03-17T19:15:17.1370684Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1371082Z status: exit code: 101
2020-03-17T19:15:17.1375081Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/subslice-patterns-const-eval.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/subslice-patterns-const-eval/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/subslice-patterns-const-eval/auxiliary"
2020-03-17T19:15:17.1376988Z ------------------------------------------
2020-03-17T19:15:17.1377294Z 
2020-03-17T19:15:17.1377805Z ------------------------------------------
2020-03-17T19:15:17.1378137Z stderr:
---
2020-03-17T19:15:17.1380667Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1380952Z 
2020-03-17T19:15:17.1381318Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1381667Z 
2020-03-17T19:15:17.1382576Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1384178Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1384544Z 
2020-03-17T19:15:17.1384544Z 
2020-03-17T19:15:17.1385278Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1385929Z 
2020-03-17T19:15:17.1386528Z ------------------------------------------
2020-03-17T19:15:17.1386839Z 
2020-03-17T19:15:17.1387032Z 
2020-03-17T19:15:17.1387032Z 
2020-03-17T19:15:17.1387610Z ---- [ui] ui/array-slice-vec/subslice-patterns-const-eval-match.rs stdout ----
2020-03-17T19:15:17.1393581Z 
2020-03-17T19:15:17.1394446Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1394899Z status: exit code: 101
2020-03-17T19:15:17.1397810Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/subslice-patterns-const-eval-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/subslice-patterns-const-eval-match/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/subslice-patterns-const-eval-match/auxiliary"
2020-03-17T19:15:17.1405592Z ------------------------------------------
2020-03-17T19:15:17.1405845Z 
2020-03-17T19:15:17.1406294Z ------------------------------------------
2020-03-17T19:15:17.1406729Z stderr:
---
2020-03-17T19:15:17.1409236Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1409409Z 
2020-03-17T19:15:17.1409601Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1409795Z 
2020-03-17T19:15:17.1410357Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1411252Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1411486Z 
2020-03-17T19:15:17.1411486Z 
2020-03-17T19:15:17.1412354Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1412803Z 
2020-03-17T19:15:17.1413176Z ------------------------------------------
2020-03-17T19:15:17.1413336Z 
2020-03-17T19:15:17.1413426Z 
2020-03-17T19:15:17.1413426Z 
2020-03-17T19:15:17.1413813Z ---- [ui] ui/associated-const/issue-63496.rs stdout ----
2020-03-17T19:15:17.1414005Z 
2020-03-17T19:15:17.1414244Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1414796Z status: exit code: 101
2020-03-17T19:15:17.1416683Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/issue-63496.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/issue-63496" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/issue-63496/auxiliary"
2020-03-17T19:15:17.1418830Z ------------------------------------------
2020-03-17T19:15:17.1418989Z 
2020-03-17T19:15:17.1419501Z ------------------------------------------
2020-03-17T19:15:17.1419683Z stderr:
---
2020-03-17T19:15:17.1421893Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1422085Z 
2020-03-17T19:15:17.1422293Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1422481Z 
2020-03-17T19:15:17.1423099Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1423978Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1424211Z 
2020-03-17T19:15:17.1424211Z 
2020-03-17T19:15:17.1425557Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1426014Z 
2020-03-17T19:15:17.1426541Z ------------------------------------------
2020-03-17T19:15:17.1426705Z 
2020-03-17T19:15:17.1426796Z 
2020-03-17T19:15:17.1426796Z 
2020-03-17T19:15:17.1427168Z ---- [ui] ui/associated-item/issue-48027.rs stdout ----
2020-03-17T19:15:17.1427373Z 
2020-03-17T19:15:17.1427624Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1427917Z status: exit code: 101
2020-03-17T19:15:17.1430071Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-item/issue-48027.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-item/issue-48027" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-item/issue-48027/auxiliary"
2020-03-17T19:15:17.1431518Z ------------------------------------------
2020-03-17T19:15:17.1431675Z 
2020-03-17T19:15:17.1431993Z ------------------------------------------
2020-03-17T19:15:17.1432190Z stderr:
---
2020-03-17T19:15:17.1462335Z    |       --- this trait cannot be made into an object...
2020-03-17T19:15:17.1462638Z LL |     const X: usize;
2020-03-17T19:15:17.1463115Z    |           - ...because it contains this associated `const`
2020-03-17T19:15:17.1463364Z ...
2020-03-17T19:15:17.1463689Z LL | impl dyn Bar {} //~ ERROR: the trait `Bar` cannot be made into an object
2020-03-17T19:15:17.1464094Z    |      ^^^^^^^ the trait `Bar` cannot be made into an object
2020-03-17T19:15:17.1464566Z    = help: consider moving `X` to another trait
2020-03-17T19:15:17.1464759Z 
2020-03-17T19:15:17.1465824Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc/hir/map/mod.rs:271:20
2020-03-17T19:15:17.1466297Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-17T19:15:17.1466297Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-17T19:15:17.1466524Z 
2020-03-17T19:15:17.1466726Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1466911Z 
2020-03-17T19:15:17.1467132Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1467320Z 
2020-03-17T19:15:17.1467922Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1468873Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1469118Z 
2020-03-17T19:15:17.1469118Z 
2020-03-17T19:15:17.1469716Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1470256Z error: aborting due to previous error
2020-03-17T19:15:17.1470413Z 
2020-03-17T19:15:17.1470922Z For more information about this error, try `rustc --explain E0038`.
2020-03-17T19:15:17.1471136Z 
2020-03-17T19:15:17.1471136Z 
2020-03-17T19:15:17.1471496Z ------------------------------------------
2020-03-17T19:15:17.1471661Z 
2020-03-17T19:15:17.1471754Z 
2020-03-17T19:15:17.1472148Z ---- [ui] ui/associated-type-bounds/lcsit.rs stdout ----
2020-03-17T19:15:17.1472337Z 
2020-03-17T19:15:17.1472706Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1472973Z status: exit code: 101
2020-03-17T19:15:17.1474762Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/lcsit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/lcsit/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/lcsit/auxiliary"
2020-03-17T19:15:17.1476254Z ------------------------------------------
2020-03-17T19:15:17.1476423Z 
2020-03-17T19:15:17.1476784Z ------------------------------------------
2020-03-17T19:15:17.1476979Z stderr:
---
2020-03-17T19:15:17.1481195Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1481386Z 
2020-03-17T19:15:17.1481599Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1481811Z 
2020-03-17T19:15:17.1482584Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1483394Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1483654Z 
2020-03-17T19:15:17.1483654Z 
2020-03-17T19:15:17.1484289Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1484775Z 
2020-03-17T19:15:17.1485140Z ------------------------------------------
2020-03-17T19:15:17.1485318Z 
2020-03-17T19:15:17.1485423Z 
2020-03-17T19:15:17.1485423Z 
2020-03-17T19:15:17.1485898Z ---- [ui] ui/associated-types/associated-types-unconstrained.rs stdout ----
2020-03-17T19:15:17.1486133Z 
2020-03-17T19:15:17.1486457Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1486939Z status: exit code: 101
2020-03-17T19:15:17.1489368Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-unconstrained.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained/auxiliary"
2020-03-17T19:15:17.1491243Z ------------------------------------------
2020-03-17T19:15:17.1491739Z 
2020-03-17T19:15:17.1493048Z ------------------------------------------
2020-03-17T19:15:17.1493263Z stderr:
---
2020-03-17T19:15:17.1495450Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1495641Z 
2020-03-17T19:15:17.1495854Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1496047Z 
2020-03-17T19:15:17.1496667Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1499444Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1499722Z 
2020-03-17T19:15:17.1499722Z 
2020-03-17T19:15:17.1500364Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1500816Z 
2020-03-17T19:15:17.1501189Z ------------------------------------------
2020-03-17T19:15:17.1501360Z 
2020-03-17T19:15:17.1501456Z 
2020-03-17T19:15:17.1501456Z 
2020-03-17T19:15:17.1508836Z ---- [ui] ui/consts/const-address-of-interior-mut.rs stdout ----
2020-03-17T19:15:17.1509109Z 
2020-03-17T19:15:17.1509380Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1509693Z status: exit code: 101
2020-03-17T19:15:17.1512483Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-address-of-interior-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-address-of-interior-mut" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-address-of-interior-mut/auxiliary"
2020-03-17T19:15:17.1514550Z ------------------------------------------
2020-03-17T19:15:17.1514743Z 
2020-03-17T19:15:17.1515119Z ------------------------------------------
2020-03-17T19:15:17.1515348Z stderr:
---
2020-03-17T19:15:17.1518550Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1518743Z 
2020-03-17T19:15:17.1518974Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1519169Z 
2020-03-17T19:15:17.1519855Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1520646Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1520896Z 
2020-03-17T19:15:17.1520896Z 
2020-03-17T19:15:17.1521531Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1522161Z 
2020-03-17T19:15:17.1522545Z ------------------------------------------
2020-03-17T19:15:17.1522736Z 
2020-03-17T19:15:17.1522830Z 
2020-03-17T19:15:17.1522830Z 
2020-03-17T19:15:17.1523386Z ---- [ui] ui/consts/const-address-of-mut.rs stdout ----
2020-03-17T19:15:17.1523573Z 
2020-03-17T19:15:17.1523918Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1532607Z status: exit code: 101
2020-03-17T19:15:17.1535619Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-address-of-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-address-of-mut" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-address-of-mut/auxiliary"
2020-03-17T19:15:17.1537518Z ------------------------------------------
2020-03-17T19:15:17.1537717Z 
2020-03-17T19:15:17.1538082Z ------------------------------------------
2020-03-17T19:15:17.1538283Z stderr:
---
2020-03-17T19:15:17.1540242Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1540435Z 
2020-03-17T19:15:17.1540648Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1540860Z 
2020-03-17T19:15:17.1541478Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1542263Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1542512Z 
2020-03-17T19:15:17.1542512Z 
2020-03-17T19:15:17.1543129Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1543604Z 
2020-03-17T19:15:17.1543957Z ------------------------------------------
2020-03-17T19:15:17.1544143Z 
2020-03-17T19:15:17.1544239Z 
2020-03-17T19:15:17.1544239Z 
2020-03-17T19:15:17.1544612Z ---- [ui] ui/consts/const-address-of.rs stdout ----
2020-03-17T19:15:17.1544800Z 
2020-03-17T19:15:17.1545181Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1545460Z status: exit code: 101
2020-03-17T19:15:17.1547570Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-address-of.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-address-of" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-address-of/auxiliary"
2020-03-17T19:15:17.1550986Z ------------------------------------------
2020-03-17T19:15:17.1551196Z 
2020-03-17T19:15:17.1552261Z ------------------------------------------
2020-03-17T19:15:17.1552470Z stderr:
---
2020-03-17T19:15:17.1554930Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1555122Z 
2020-03-17T19:15:17.1555336Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1555530Z 
2020-03-17T19:15:17.1556213Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1557285Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1557554Z 
2020-03-17T19:15:17.1557554Z 
2020-03-17T19:15:17.1558222Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1558708Z 
2020-03-17T19:15:17.1559076Z ------------------------------------------
2020-03-17T19:15:17.1559254Z 
2020-03-17T19:15:17.1559354Z 
2020-03-17T19:15:17.1559354Z 
2020-03-17T19:15:17.1559794Z ---- [ui] ui/consts/const-block-non-item-statement.rs stdout ----
2020-03-17T19:15:17.1560024Z 
2020-03-17T19:15:17.1560631Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1560903Z status: exit code: 101
2020-03-17T19:15:17.1563361Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-block-non-item-statement.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-block-non-item-statement" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-block-non-item-statement/auxiliary"
2020-03-17T19:15:17.1564960Z ------------------------------------------
2020-03-17T19:15:17.1565136Z 
2020-03-17T19:15:17.1565629Z ------------------------------------------
2020-03-17T19:15:17.1565853Z stderr:
---
2020-03-17T19:15:17.1567723Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1567925Z 
2020-03-17T19:15:17.1568328Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1568524Z 
2020-03-17T19:15:17.1569285Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1570049Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1570297Z 
2020-03-17T19:15:17.1570297Z 
2020-03-17T19:15:17.1570928Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1571655Z 
2020-03-17T19:15:17.1573437Z ------------------------------------------
2020-03-17T19:15:17.1573645Z 
2020-03-17T19:15:17.1573738Z 
2020-03-17T19:15:17.1573738Z 
2020-03-17T19:15:17.1574116Z ---- [ui] ui/consts/const-enum-cast.rs stdout ----
2020-03-17T19:15:17.1574296Z 
2020-03-17T19:15:17.1574684Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1574948Z status: exit code: 101
2020-03-17T19:15:17.1577596Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-enum-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-enum-cast/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-enum-cast/auxiliary"
2020-03-17T19:15:17.1580442Z ------------------------------------------
2020-03-17T19:15:17.1580649Z 
2020-03-17T19:15:17.1581023Z ------------------------------------------
2020-03-17T19:15:17.1581231Z stderr:
---
2020-03-17T19:15:17.1583820Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1584005Z 
2020-03-17T19:15:17.1584212Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1584417Z 
2020-03-17T19:15:17.1585046Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1585805Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1586045Z 
2020-03-17T19:15:17.1586045Z 
2020-03-17T19:15:17.1586692Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1587144Z 
2020-03-17T19:15:17.1587492Z ------------------------------------------
2020-03-17T19:15:17.1587658Z 
2020-03-17T19:15:17.1587750Z 
2020-03-17T19:15:17.1587750Z 
2020-03-17T19:15:17.1588283Z ---- [ui] ui/consts/const-eval/const_let.rs stdout ----
2020-03-17T19:15:17.1588479Z 
2020-03-17T19:15:17.1588728Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1589035Z status: exit code: 101
2020-03-17T19:15:17.1591345Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let/auxiliary"
2020-03-17T19:15:17.1592905Z ------------------------------------------
2020-03-17T19:15:17.1593074Z 
2020-03-17T19:15:17.1594000Z ------------------------------------------
2020-03-17T19:15:17.1594428Z stderr:
---
2020-03-17T19:15:17.1598991Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1599200Z 
2020-03-17T19:15:17.1599416Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1599611Z 
2020-03-17T19:15:17.1600244Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1601014Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1601268Z 
2020-03-17T19:15:17.1601268Z 
2020-03-17T19:15:17.1601905Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1602357Z 
2020-03-17T19:15:17.1602726Z ------------------------------------------
2020-03-17T19:15:17.1602898Z 
2020-03-17T19:15:17.1602993Z 
2020-03-17T19:15:17.1602993Z 
2020-03-17T19:15:17.1603372Z ---- [ui] ui/consts/const-eval/dangling.rs stdout ----
2020-03-17T19:15:17.1603709Z 
2020-03-17T19:15:17.1603988Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1604292Z status: exit code: 101
2020-03-17T19:15:17.1606344Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/dangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dangling" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dangling/auxiliary"
2020-03-17T19:15:17.1608742Z ------------------------------------------
2020-03-17T19:15:17.1608926Z 
2020-03-17T19:15:17.1609286Z ------------------------------------------
2020-03-17T19:15:17.1609498Z stderr:
---
2020-03-17T19:15:17.1612837Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1613038Z 
2020-03-17T19:15:17.1613269Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1613466Z 
2020-03-17T19:15:17.1614416Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1615235Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1615485Z 
2020-03-17T19:15:17.1615485Z 
2020-03-17T19:15:17.1616103Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1616579Z 
2020-03-17T19:15:17.1616934Z ------------------------------------------
2020-03-17T19:15:17.1617105Z 
2020-03-17T19:15:17.1617216Z 
2020-03-17T19:15:17.1617216Z 
2020-03-17T19:15:17.1617840Z ---- [ui] ui/consts/const-eval/generic-slice.rs stdout ----
2020-03-17T19:15:17.1618048Z 
2020-03-17T19:15:17.1618650Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1619273Z status: exit code: 101
2020-03-17T19:15:17.1622798Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/generic-slice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice/auxiliary"
2020-03-17T19:15:17.1624542Z ------------------------------------------
2020-03-17T19:15:17.1624723Z 
2020-03-17T19:15:17.1625257Z ------------------------------------------
2020-03-17T19:15:17.1625457Z stderr:
---
2020-03-17T19:15:17.1627395Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1627586Z 
2020-03-17T19:15:17.1627800Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1627995Z 
2020-03-17T19:15:17.1628782Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1629536Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1629801Z 
2020-03-17T19:15:17.1629801Z 
2020-03-17T19:15:17.1630419Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1630868Z 
2020-03-17T19:15:17.1631320Z ------------------------------------------
2020-03-17T19:15:17.1631499Z 
2020-03-17T19:15:17.1631593Z 
2020-03-17T19:15:17.1631593Z 
2020-03-17T19:15:17.1632054Z ---- [ui] ui/consts/const-eval/index-out-of-bounds-never-type.rs stdout ----
2020-03-17T19:15:17.1632304Z 
2020-03-17T19:15:17.1632690Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1632950Z status: exit code: 101
2020-03-17T19:15:17.1635074Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index-out-of-bounds-never-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index-out-of-bounds-never-type/auxiliary"
2020-03-17T19:15:17.1636788Z ------------------------------------------
2020-03-17T19:15:17.1636964Z 
2020-03-17T19:15:17.1637317Z ------------------------------------------
2020-03-17T19:15:17.1637534Z stderr:
---
2020-03-17T19:15:17.1639457Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1639648Z 
2020-03-17T19:15:17.1639880Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1640075Z 
2020-03-17T19:15:17.1640674Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1641448Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1641697Z 
2020-03-17T19:15:17.1641697Z 
2020-03-17T19:15:17.1642330Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1642780Z 
2020-03-17T19:15:17.1643133Z ------------------------------------------
2020-03-17T19:15:17.1643318Z 
2020-03-17T19:15:17.1643419Z 
2020-03-17T19:15:17.1643419Z 
2020-03-17T19:15:17.1643968Z ---- [ui] ui/consts/const-eval/issue-65394.rs stdout ----
2020-03-17T19:15:17.1644159Z 
2020-03-17T19:15:17.1644424Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1644717Z status: exit code: 101
2020-03-17T19:15:17.1646611Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-65394.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-65394" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-65394/auxiliary"
2020-03-17T19:15:17.1648264Z ------------------------------------------
2020-03-17T19:15:17.1648449Z 
2020-03-17T19:15:17.1648792Z ------------------------------------------
2020-03-17T19:15:17.1648985Z stderr:
---
2020-03-17T19:15:17.1652600Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1652786Z 
2020-03-17T19:15:17.1652993Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1653198Z 
2020-03-17T19:15:17.1653887Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1654638Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1654885Z 
2020-03-17T19:15:17.1654885Z 
2020-03-17T19:15:17.1655482Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1655932Z 
2020-03-17T19:15:17.1656273Z ------------------------------------------
2020-03-17T19:15:17.1656438Z 
2020-03-17T19:15:17.1656531Z 
2020-03-17T19:15:17.1656531Z 
2020-03-17T19:15:17.1656931Z ---- [ui] ui/consts/const-eval/promote-static.rs stdout ----
2020-03-17T19:15:17.1657311Z 
2020-03-17T19:15:17.1657699Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1657961Z status: exit code: 101
2020-03-17T19:15:17.1660084Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promote-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promote-static" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promote-static/auxiliary"
2020-03-17T19:15:17.1661850Z ------------------------------------------
2020-03-17T19:15:17.1662025Z 
2020-03-17T19:15:17.1662396Z ------------------------------------------
2020-03-17T19:15:17.1662601Z stderr:
---
2020-03-17T19:15:17.1664522Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1664884Z 
2020-03-17T19:15:17.1665089Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1665284Z 
2020-03-17T19:15:17.1665871Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1666786Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1667034Z 
2020-03-17T19:15:17.1667034Z 
2020-03-17T19:15:17.1667669Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1668120Z 
2020-03-17T19:15:17.1668490Z ------------------------------------------
2020-03-17T19:15:17.1668660Z 
2020-03-17T19:15:17.1668755Z 
2020-03-17T19:15:17.1668755Z 
2020-03-17T19:15:17.1669300Z ---- [ui] ui/consts/const-eval/ub-nonnull.rs stdout ----
2020-03-17T19:15:17.1669488Z 
2020-03-17T19:15:17.1669755Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1670046Z status: exit code: 101
2020-03-17T19:15:17.1672224Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary"
2020-03-17T19:15:17.1673796Z ------------------------------------------
2020-03-17T19:15:17.1673965Z 
2020-03-17T19:15:17.1674309Z ------------------------------------------
2020-03-17T19:15:17.1674685Z stderr:
---
2020-03-17T19:15:17.1676739Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1676923Z 
2020-03-17T19:15:17.1677144Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1677509Z 
2020-03-17T19:15:17.1678104Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1678877Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1679126Z 
2020-03-17T19:15:17.1679126Z 
2020-03-17T19:15:17.1679741Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1680362Z 
2020-03-17T19:15:17.1680702Z ------------------------------------------
2020-03-17T19:15:17.1680873Z 
2020-03-17T19:15:17.1680981Z 
2020-03-17T19:15:17.1680981Z 
2020-03-17T19:15:17.1681352Z ---- [ui] ui/consts/const-eval/ub-upvars.rs stdout ----
2020-03-17T19:15:17.1681539Z 
2020-03-17T19:15:17.1681785Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1682093Z status: exit code: 101
2020-03-17T19:15:17.1683964Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-upvars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars/auxiliary"
2020-03-17T19:15:17.1685566Z ------------------------------------------
2020-03-17T19:15:17.1685738Z 
2020-03-17T19:15:17.1686100Z ------------------------------------------
2020-03-17T19:15:17.1686292Z stderr:
---
2020-03-17T19:15:17.1688161Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1688345Z 
2020-03-17T19:15:17.1688550Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1688737Z 
2020-03-17T19:15:17.1689324Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1690050Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1690403Z 
2020-03-17T19:15:17.1690403Z 
2020-03-17T19:15:17.1691018Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1691451Z 
2020-03-17T19:15:17.1691808Z ------------------------------------------
2020-03-17T19:15:17.1695282Z 
2020-03-17T19:15:17.1695391Z 
2020-03-17T19:15:17.1695391Z 
2020-03-17T19:15:17.1696057Z ---- [ui] ui/consts/const-eval/ub-wide-ptr.rs stdout ----
2020-03-17T19:15:17.1696291Z 
2020-03-17T19:15:17.1696986Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1697651Z status: exit code: 101
2020-03-17T19:15:17.1699649Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/auxiliary"
2020-03-17T19:15:17.1701230Z ------------------------------------------
2020-03-17T19:15:17.1701399Z 
2020-03-17T19:15:17.1701748Z ------------------------------------------
2020-03-17T19:15:17.1701960Z stderr:
---
2020-03-17T19:15:17.1703819Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1704010Z 
2020-03-17T19:15:17.1704233Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1705043Z 
2020-03-17T19:15:17.1705718Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1706531Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1706791Z 
2020-03-17T19:15:17.1706791Z 
2020-03-17T19:15:17.1707455Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1707924Z 
2020-03-17T19:15:17.1708292Z ------------------------------------------
2020-03-17T19:15:17.1708639Z 
2020-03-17T19:15:17.1708734Z 
2020-03-17T19:15:17.1708734Z 
2020-03-17T19:15:17.1709106Z ---- [ui] ui/consts/const-multi-ref.rs stdout ----
2020-03-17T19:15:17.1709292Z 
2020-03-17T19:15:17.1709630Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1710123Z status: exit code: 101
2020-03-17T19:15:17.1712334Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-multi-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-multi-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-multi-ref/auxiliary"
2020-03-17T19:15:17.1714020Z ------------------------------------------
2020-03-17T19:15:17.1714205Z 
2020-03-17T19:15:17.1714546Z ------------------------------------------
2020-03-17T19:15:17.1714739Z stderr:
---
2020-03-17T19:15:17.1716773Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1716958Z 
2020-03-17T19:15:17.1717164Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1717352Z 
2020-03-17T19:15:17.1718202Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1719338Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1719588Z 
2020-03-17T19:15:17.1719588Z 
2020-03-17T19:15:17.1720205Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1720820Z 
2020-03-17T19:15:17.1721577Z ------------------------------------------
2020-03-17T19:15:17.1721923Z 
2020-03-17T19:15:17.1722019Z 
2020-03-17T19:15:17.1722019Z 
2020-03-17T19:15:17.1722490Z ---- [ui] ui/consts/const_constructor/const-construct-call.rs#const_fn stdout ----
2020-03-17T19:15:17.1722728Z 
2020-03-17T19:15:17.1723183Z error in revision `const_fn`: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1723492Z status: exit code: 101
2020-03-17T19:15:17.1726040Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_constructor/const-construct-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "const_fn" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_constructor/const-construct-call.const_fn/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_constructor/const-construct-call.const_fn/auxiliary"
2020-03-17T19:15:17.1727946Z ------------------------------------------
2020-03-17T19:15:17.1728122Z 
2020-03-17T19:15:17.1728699Z ------------------------------------------
2020-03-17T19:15:17.1728907Z stderr:
---
2020-03-17T19:15:17.1730901Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1731117Z 
2020-03-17T19:15:17.1731340Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1731544Z 
2020-03-17T19:15:17.1732399Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1733725Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1733975Z 
2020-03-17T19:15:17.1733975Z 
2020-03-17T19:15:17.1734610Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1735060Z 
2020-03-17T19:15:17.1735428Z ------------------------------------------
2020-03-17T19:15:17.1735599Z 
2020-03-17T19:15:17.1735700Z 
2020-03-17T19:15:17.1735700Z 
2020-03-17T19:15:17.1736164Z ---- [ui] ui/consts/const_constructor/const-construct-call.rs#min_const_fn stdout ----
2020-03-17T19:15:17.1736411Z 
2020-03-17T19:15:17.1736889Z error in revision `min_const_fn`: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1737203Z status: exit code: 101
2020-03-17T19:15:17.1739838Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_constructor/const-construct-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_const_fn" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_constructor/const-construct-call.min_const_fn/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_constructor/const-construct-call.min_const_fn/auxiliary"
2020-03-17T19:15:17.1741682Z ------------------------------------------
2020-03-17T19:15:17.1741859Z 
2020-03-17T19:15:17.1742544Z ------------------------------------------
2020-03-17T19:15:17.1742767Z stderr:
---
2020-03-17T19:15:17.1745212Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1745412Z 
2020-03-17T19:15:17.1745650Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1745852Z 
2020-03-17T19:15:17.1746629Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1747540Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1747782Z 
2020-03-17T19:15:17.1747782Z 
2020-03-17T19:15:17.1748377Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1748828Z 
2020-03-17T19:15:17.1749168Z ------------------------------------------
2020-03-17T19:15:17.1749333Z 
2020-03-17T19:15:17.1749616Z 
2020-03-17T19:15:17.1749616Z 
2020-03-17T19:15:17.1749993Z ---- [ui] ui/consts/const_let_assign.rs stdout ----
2020-03-17T19:15:17.1750367Z 
2020-03-17T19:15:17.1750764Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1751050Z status: exit code: 101
2020-03-17T19:15:17.1752980Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_let_assign.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_assign" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_assign/auxiliary"
2020-03-17T19:15:17.1754685Z ------------------------------------------
2020-03-17T19:15:17.1754859Z 
2020-03-17T19:15:17.1755229Z ------------------------------------------
2020-03-17T19:15:17.1755435Z stderr:
---
2020-03-17T19:15:17.1757547Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1757904Z 
2020-03-17T19:15:17.1758117Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1758311Z 
2020-03-17T19:15:17.1758917Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1759669Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1759933Z 
2020-03-17T19:15:17.1759933Z 
2020-03-17T19:15:17.1760546Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1761106Z 
2020-03-17T19:15:17.1761499Z ------------------------------------------
2020-03-17T19:15:17.1761671Z 
2020-03-17T19:15:17.1761765Z 
2020-03-17T19:15:17.1761765Z 
2020-03-17T19:15:17.1762144Z ---- [ui] ui/consts/const_let_assign3.rs stdout ----
2020-03-17T19:15:17.1762350Z 
2020-03-17T19:15:17.1762606Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1762978Z status: exit code: 101
2020-03-17T19:15:17.1765034Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_let_assign3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_assign3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_assign3/auxiliary"
2020-03-17T19:15:17.1767139Z ------------------------------------------
2020-03-17T19:15:17.1767315Z 
2020-03-17T19:15:17.1767668Z ------------------------------------------
2020-03-17T19:15:17.1767888Z stderr:
---
2020-03-17T19:15:17.1770009Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1770207Z 
2020-03-17T19:15:17.1770445Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1770648Z 
2020-03-17T19:15:17.1771261Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1772225Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1772488Z 
2020-03-17T19:15:17.1772488Z 
2020-03-17T19:15:17.1773148Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1773918Z 
2020-03-17T19:15:17.1774268Z ------------------------------------------
2020-03-17T19:15:17.1774449Z 
2020-03-17T19:15:17.1774541Z 
2020-03-17T19:15:17.1774541Z 
2020-03-17T19:15:17.1774907Z ---- [ui] ui/consts/const_let_promote.rs stdout ----
2020-03-17T19:15:17.1775089Z 
2020-03-17T19:15:17.1775636Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1775897Z status: exit code: 101
2020-03-17T19:15:17.1777706Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_let_promote.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_promote/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_promote/auxiliary"
2020-03-17T19:15:17.1779234Z ------------------------------------------
2020-03-17T19:15:17.1779427Z 
2020-03-17T19:15:17.1779780Z ------------------------------------------
2020-03-17T19:15:17.1779982Z stderr:
---
2020-03-17T19:15:17.1782066Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1782259Z 
2020-03-17T19:15:17.1782472Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1782667Z 
2020-03-17T19:15:17.1783311Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1784064Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1784556Z 
2020-03-17T19:15:17.1784556Z 
2020-03-17T19:15:17.1785391Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1786196Z 
2020-03-17T19:15:17.1786551Z ------------------------------------------
2020-03-17T19:15:17.1786723Z 
2020-03-17T19:15:17.1786817Z 
2020-03-17T19:15:17.1786817Z 
2020-03-17T19:15:17.1787213Z ---- [ui] ui/consts/const_short_circuit.rs stdout ----
2020-03-17T19:15:17.1787405Z 
2020-03-17T19:15:17.1787661Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1787969Z status: exit code: 101
2020-03-17T19:15:17.1789923Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_short_circuit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_short_circuit" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_short_circuit/auxiliary"
2020-03-17T19:15:17.1792041Z ------------------------------------------
2020-03-17T19:15:17.1792209Z 
2020-03-17T19:15:17.1792552Z ------------------------------------------
2020-03-17T19:15:17.1792763Z stderr:
---
2020-03-17T19:15:17.1794757Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1794959Z 
2020-03-17T19:15:17.1795171Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1795358Z 
2020-03-17T19:15:17.1796130Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1796882Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1797131Z 
2020-03-17T19:15:17.1797131Z 
2020-03-17T19:15:17.1797760Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1798216Z 
2020-03-17T19:15:17.1798570Z ------------------------------------------
2020-03-17T19:15:17.1798758Z 
2020-03-17T19:15:17.1798852Z 
2020-03-17T19:15:17.1798852Z 
2020-03-17T19:15:17.1799246Z ---- [ui] ui/consts/control-flow/drop-failure.rs stdout ----
2020-03-17T19:15:17.1799449Z 
2020-03-17T19:15:17.1799720Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1800022Z status: exit code: 101
2020-03-17T19:15:17.1801997Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/drop-failure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-failure" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-failure/auxiliary"
2020-03-17T19:15:17.1804107Z ------------------------------------------
2020-03-17T19:15:17.1804300Z 
2020-03-17T19:15:17.1804655Z ------------------------------------------
2020-03-17T19:15:17.1804857Z stderr:
---
2020-03-17T19:15:17.1806858Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1807049Z 
2020-03-17T19:15:17.1807262Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1807473Z 
2020-03-17T19:15:17.1808079Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1808858Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1809107Z 
2020-03-17T19:15:17.1809107Z 
2020-03-17T19:15:17.1809721Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1810186Z 
2020-03-17T19:15:17.1810543Z ------------------------------------------
2020-03-17T19:15:17.1810715Z 
2020-03-17T19:15:17.1810811Z 
2020-03-17T19:15:17.1810811Z 
2020-03-17T19:15:17.1811224Z ---- [ui] ui/consts/control-flow/drop-success.rs stdout ----
2020-03-17T19:15:17.1811426Z 
2020-03-17T19:15:17.1811809Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1812277Z status: exit code: 101
2020-03-17T19:15:17.1814193Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/drop-success.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-success/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/drop-success/auxiliary"
2020-03-17T19:15:17.1815767Z ------------------------------------------
2020-03-17T19:15:17.1815942Z 
2020-03-17T19:15:17.1816314Z ------------------------------------------
2020-03-17T19:15:17.1816516Z stderr:
---
2020-03-17T19:15:17.1818436Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1818645Z 
2020-03-17T19:15:17.1818857Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1819051Z 
2020-03-17T19:15:17.1819657Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1820410Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1820666Z 
2020-03-17T19:15:17.1820666Z 
2020-03-17T19:15:17.1821301Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1821749Z 
2020-03-17T19:15:17.1822115Z ------------------------------------------
2020-03-17T19:15:17.1822286Z 
2020-03-17T19:15:17.1822380Z 
2020-03-17T19:15:17.1822380Z 
2020-03-17T19:15:17.1822837Z ---- [ui] ui/consts/control-flow/feature-gate-const-if-match.rs#if_match stdout ----
2020-03-17T19:15:17.1823216Z 
2020-03-17T19:15:17.1823523Z error in revision `if_match`: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1823874Z status: exit code: 101
2020-03-17T19:15:17.1826283Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/feature-gate-const-if-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "if_match" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/feature-gate-const-if-match.if_match" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/feature-gate-const-if-match.if_match/auxiliary"
2020-03-17T19:15:17.1828011Z ------------------------------------------
2020-03-17T19:15:17.1828185Z 
2020-03-17T19:15:17.1828527Z ------------------------------------------
2020-03-17T19:15:17.1828738Z stderr:
---
2020-03-17T19:15:17.1830785Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1830975Z 
2020-03-17T19:15:17.1831209Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1831404Z 
2020-03-17T19:15:17.1832147Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1832890Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1833137Z 
2020-03-17T19:15:17.1833137Z 
2020-03-17T19:15:17.1833732Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1834182Z 
2020-03-17T19:15:17.1834522Z ------------------------------------------
2020-03-17T19:15:17.1834701Z 
2020-03-17T19:15:17.1834793Z 
2020-03-17T19:15:17.1834793Z 
2020-03-17T19:15:17.1835191Z ---- [ui] ui/consts/control-flow/interior-mutability.rs stdout ----
2020-03-17T19:15:17.1835398Z 
2020-03-17T19:15:17.1835666Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1835957Z status: exit code: 101
2020-03-17T19:15:17.1837916Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/interior-mutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability/auxiliary"
2020-03-17T19:15:17.1839525Z ------------------------------------------
2020-03-17T19:15:17.1839710Z 
2020-03-17T19:15:17.1840055Z ------------------------------------------
2020-03-17T19:15:17.1840250Z stderr:
---
2020-03-17T19:15:17.1842322Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1842597Z 
2020-03-17T19:15:17.1842990Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1843194Z 
2020-03-17T19:15:17.1843785Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1844527Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1844766Z 
2020-03-17T19:15:17.1844766Z 
2020-03-17T19:15:17.1845586Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1846030Z 
2020-03-17T19:15:17.1846380Z ------------------------------------------
2020-03-17T19:15:17.1846539Z 
2020-03-17T19:15:17.1846629Z 
2020-03-17T19:15:17.1846629Z 
2020-03-17T19:15:17.1847025Z ---- [ui] ui/consts/control-flow/short-circuit-let.rs stdout ----
2020-03-17T19:15:17.1847223Z 
2020-03-17T19:15:17.1847582Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1847825Z status: exit code: 101
2020-03-17T19:15:17.1849636Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/short-circuit-let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/short-circuit-let/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/short-circuit-let/auxiliary"
2020-03-17T19:15:17.1851112Z ------------------------------------------
2020-03-17T19:15:17.1851274Z 
2020-03-17T19:15:17.1851602Z ------------------------------------------
2020-03-17T19:15:17.1851807Z stderr:
---
2020-03-17T19:15:17.1854530Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1854748Z 
2020-03-17T19:15:17.1854969Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1855171Z 
2020-03-17T19:15:17.1855950Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1856712Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1857118Z 
2020-03-17T19:15:17.1857118Z 
2020-03-17T19:15:17.1857732Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1858166Z 
2020-03-17T19:15:17.1858524Z ------------------------------------------
2020-03-17T19:15:17.1858695Z 
2020-03-17T19:15:17.1858787Z 
2020-03-17T19:15:17.1858787Z 
2020-03-17T19:15:17.1859160Z ---- [ui] ui/consts/dangling-alloc-id-ice.rs stdout ----
2020-03-17T19:15:17.1859349Z 
2020-03-17T19:15:17.1859612Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1860079Z status: exit code: 101
2020-03-17T19:15:17.1863368Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/dangling-alloc-id-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/dangling-alloc-id-ice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/dangling-alloc-id-ice/auxiliary"
2020-03-17T19:15:17.1867261Z ------------------------------------------
2020-03-17T19:15:17.1867453Z 
2020-03-17T19:15:17.1867834Z ------------------------------------------
2020-03-17T19:15:17.1868044Z stderr:
---
2020-03-17T19:15:17.1871565Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1871750Z 
2020-03-17T19:15:17.1871959Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1872166Z 
2020-03-17T19:15:17.1872814Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1873571Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1873821Z 
2020-03-17T19:15:17.1873821Z 
2020-03-17T19:15:17.1874416Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1874869Z 
2020-03-17T19:15:17.1875209Z ------------------------------------------
2020-03-17T19:15:17.1875376Z 
2020-03-17T19:15:17.1875468Z 
2020-03-17T19:15:17.1875468Z 
2020-03-17T19:15:17.1875852Z ---- [ui] ui/consts/dangling_raw_ptr.rs stdout ----
2020-03-17T19:15:17.1876033Z 
2020-03-17T19:15:17.1876280Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1876588Z status: exit code: 101
2020-03-17T19:15:17.1878428Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/dangling_raw_ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/dangling_raw_ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/dangling_raw_ptr/auxiliary"
2020-03-17T19:15:17.1880009Z ------------------------------------------
2020-03-17T19:15:17.1880178Z 
2020-03-17T19:15:17.1880543Z ------------------------------------------
2020-03-17T19:15:17.1880739Z stderr:
---
2020-03-17T19:15:17.1882609Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1882800Z 
2020-03-17T19:15:17.1883006Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1883193Z 
2020-03-17T19:15:17.1883785Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1884512Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1884767Z 
2020-03-17T19:15:17.1884767Z 
2020-03-17T19:15:17.1885367Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1885803Z 
2020-03-17T19:15:17.1886313Z ------------------------------------------
2020-03-17T19:15:17.1886473Z 
2020-03-17T19:15:17.1886560Z 
2020-03-17T19:15:17.1886560Z 
2020-03-17T19:15:17.1886900Z ---- [ui] ui/consts/issue-64506.rs stdout ----
2020-03-17T19:15:17.1887081Z 
2020-03-17T19:15:17.1887437Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1887678Z status: exit code: 101
2020-03-17T19:15:17.1889641Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-64506.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-64506" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-64506/auxiliary"
2020-03-17T19:15:17.1891115Z ------------------------------------------
2020-03-17T19:15:17.1891277Z 
2020-03-17T19:15:17.1891607Z ------------------------------------------
2020-03-17T19:15:17.1891794Z stderr:
---
2020-03-17T19:15:17.1893856Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1894035Z 
2020-03-17T19:15:17.1894251Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1894431Z 
2020-03-17T19:15:17.1894994Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1895714Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1895946Z 
2020-03-17T19:15:17.1895946Z 
2020-03-17T19:15:17.1896521Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1896954Z 
2020-03-17T19:15:17.1897284Z ------------------------------------------
2020-03-17T19:15:17.1897448Z 
2020-03-17T19:15:17.1897552Z 
2020-03-17T19:15:17.1897552Z 
2020-03-17T19:15:17.1897914Z ---- [ui] ui/consts/miri_unleashed/drop.rs stdout ----
2020-03-17T19:15:17.1898093Z 
2020-03-17T19:15:17.1898331Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1898627Z status: exit code: 101
2020-03-17T19:15:17.1900509Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/drop" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/drop/auxiliary"
2020-03-17T19:15:17.1902058Z ------------------------------------------
2020-03-17T19:15:17.1902220Z 
2020-03-17T19:15:17.1902569Z ------------------------------------------
2020-03-17T19:15:17.1902757Z stderr:
---
2020-03-17T19:15:17.1904562Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1904740Z 
2020-03-17T19:15:17.1904942Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1905123Z 
2020-03-17T19:15:17.1906027Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1906923Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1907295Z 
2020-03-17T19:15:17.1907295Z 
2020-03-17T19:15:17.1908002Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1908661Z 
2020-03-17T19:15:17.1908993Z ------------------------------------------
2020-03-17T19:15:17.1909310Z 
2020-03-17T19:15:17.1909395Z 
2020-03-17T19:15:17.1909395Z 
2020-03-17T19:15:17.1909819Z ---- [ui] ui/consts/offset_from.rs stdout ----
2020-03-17T19:15:17.1909986Z 
2020-03-17T19:15:17.1910352Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1910585Z status: exit code: 101
2020-03-17T19:15:17.1912193Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from/auxiliary"
2020-03-17T19:15:17.1913535Z ------------------------------------------
2020-03-17T19:15:17.1913692Z 
2020-03-17T19:15:17.1914189Z ------------------------------------------
2020-03-17T19:15:17.1914396Z stderr:
---
2020-03-17T19:15:17.1916187Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1916364Z 
2020-03-17T19:15:17.1916580Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1916766Z 
2020-03-17T19:15:17.1917319Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1918037Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1918269Z 
2020-03-17T19:15:17.1918269Z 
2020-03-17T19:15:17.1918865Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1919286Z 
2020-03-17T19:15:17.1919616Z ------------------------------------------
2020-03-17T19:15:17.1919790Z 
2020-03-17T19:15:17.1919878Z 
2020-03-17T19:15:17.1919878Z 
2020-03-17T19:15:17.1920223Z ---- [ui] ui/consts/offset_from_ub.rs stdout ----
2020-03-17T19:15:17.1920394Z 
2020-03-17T19:15:17.1920646Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1920927Z status: exit code: 101
2020-03-17T19:15:17.1922688Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary"
2020-03-17T19:15:17.1924155Z ------------------------------------------
2020-03-17T19:15:17.1924332Z 
2020-03-17T19:15:17.1924660Z ------------------------------------------
2020-03-17T19:15:17.1924847Z stderr:
---
2020-03-17T19:15:17.1928629Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1928829Z 
2020-03-17T19:15:17.1929050Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1929269Z 
2020-03-17T19:15:17.1930097Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1931258Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1931485Z 
2020-03-17T19:15:17.1931485Z 
2020-03-17T19:15:17.1932360Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1932804Z 
2020-03-17T19:15:17.1933166Z ------------------------------------------
2020-03-17T19:15:17.1933326Z 
2020-03-17T19:15:17.1933422Z 
2020-03-17T19:15:17.1933422Z 
2020-03-17T19:15:17.1933785Z ---- [ui] ui/consts/partial_qualif.rs stdout ----
2020-03-17T19:15:17.1933957Z 
2020-03-17T19:15:17.1934198Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1934652Z status: exit code: 101
2020-03-17T19:15:17.1936662Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/partial_qualif.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/partial_qualif" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/partial_qualif/auxiliary"
2020-03-17T19:15:17.1938120Z ------------------------------------------
2020-03-17T19:15:17.1938288Z 
2020-03-17T19:15:17.1938616Z ------------------------------------------
2020-03-17T19:15:17.1938820Z stderr:
---
2020-03-17T19:15:17.1940617Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1940812Z 
2020-03-17T19:15:17.1941010Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1941191Z 
2020-03-17T19:15:17.1941761Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1942464Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1942697Z 
2020-03-17T19:15:17.1942697Z 
2020-03-17T19:15:17.1943294Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1943713Z 
2020-03-17T19:15:17.1944038Z ------------------------------------------
2020-03-17T19:15:17.1944212Z 
2020-03-17T19:15:17.1944300Z 
2020-03-17T19:15:17.1944300Z 
2020-03-17T19:15:17.1944675Z ---- [ui] ui/consts/projection_qualif.rs#mut_refs stdout ----
2020-03-17T19:15:17.1945016Z 
2020-03-17T19:15:17.1945311Z error in revision `mut_refs`: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1945628Z status: exit code: 101
2020-03-17T19:15:17.1947436Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/projection_qualif.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mut_refs" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/projection_qualif.mut_refs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/projection_qualif.mut_refs/auxiliary"
2020-03-17T19:15:17.1949063Z ------------------------------------------
2020-03-17T19:15:17.1949221Z 
2020-03-17T19:15:17.1949608Z ------------------------------------------
2020-03-17T19:15:17.1949795Z stderr:
---
2020-03-17T19:15:17.1951751Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1951935Z 
2020-03-17T19:15:17.1952134Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1952333Z 
2020-03-17T19:15:17.1953587Z note: we would appreciate a bug report: ***/blobthread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-17T19:15:17.1954572Z /master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1954728Z 
2020-03-17T19:15:17.1955213Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1955456Z 
2020-03-17T19:15:17.1955456Z 
2020-03-17T19:15:17.1956339Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1956759Z 
2020-03-17T19:15:17.1957077Z ------------------------------------------
2020-03-17T19:15:17.1957231Z 
2020-03-17T19:15:17.1957317Z 
2020-03-17T19:15:17.1957317Z 
2020-03-17T19:15:17.1957696Z ---- [ui] ui/consts/projection_qualif.rs#stock stdout ----
2020-03-17T19:15:17.1957876Z 
2020-03-17T19:15:17.1958144Z error in revision `stock`: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1958454Z status: exit code: 101
2020-03-17T19:15:17.1960268Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/projection_qualif.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/projection_qualif.stock" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/projection_qualif.stock/auxiliary"
2020-03-17T19:15:17.1961740Z ------------------------------------------
2020-03-17T19:15:17.1961903Z 
2020-03-17T19:15:17.1962221Z ------------------------------------------
2020-03-17T19:15:17.1962419Z stderr:
---
2020-03-17T19:15:17.1964151Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1964341Z 
2020-03-17T19:15:17.1964534Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1964708Z 
2020-03-17T19:15:17.1965254Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1965934Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1966259Z 
2020-03-17T19:15:17.1966259Z 
2020-03-17T19:15:17.1966859Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1967265Z 
2020-03-17T19:15:17.1967598Z ------------------------------------------
2020-03-17T19:15:17.1967752Z 
2020-03-17T19:15:17.1967837Z 
2020-03-17T19:15:17.1967837Z 
2020-03-17T19:15:17.1968189Z ---- [ui] ui/consts/promote_borrowed_field.rs stdout ----
2020-03-17T19:15:17.1968367Z 
2020-03-17T19:15:17.1968912Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.1969147Z status: exit code: 101
2020-03-17T19:15:17.1970780Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promote_borrowed_field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote_borrowed_field/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote_borrowed_field/auxiliary"
2020-03-17T19:15:17.1972264Z ------------------------------------------
2020-03-17T19:15:17.1972435Z 
2020-03-17T19:15:17.1972744Z ------------------------------------------
2020-03-17T19:15:17.1972921Z stderr:
---
2020-03-17T19:15:17.1974658Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1974824Z 
2020-03-17T19:15:17.1975009Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1975201Z 
2020-03-17T19:15:17.1975728Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1976404Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1976621Z 
2020-03-17T19:15:17.1976621Z 
2020-03-17T19:15:17.1977158Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1977571Z 
2020-03-17T19:15:17.1977880Z ------------------------------------------
2020-03-17T19:15:17.1978029Z 
2020-03-17T19:15:17.1978112Z 
2020-03-17T19:15:17.1978112Z 
2020-03-17T19:15:17.1978455Z ---- [ui] ui/consts/qualif_overwrite.rs stdout ----
2020-03-17T19:15:17.1978619Z 
2020-03-17T19:15:17.1978843Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1979124Z status: exit code: 101
2020-03-17T19:15:17.1980790Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/qualif_overwrite.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/qualif_overwrite" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/qualif_overwrite/auxiliary"
2020-03-17T19:15:17.1982174Z ------------------------------------------
2020-03-17T19:15:17.1982326Z 
2020-03-17T19:15:17.1982819Z ------------------------------------------
2020-03-17T19:15:17.1983001Z stderr:
---
2020-03-17T19:15:17.1984840Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1985013Z 
2020-03-17T19:15:17.1985205Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1985381Z 
2020-03-17T19:15:17.1986109Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1986986Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1987205Z 
2020-03-17T19:15:17.1987205Z 
2020-03-17T19:15:17.1987763Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1988144Z 
2020-03-17T19:15:17.1988456Z ------------------------------------------
2020-03-17T19:15:17.1988601Z 
2020-03-17T19:15:17.1988681Z 
2020-03-17T19:15:17.1988681Z 
2020-03-17T19:15:17.1989006Z ---- [ui] ui/consts/qualif_overwrite_2.rs stdout ----
2020-03-17T19:15:17.1989167Z 
2020-03-17T19:15:17.1989399Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.1989653Z status: exit code: 101
2020-03-17T19:15:17.1991298Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/qualif_overwrite_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/qualif_overwrite_2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/qualif_overwrite_2/auxiliary"
2020-03-17T19:15:17.1993034Z ------------------------------------------
2020-03-17T19:15:17.1993195Z 
2020-03-17T19:15:17.1993512Z ------------------------------------------
2020-03-17T19:15:17.1993694Z stderr:
---
2020-03-17T19:15:17.1995726Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.1995888Z 
2020-03-17T19:15:17.1996084Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.1996248Z 
2020-03-17T19:15:17.1996749Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.1997814Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.1998046Z 
2020-03-17T19:15:17.1998046Z 
2020-03-17T19:15:17.1998808Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.1999264Z 
2020-03-17T19:15:17.1999604Z ------------------------------------------
2020-03-17T19:15:17.1999770Z 
2020-03-17T19:15:17.2000051Z 
2020-03-17T19:15:17.2000051Z 
2020-03-17T19:15:17.2001039Z ---- [ui] ui/enum-discriminant/niche.rs stdout ----
2020-03-17T19:15:17.2001213Z 
2020-03-17T19:15:17.2001964Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.2002213Z status: exit code: 101
2020-03-17T19:15:17.2003841Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/enum-discriminant/niche.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-discriminant/niche/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/enum-discriminant/niche/auxiliary"
2020-03-17T19:15:17.2005676Z ------------------------------------------
2020-03-17T19:15:17.2005824Z 
2020-03-17T19:15:17.2006140Z ------------------------------------------
2020-03-17T19:15:17.2006310Z stderr:
---
2020-03-17T19:15:17.2008012Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.2008174Z 
2020-03-17T19:15:17.2008352Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.2008516Z 
2020-03-17T19:15:17.2009047Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.2009691Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.2009919Z 
2020-03-17T19:15:17.2009919Z 
2020-03-17T19:15:17.2010439Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.2010819Z 
2020-03-17T19:15:17.2011138Z ------------------------------------------
2020-03-17T19:15:17.2011282Z 
2020-03-17T19:15:17.2011362Z 
2020-03-17T19:15:17.2011362Z 
2020-03-17T19:15:17.2011666Z ---- [ui] ui/error-codes/E0283.rs stdout ----
2020-03-17T19:15:17.2011830Z 
2020-03-17T19:15:17.2012147Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.2012402Z status: exit code: 101
2020-03-17T19:15:17.2014224Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0283.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0283" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0283/auxiliary"
2020-03-17T19:15:17.2015588Z ------------------------------------------
2020-03-17T19:15:17.2015741Z 
2020-03-17T19:15:17.2016049Z ------------------------------------------
2020-03-17T19:15:17.2016392Z stderr:
---
2020-03-17T19:15:17.2018916Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.2019114Z 
2020-03-17T19:15:17.2019356Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.2019558Z 
2020-03-17T19:15:17.2020188Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.2021473Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.2021841Z 
2020-03-17T19:15:17.2021841Z 
2020-03-17T19:15:17.2022731Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.2023153Z 
2020-03-17T19:15:17.2023472Z ------------------------------------------
2020-03-17T19:15:17.2023625Z 
2020-03-17T19:15:17.2023725Z 
2020-03-17T19:15:17.2023725Z 
2020-03-17T19:15:17.2024206Z ---- [ui] ui/issues/issue-18118.rs stdout ----
2020-03-17T19:15:17.2024834Z 
2020-03-17T19:15:17.2025075Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.2025373Z status: exit code: 101
2020-03-17T19:15:17.2027924Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18118.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118/auxiliary"
2020-03-17T19:15:17.2029675Z ------------------------------------------
2020-03-17T19:15:17.2029843Z 
2020-03-17T19:15:17.2030353Z ------------------------------------------
2020-03-17T19:15:17.2030546Z stderr:
---
2020-03-17T19:15:17.2032611Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.2032779Z 
2020-03-17T19:15:17.2032969Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.2033139Z 
2020-03-17T19:15:17.2033865Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.2034703Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.2035188Z 
2020-03-17T19:15:17.2035188Z 
2020-03-17T19:15:17.2035933Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.2036523Z 
2020-03-17T19:15:17.2036847Z ------------------------------------------
2020-03-17T19:15:17.2037001Z 
2020-03-17T19:15:17.2037086Z 
2020-03-17T19:15:17.2037086Z 
2020-03-17T19:15:17.2037412Z ---- [ui] ui/issues/issue-54954.rs stdout ----
2020-03-17T19:15:17.2037590Z 
2020-03-17T19:15:17.2037982Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.2038249Z status: exit code: 101
2020-03-17T19:15:17.2040243Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-54954.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54954" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54954/auxiliary"
2020-03-17T19:15:17.2041605Z ------------------------------------------
2020-03-17T19:15:17.2041757Z 
2020-03-17T19:15:17.2042064Z ------------------------------------------
2020-03-17T19:15:17.2042254Z stderr:
2020-03-17T19:15:17.2042254Z stderr:
2020-03-17T19:15:17.2042568Z ------------------------------------------
2020-03-17T19:15:17.2042826Z error[E0379]: functions in traits cannot be declared const
2020-03-17T19:15:17.2043288Z   --> /checkout/src/test/ui/issues/issue-54954.rs:7:5
2020-03-17T19:15:17.2043487Z    |
2020-03-17T19:15:17.2043835Z LL |     const fn const_val<T: Sized>() -> usize {
2020-03-17T19:15:17.2044117Z    |     ^^^^^ functions in traits cannot be const
2020-03-17T19:15:17.2044756Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc/hir/map/mod.rs:271:20
2020-03-17T19:15:17.2045184Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-17T19:15:17.2045663Z 
2020-03-17T19:15:17.2045853Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.2045853Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.2046203Z 
2020-03-17T19:15:17.2046417Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.2046598Z 
2020-03-17T19:15:17.2047168Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.2048109Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.2048341Z 
2020-03-17T19:15:17.2048341Z 
2020-03-17T19:15:17.2048915Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.2049562Z error: aborting due to previous error
2020-03-17T19:15:17.2049702Z 
2020-03-17T19:15:17.2050088Z For more information about this error, try `rustc --explain E0379`.
2020-03-17T19:15:17.2050276Z 
2020-03-17T19:15:17.2050276Z 
2020-03-17T19:15:17.2050582Z ------------------------------------------
2020-03-17T19:15:17.2050736Z 
2020-03-17T19:15:17.2050821Z 
2020-03-17T19:15:17.2051153Z ---- [ui] ui/issues/issue-58022.rs stdout ----
2020-03-17T19:15:17.2051311Z 
2020-03-17T19:15:17.2051534Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.2051812Z status: exit code: 101
2020-03-17T19:15:17.2053899Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-58022.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-58022" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-58022/auxiliary"
2020-03-17T19:15:17.2055655Z ------------------------------------------
2020-03-17T19:15:17.2055812Z 
2020-03-17T19:15:17.2056147Z ------------------------------------------
2020-03-17T19:15:17.2056327Z stderr:
2020-03-17T19:15:17.2056327Z stderr:
2020-03-17T19:15:17.2056651Z ------------------------------------------
2020-03-17T19:15:17.2056969Z error[E0423]: expected function, tuple struct or tuple variant, found trait `Foo`
2020-03-17T19:15:17.2057464Z   --> /checkout/src/test/ui/issues/issue-58022.rs:14:9
2020-03-17T19:15:17.2057676Z    |
2020-03-17T19:15:17.2057865Z LL |         Foo(Box::new(*slice))
2020-03-17T19:15:17.2058311Z 
2020-03-17T19:15:17.2058812Z thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc/hir/map/mod.rs:271:20
2020-03-17T19:15:17.2059250Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-17T19:15:17.2059459Z 
2020-03-17T19:15:17.2059459Z 
2020-03-17T19:15:17.2059647Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.2059840Z 
2020-03-17T19:15:17.2060032Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.2060207Z 
2020-03-17T19:15:17.2060935Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.2061785Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.2062010Z 
2020-03-17T19:15:17.2062010Z 
2020-03-17T19:15:17.2063122Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.2063824Z error: aborting due to previous error
2020-03-17T19:15:17.2063995Z 
2020-03-17T19:15:17.2064451Z For more information about this error, try `rustc --explain E0423`.
2020-03-17T19:15:17.2064674Z 
2020-03-17T19:15:17.2064674Z 
2020-03-17T19:15:17.2065033Z ------------------------------------------
2020-03-17T19:15:17.2065387Z 
2020-03-17T19:15:17.2065594Z 
2020-03-17T19:15:17.2066027Z ---- [ui] ui/mir-dataflow/indirect-mutation-offset.rs stdout ----
2020-03-17T19:15:17.2066398Z 
2020-03-17T19:15:17.2066841Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.2067397Z status: exit code: 101
2020-03-17T19:15:17.2070297Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir-dataflow/indirect-mutation-offset.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-dataflow/indirect-mutation-offset/auxiliary"
2020-03-17T19:15:17.2072242Z ------------------------------------------
2020-03-17T19:15:17.2072414Z 
2020-03-17T19:15:17.2072733Z ------------------------------------------
2020-03-17T19:15:17.2072914Z stderr:
---
2020-03-17T19:15:17.2074662Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.2074834Z 
2020-03-17T19:15:17.2075025Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.2075217Z 
2020-03-17T19:15:17.2075752Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.2076450Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.2076680Z 
2020-03-17T19:15:17.2076680Z 
2020-03-17T19:15:17.2077316Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.2077913Z 
2020-03-17T19:15:17.2078246Z ------------------------------------------
2020-03-17T19:15:17.2078406Z 
2020-03-17T19:15:17.2078493Z 
2020-03-17T19:15:17.2078493Z 
2020-03-17T19:15:17.2078873Z ---- [ui] ui/raw-ref-op/unusual_locations.rs stdout ----
2020-03-17T19:15:17.2079056Z 
2020-03-17T19:15:17.2079411Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.2079668Z status: exit code: 101
2020-03-17T19:15:17.2081478Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/raw-ref-op/unusual_locations.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-ref-op/unusual_locations" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw-ref-op/unusual_locations/auxiliary"
2020-03-17T19:15:17.2082977Z ------------------------------------------
2020-03-17T19:15:17.2083140Z 
2020-03-17T19:15:17.2083491Z ------------------------------------------
2020-03-17T19:15:17.2083679Z stderr:
---
2020-03-17T19:15:17.2085468Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.2085750Z 
2020-03-17T19:15:17.2086105Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.2086279Z 
2020-03-17T19:15:17.2086843Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.2087524Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.2087750Z 
2020-03-17T19:15:17.2087750Z 
2020-03-17T19:15:17.2088390Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.2088803Z 
2020-03-17T19:15:17.2089154Z ------------------------------------------
2020-03-17T19:15:17.2089310Z 
2020-03-17T19:15:17.2089395Z 
2020-03-17T19:15:17.2089395Z 
2020-03-17T19:15:17.2089739Z ---- [ui] ui/save-analysis/issue-68621.rs stdout ----
2020-03-17T19:15:17.2090135Z 
2020-03-17T19:15:17.2090391Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.2090678Z status: exit code: 101
2020-03-17T19:15:17.2093023Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/save-analysis/issue-68621.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/issue-68621" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/issue-68621/auxiliary"
2020-03-17T19:15:17.2094680Z ------------------------------------------
2020-03-17T19:15:17.2094854Z 
2020-03-17T19:15:17.2095208Z ------------------------------------------
2020-03-17T19:15:17.2095408Z stderr:
---
2020-03-17T19:15:17.2099475Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.2099659Z 
2020-03-17T19:15:17.2099863Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.2100070Z 
2020-03-17T19:15:17.2100994Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.2101800Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.2102058Z 
2020-03-17T19:15:17.2102058Z 
2020-03-17T19:15:17.2102742Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.2103336Z error: aborting due to previous error
2020-03-17T19:15:17.2103511Z 
2020-03-17T19:15:17.2103609Z 
2020-03-17T19:15:17.2103973Z ------------------------------------------
2020-03-17T19:15:17.2103973Z ------------------------------------------
2020-03-17T19:15:17.2104525Z 
2020-03-17T19:15:17.2104778Z 
2020-03-17T19:15:17.2105154Z ---- [ui] ui/type-alias-impl-trait/issue-63279.rs stdout ----
2020-03-17T19:15:17.2105344Z 
2020-03-17T19:15:17.2105598Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-17T19:15:17.2105877Z status: exit code: 101
2020-03-17T19:15:17.2108550Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63279" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63279/auxiliary"
2020-03-17T19:15:17.2110392Z ------------------------------------------
2020-03-17T19:15:17.2110574Z 
2020-03-17T19:15:17.2110941Z ------------------------------------------
2020-03-17T19:15:17.2111151Z stderr:
2020-03-17T19:15:17.2111151Z stderr:
2020-03-17T19:15:17.2111539Z ------------------------------------------
2020-03-17T19:15:17.2113434Z error[E0271]: type mismatch resolving `<[closure@/checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs:8:5: 8:28] as std::ops::FnOnce<()>>::Output == ()`
2020-03-17T19:15:17.2114683Z    |
2020-03-17T19:15:17.2114960Z LL | type Closure = impl FnOnce(); //~ ERROR: type mismatch resolving
2020-03-17T19:15:17.2115343Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found `()`
2020-03-17T19:15:17.2115604Z    |
---
2020-03-17T19:15:17.2118538Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.2118722Z 
2020-03-17T19:15:17.2118944Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.2119132Z 
2020-03-17T19:15:17.2119706Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.2120453Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.2120699Z 
2020-03-17T19:15:17.2120699Z 
2020-03-17T19:15:17.2121336Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.2121892Z error: aborting due to previous error
2020-03-17T19:15:17.2122048Z 
2020-03-17T19:15:17.2122466Z For more information about this error, try `rustc --explain E0271`.
2020-03-17T19:15:17.2122673Z 
2020-03-17T19:15:17.2122673Z 
2020-03-17T19:15:17.2123008Z ------------------------------------------
2020-03-17T19:15:17.2123179Z 
2020-03-17T19:15:17.2123270Z 
2020-03-17T19:15:17.2123749Z ---- [ui] ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice.rs stdout ----
2020-03-17T19:15:17.2123996Z 
2020-03-17T19:15:17.2125155Z error: test compilation failed although it shouldn't!
2020-03-17T19:15:17.2125945Z status: exit code: 101
2020-03-17T19:15:17.2129072Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice/auxiliary"
2020-03-17T19:15:17.2130933Z ------------------------------------------
2020-03-17T19:15:17.2131099Z 
2020-03-17T19:15:17.2131448Z ------------------------------------------
2020-03-17T19:15:17.2131637Z stderr:
---
2020-03-17T19:15:17.2134139Z error: internal compiler error: unexpected panic
2020-03-17T19:15:17.2134329Z 
2020-03-17T19:15:17.2134529Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-17T19:15:17.2134710Z 
2020-03-17T19:15:17.2135853Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-17T19:15:17.2136971Z note: rustc 1.44.0-nightly (c92c4cf24 2020-03-17) running on x86_64-unknown-linux-gnu
2020-03-17T19:15:17.2137247Z 
2020-03-17T19:15:17.2137247Z 
2020-03-17T19:15:17.2137933Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-17T19:15:17.2138430Z 
2020-03-17T19:15:17.2138820Z ------------------------------------------
2020-03-17T19:15:17.2139000Z 
2020-03-17T19:15:17.2139098Z 
---
2020-03-17T19:15:17.2165193Z test result: FAILED. 9684 passed; 54 failed; 57 ignored; 0 measured; 0 filtered out
2020-03-17T19:15:17.2165452Z 
2020-03-17T19:15:17.2165544Z 
2020-03-17T19:15:17.2165634Z 
2020-03-17T19:15:17.2169318Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-17T19:15:17.2172601Z 
2020-03-17T19:15:17.2172858Z 
2020-03-17T19:15:17.2173094Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-17T19:15:17.2174210Z Build completed unsuccessfully in 1:01:56
2020-03-17T19:15:17.2174210Z Build completed unsuccessfully in 1:01:56
2020-03-17T19:15:17.2174478Z == clock drift check ==
2020-03-17T19:15:17.2174751Z   local time: Tue Mar 17 19:15:17 UTC 2020
2020-03-17T19:15:17.4429822Z   network time: Tue, 17 Mar 2020 19:15:17 GMT
2020-03-17T19:15:17.4435447Z == end clock drift check ==
2020-03-17T19:15:17.9377305Z 
2020-03-17T19:15:17.9465510Z ##[error]Bash exited with code '1'.
2020-03-17T19:15:17.9483416Z ##[section]Finishing: Run build
2020-03-17T19:15:17.9536738Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-17T19:15:17.9542560Z Task         : Get sources
2020-03-17T19:15:17.9542911Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T19:15:17.9543236Z Version      : 1.0.0
2020-03-17T19:15:17.9543478Z Author       : Microsoft
2020-03-17T19:15:17.9543478Z Author       : Microsoft
2020-03-17T19:15:17.9543833Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T19:15:17.9544451Z ==============================================================================
2020-03-17T19:15:18.2797526Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T19:15:18.2846518Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-17T19:15:18.2938847Z Cleaning up task key
2020-03-17T19:15:18.2940156Z Start cleaning up orphan processes.
2020-03-17T19:15:18.3155261Z Terminate orphan process: pid (4192) (python)
2020-03-17T19:15:18.3327091Z ##[section]Finishing: Finalize Job
