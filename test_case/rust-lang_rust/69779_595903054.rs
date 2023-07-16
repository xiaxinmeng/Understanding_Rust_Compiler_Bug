plain
2020-03-06T17:24:47.1046040Z ========================== Starting Command Output ===========================
2020-03-06T17:24:47.1050856Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/90094399-bd69-42d1-abe7-7a06015c4eb1.sh
2020-03-06T17:24:47.1051565Z 
2020-03-06T17:24:47.1056372Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-06T17:24:47.1075938Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69779/merge to s
2020-03-06T17:24:47.1079481Z Task         : Get sources
2020-03-06T17:24:47.1079775Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-06T17:24:47.1080081Z Version      : 1.0.0
2020-03-06T17:24:47.1080273Z Author       : Microsoft
---
2020-03-06T17:24:48.1087020Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-06T17:24:48.1092488Z ##[command]git config gc.auto 0
2020-03-06T17:24:48.1095846Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-06T17:24:48.1098694Z ##[command]git config --get-all http.proxy
2020-03-06T17:24:48.1104949Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69779/merge:refs/remotes/pull/69779/merge
---
2020-03-06T18:25:42.0203570Z .................................................................................................... 1700/9732
2020-03-06T18:25:46.6534595Z .................................................................................................... 1800/9732
2020-03-06T18:25:57.8342476Z ..........................................................i......................................... 1900/9732
2020-03-06T18:26:05.7765057Z .................................................................................................... 2000/9732
2020-03-06T18:26:20.1003685Z ................................................iiiii............................................... 2100/9732
2020-03-06T18:26:30.0612820Z .................................................................................................... 2300/9732
2020-03-06T18:26:32.3065244Z .................................................................................................... 2400/9732
2020-03-06T18:26:35.8196705Z .................................................................................................... 2500/9732
2020-03-06T18:26:58.7395673Z .................................................................................................... 2600/9732
---
2020-03-06T18:29:40.2156653Z .........i...............i.......................................................................... 5000/9732
2020-03-06T18:29:49.8781344Z .................................................................................................... 5100/9732
2020-03-06T18:29:55.0130560Z ....................................................i............................................... 5200/9732
2020-03-06T18:30:03.3937597Z .................................................................................................... 5300/9732
2020-03-06T18:30:10.7393276Z ................................ii.ii........i...i.................................................. 5400/9732
2020-03-06T18:30:18.7680221Z .................................................................................................... 5600/9732
2020-03-06T18:30:28.5075920Z .................................................................................................... 5700/9732
2020-03-06T18:30:34.9698552Z .......................i............................................................................ 5800/9732
2020-03-06T18:30:40.5712298Z .................................................................................................... 5900/9732
2020-03-06T18:30:40.5712298Z .................................................................................................... 5900/9732
2020-03-06T18:30:51.7663364Z .................................................................................................... 6000/9732
2020-03-06T18:31:02.0979602Z ...............ii...i..ii...........i............................................................... 6100/9732
2020-03-06T18:31:18.3346131Z .................................................................................................... 6300/9732
2020-03-06T18:31:25.3072591Z .................................................................................................... 6400/9732
2020-03-06T18:31:25.3072591Z .................................................................................................... 6400/9732
2020-03-06T18:31:41.8720829Z ..............................................i..ii................................................. 6500/9732
2020-03-06T18:32:05.8098877Z .................................................................................................... 6700/9732
2020-03-06T18:32:07.8361909Z ......................................i............................................................. 6800/9732
2020-03-06T18:32:10.1413004Z .................................................................................................... 6900/9732
2020-03-06T18:32:12.3200700Z ....................................................................i............................... 7000/9732
---
2020-03-06T18:33:53.9134557Z .................................................................................................... 7700/9732
2020-03-06T18:33:59.1482958Z .................................................................................................... 7800/9732
2020-03-06T18:34:04.0997333Z .................................................................................................... 7900/9732
2020-03-06T18:34:12.9489460Z ...............i.................................................................................... 8000/9732
2020-03-06T18:34:21.7502703Z ...............................................................iiiiiiiii.i.......................... 8100/9732
2020-03-06T18:34:35.8408884Z ......i......i...................................................................................... 8300/9732
2020-03-06T18:34:41.2371393Z .................................................................................................... 8400/9732
2020-03-06T18:34:54.3492613Z ..............................F..................................................................... 8500/9732
2020-03-06T18:35:03.7320106Z .................................................................................................... 8600/9732
---
2020-03-06T18:37:07.5725359Z ---- [ui] ui/async-await/issue-60709.rs stdout ----
2020-03-06T18:37:07.5725971Z 
2020-03-06T18:37:07.5727157Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5727725Z status: exit code: 101
2020-03-06T18:37:07.5729969Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-60709.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-60709/a" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Copt-level=z" "-Cdebuginfo=2" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-60709/auxiliary"
2020-03-06T18:37:07.5731989Z ------------------------------------------
2020-03-06T18:37:07.5732328Z 
2020-03-06T18:37:07.5732878Z ------------------------------------------
2020-03-06T18:37:07.5733270Z stderr:
2020-03-06T18:37:07.5733270Z stderr:
2020-03-06T18:37:07.5734010Z ------------------------------------------
2020-03-06T18:37:07.5734991Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5736007Z 
2020-03-06T18:37:07.5736373Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5736698Z 
2020-03-06T18:37:07.5737044Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5737044Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5737549Z 
2020-03-06T18:37:07.5738501Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5739646Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5740064Z 
2020-03-06T18:37:07.5740064Z 
2020-03-06T18:37:07.5740918Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C opt-level=z -C debuginfo=2
2020-03-06T18:37:07.5741749Z 
2020-03-06T18:37:07.5742315Z ------------------------------------------
2020-03-06T18:37:07.5742656Z 
2020-03-06T18:37:07.5742884Z 
2020-03-06T18:37:07.5742884Z 
2020-03-06T18:37:07.5743429Z ---- [ui] ui/backtrace-debuginfo.rs stdout ----
2020-03-06T18:37:07.5743980Z 
2020-03-06T18:37:07.5746215Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5748338Z status: exit code: 101
2020-03-06T18:37:07.5751417Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/backtrace-debuginfo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/backtrace-debuginfo/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-Cllvm-args=-enable-tail-merge=0" "-Cllvm-args=-opt-bisect-limit=0" "-Cforce-frame-pointers=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/backtrace-debuginfo/auxiliary"
2020-03-06T18:37:07.5753746Z ------------------------------------------
2020-03-06T18:37:07.5754091Z 
2020-03-06T18:37:07.5754987Z ------------------------------------------
2020-03-06T18:37:07.5756460Z stderr:
2020-03-06T18:37:07.5756460Z stderr:
2020-03-06T18:37:07.5757155Z ------------------------------------------
2020-03-06T18:37:07.5760668Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5762011Z 
2020-03-06T18:37:07.5762368Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5764107Z 
2020-03-06T18:37:07.5764485Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5764485Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5764836Z 
2020-03-06T18:37:07.5765737Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5766931Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5767360Z 
2020-03-06T18:37:07.5767360Z 
2020-03-06T18:37:07.5768423Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C llvm-args=-enable-tail-merge=0 -C llvm-args=-opt-bisect-limit=0 -C force-frame-pointers=yes
2020-03-06T18:37:07.5769338Z 
2020-03-06T18:37:07.5769899Z ------------------------------------------
2020-03-06T18:37:07.5770257Z 
2020-03-06T18:37:07.5770509Z 
2020-03-06T18:37:07.5770509Z 
2020-03-06T18:37:07.5771040Z ---- [ui] ui/backtrace.rs stdout ----
2020-03-06T18:37:07.5773160Z 
2020-03-06T18:37:07.5774039Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5774507Z status: exit code: 101
2020-03-06T18:37:07.5776620Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/backtrace.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/backtrace/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/backtrace/auxiliary"
2020-03-06T18:37:07.5778526Z ------------------------------------------
2020-03-06T18:37:07.5779012Z 
2020-03-06T18:37:07.5779570Z ------------------------------------------
2020-03-06T18:37:07.5782117Z stderr:
2020-03-06T18:37:07.5782117Z stderr:
2020-03-06T18:37:07.5782600Z ------------------------------------------
2020-03-06T18:37:07.5783241Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5783940Z 
2020-03-06T18:37:07.5784173Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5784371Z 
2020-03-06T18:37:07.5784601Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5784601Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5784803Z 
2020-03-06T18:37:07.5785439Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5786205Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5786476Z 
2020-03-06T18:37:07.5786476Z 
2020-03-06T18:37:07.5787072Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath
2020-03-06T18:37:07.5787505Z 
2020-03-06T18:37:07.5787880Z ------------------------------------------
2020-03-06T18:37:07.5788055Z 
2020-03-06T18:37:07.5788153Z 
2020-03-06T18:37:07.5788153Z 
2020-03-06T18:37:07.5788537Z ---- [ui] ui/binding/match-arm-statics.rs stdout ----
2020-03-06T18:37:07.5788746Z 
2020-03-06T18:37:07.5789133Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5789396Z status: exit code: 101
2020-03-06T18:37:07.5791248Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binding/match-arm-statics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/match-arm-statics/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/match-arm-statics/auxiliary"
2020-03-06T18:37:07.5792790Z ------------------------------------------
2020-03-06T18:37:07.5792967Z 
2020-03-06T18:37:07.5793324Z ------------------------------------------
2020-03-06T18:37:07.5793527Z stderr:
2020-03-06T18:37:07.5793527Z stderr:
2020-03-06T18:37:07.5793907Z ------------------------------------------
2020-03-06T18:37:07.5797203Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5797957Z 
2020-03-06T18:37:07.5798175Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5798371Z 
2020-03-06T18:37:07.5798605Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5798605Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5798807Z 
2020-03-06T18:37:07.5799510Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5800307Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5800562Z 
2020-03-06T18:37:07.5800562Z 
2020-03-06T18:37:07.5801148Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath
2020-03-06T18:37:07.5801597Z 
2020-03-06T18:37:07.5801955Z ------------------------------------------
2020-03-06T18:37:07.5802128Z 
2020-03-06T18:37:07.5802227Z 
2020-03-06T18:37:07.5802227Z 
2020-03-06T18:37:07.5802758Z ---- [ui] ui/generator/issue-58888.rs stdout ----
2020-03-06T18:37:07.5802958Z 
2020-03-06T18:37:07.5803387Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5803669Z status: exit code: 101
2020-03-06T18:37:07.5805474Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/issue-58888.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-58888/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-58888/auxiliary"
2020-03-06T18:37:07.5807108Z ------------------------------------------
2020-03-06T18:37:07.5807284Z 
2020-03-06T18:37:07.5807658Z ------------------------------------------
2020-03-06T18:37:07.5807869Z stderr:
2020-03-06T18:37:07.5807869Z stderr:
2020-03-06T18:37:07.5808237Z ------------------------------------------
2020-03-06T18:37:07.5808865Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5809562Z 
2020-03-06T18:37:07.5809793Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5809995Z 
2020-03-06T18:37:07.5810212Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5810212Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5810411Z 
2020-03-06T18:37:07.5811033Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5811801Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5812055Z 
2020-03-06T18:37:07.5812055Z 
2020-03-06T18:37:07.5812667Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath
2020-03-06T18:37:07.5813099Z 
2020-03-06T18:37:07.5813473Z ------------------------------------------
2020-03-06T18:37:07.5813647Z 
2020-03-06T18:37:07.5813745Z 
2020-03-06T18:37:07.5813745Z 
2020-03-06T18:37:07.5814175Z ---- [ui] ui/issues/issue-24687-embed-debuginfo/main.rs stdout ----
2020-03-06T18:37:07.5814394Z 
2020-03-06T18:37:07.5814798Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5815061Z status: exit code: 101
2020-03-06T18:37:07.5817027Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24687-embed-debuginfo/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24687-embed-debuginfo/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24687-embed-debuginfo/main/auxiliary"
2020-03-06T18:37:07.5818629Z ------------------------------------------
2020-03-06T18:37:07.5818806Z 
2020-03-06T18:37:07.5819163Z ------------------------------------------
2020-03-06T18:37:07.5819367Z stderr:
2020-03-06T18:37:07.5819367Z stderr:
2020-03-06T18:37:07.5819748Z ------------------------------------------
2020-03-06T18:37:07.5820361Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5821082Z 
2020-03-06T18:37:07.5821297Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5821496Z 
2020-03-06T18:37:07.5821729Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5821729Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5821929Z 
2020-03-06T18:37:07.5822524Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5823428Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5823682Z 
2020-03-06T18:37:07.5823682Z 
2020-03-06T18:37:07.5824271Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath
2020-03-06T18:37:07.5824720Z 
2020-03-06T18:37:07.5832022Z ------------------------------------------
2020-03-06T18:37:07.5832433Z 
2020-03-06T18:37:07.5832568Z 
2020-03-06T18:37:07.5832568Z 
2020-03-06T18:37:07.5833249Z ---- [ui] ui/issues/issue-24945-repeat-dash-opts.rs stdout ----
2020-03-06T18:37:07.5833468Z 
2020-03-06T18:37:07.5833874Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5834161Z status: exit code: 101
2020-03-06T18:37:07.5836685Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24945-repeat-dash-opts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24945-repeat-dash-opts/a" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-g" "-O" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24945-repeat-dash-opts/auxiliary"
2020-03-06T18:37:07.5838383Z ------------------------------------------
2020-03-06T18:37:07.5838564Z 
2020-03-06T18:37:07.5838952Z ------------------------------------------
2020-03-06T18:37:07.5839156Z stderr:
2020-03-06T18:37:07.5839156Z stderr:
2020-03-06T18:37:07.5839522Z ------------------------------------------
2020-03-06T18:37:07.5840296Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5841007Z 
2020-03-06T18:37:07.5841248Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5841449Z 
2020-03-06T18:37:07.5841668Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5841668Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5841868Z 
2020-03-06T18:37:07.5842550Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5843321Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5843599Z 
2020-03-06T18:37:07.5843599Z 
2020-03-06T18:37:07.5844190Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath
2020-03-06T18:37:07.5844621Z 
2020-03-06T18:37:07.5844999Z ------------------------------------------
2020-03-06T18:37:07.5845172Z 
2020-03-06T18:37:07.5845271Z 
2020-03-06T18:37:07.5845271Z 
2020-03-06T18:37:07.5845639Z ---- [ui] ui/issues/issue-26484.rs stdout ----
2020-03-06T18:37:07.5845837Z 
2020-03-06T18:37:07.5846227Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5846496Z status: exit code: 101
2020-03-06T18:37:07.5848297Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-26484.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26484/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26484/auxiliary"
2020-03-06T18:37:07.5849797Z ------------------------------------------
2020-03-06T18:37:07.5849974Z 
2020-03-06T18:37:07.5850333Z ------------------------------------------
2020-03-06T18:37:07.5850537Z stderr:
2020-03-06T18:37:07.5850537Z stderr:
2020-03-06T18:37:07.5850916Z ------------------------------------------
2020-03-06T18:37:07.5851673Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5852408Z 
2020-03-06T18:37:07.5852623Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5852821Z 
2020-03-06T18:37:07.5853056Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5853056Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5853257Z 
2020-03-06T18:37:07.5853903Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5854819Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5855075Z 
2020-03-06T18:37:07.5855075Z 
2020-03-06T18:37:07.5855666Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath
2020-03-06T18:37:07.5856114Z 
2020-03-06T18:37:07.5856754Z ------------------------------------------
2020-03-06T18:37:07.5856935Z 
2020-03-06T18:37:07.5857055Z 
2020-03-06T18:37:07.5857055Z 
2020-03-06T18:37:07.5857460Z ---- [ui] ui/issues/issue-31702.rs stdout ----
2020-03-06T18:37:07.5857643Z 
2020-03-06T18:37:07.5858189Z error: auxiliary build of "/checkout/src/test/ui/issues/auxiliary/issue-31702-2.rs" failed to compile: 
2020-03-06T18:37:07.5858576Z status: exit code: 101
2020-03-06T18:37:07.5860493Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/auxiliary/issue-31702-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31702/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31702/auxiliary"
2020-03-06T18:37:07.5862087Z ------------------------------------------
2020-03-06T18:37:07.5862268Z 
2020-03-06T18:37:07.5862643Z ------------------------------------------
2020-03-06T18:37:07.5862847Z stderr:
2020-03-06T18:37:07.5862847Z stderr:
2020-03-06T18:37:07.5863212Z ------------------------------------------
2020-03-06T18:37:07.5863838Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5864541Z 
2020-03-06T18:37:07.5864772Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5864969Z 
2020-03-06T18:37:07.5865185Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5865185Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5865385Z 
2020-03-06T18:37:07.5866006Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5866947Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5867220Z 
2020-03-06T18:37:07.5867220Z 
2020-03-06T18:37:07.5867878Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath --crate-type dylib
2020-03-06T18:37:07.5868344Z 
2020-03-06T18:37:07.5868721Z ------------------------------------------
2020-03-06T18:37:07.5869033Z 
2020-03-06T18:37:07.5869135Z 
2020-03-06T18:37:07.5869135Z 
2020-03-06T18:37:07.5869539Z ---- [ui] ui/issues/issue-36856.rs stdout ----
2020-03-06T18:37:07.5869740Z 
2020-03-06T18:37:07.5870464Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5870732Z status: exit code: 101
2020-03-06T18:37:07.5872649Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-36856.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36856/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36856/auxiliary"
2020-03-06T18:37:07.5874203Z ------------------------------------------
2020-03-06T18:37:07.5874378Z 
2020-03-06T18:37:07.5878739Z ------------------------------------------
2020-03-06T18:37:07.5878992Z stderr:
2020-03-06T18:37:07.5878992Z stderr:
2020-03-06T18:37:07.5879375Z ------------------------------------------
2020-03-06T18:37:07.5880199Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5880930Z 
2020-03-06T18:37:07.5881148Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5881344Z 
2020-03-06T18:37:07.5881581Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5881581Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5881783Z 
2020-03-06T18:37:07.5882414Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5883204Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5883459Z 
2020-03-06T18:37:07.5883459Z 
2020-03-06T18:37:07.5884051Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath
2020-03-06T18:37:07.5884498Z 
2020-03-06T18:37:07.5884864Z ------------------------------------------
2020-03-06T18:37:07.5885039Z 
2020-03-06T18:37:07.5885153Z 
2020-03-06T18:37:07.5885153Z 
2020-03-06T18:37:07.5885536Z ---- [ui] ui/sepcomp/sepcomp-lib-lto.rs stdout ----
2020-03-06T18:37:07.5885729Z 
2020-03-06T18:37:07.5886117Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5886398Z status: exit code: 101
2020-03-06T18:37:07.5888654Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/sepcomp/sepcomp-lib-lto.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sepcomp/sepcomp-lib-lto/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sepcomp/sepcomp-lib-lto/auxiliary"
2020-03-06T18:37:07.5890386Z ------------------------------------------
2020-03-06T18:37:07.5890571Z 
2020-03-06T18:37:07.5890955Z ------------------------------------------
2020-03-06T18:37:07.5891159Z stderr:
2020-03-06T18:37:07.5891159Z stderr:
2020-03-06T18:37:07.5891527Z ------------------------------------------
2020-03-06T18:37:07.5892159Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5892857Z 
2020-03-06T18:37:07.5893091Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5893296Z 
2020-03-06T18:37:07.5893515Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5893515Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5893715Z 
2020-03-06T18:37:07.5894344Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5895109Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5895379Z 
2020-03-06T18:37:07.5895379Z 
2020-03-06T18:37:07.5895948Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C rpath -C lto
2020-03-06T18:37:07.5896357Z 
2020-03-06T18:37:07.5896728Z ------------------------------------------
2020-03-06T18:37:07.5896901Z 
2020-03-06T18:37:07.5896999Z 
2020-03-06T18:37:07.5896999Z 
2020-03-06T18:37:07.5897352Z ---- [ui] ui/std-backtrace.rs stdout ----
2020-03-06T18:37:07.5897526Z 
2020-03-06T18:37:07.5897931Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5898195Z status: exit code: 101
2020-03-06T18:37:07.5900047Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/std-backtrace.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std-backtrace/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/std-backtrace/auxiliary"
2020-03-06T18:37:07.5901628Z ------------------------------------------
2020-03-06T18:37:07.5901822Z 
2020-03-06T18:37:07.5902183Z ------------------------------------------
2020-03-06T18:37:07.5902387Z stderr:
2020-03-06T18:37:07.5902387Z stderr:
2020-03-06T18:37:07.5902770Z ------------------------------------------
2020-03-06T18:37:07.5903384Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5904103Z 
2020-03-06T18:37:07.5904320Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5904517Z 
2020-03-06T18:37:07.5904735Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5904735Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5904951Z 
2020-03-06T18:37:07.5905556Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5906339Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5906593Z 
2020-03-06T18:37:07.5906593Z 
2020-03-06T18:37:07.5907187Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath
2020-03-06T18:37:07.5907633Z 
2020-03-06T18:37:07.5907990Z ------------------------------------------
2020-03-06T18:37:07.5908164Z 
2020-03-06T18:37:07.5908263Z 
2020-03-06T18:37:07.5908263Z 
2020-03-06T18:37:07.5908731Z ---- [ui] ui/unboxed-closures/unboxed-closures-unique-type-id.rs stdout ----
2020-03-06T18:37:07.5908965Z 
2020-03-06T18:37:07.5909351Z error: test compilation failed although it shouldn't!
2020-03-06T18:37:07.5909630Z status: exit code: 101
2020-03-06T18:37:07.5911638Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-unique-type-id.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-unique-type-id/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-unique-type-id/auxiliary"
2020-03-06T18:37:07.5913298Z ------------------------------------------
2020-03-06T18:37:07.5913475Z 
2020-03-06T18:37:07.5913850Z ------------------------------------------
2020-03-06T18:37:07.5914055Z stderr:
2020-03-06T18:37:07.5914055Z stderr:
2020-03-06T18:37:07.5918272Z ------------------------------------------
2020-03-06T18:37:07.5922534Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-03-06T18:37:07.5924323Z 
2020-03-06T18:37:07.5924561Z error: internal compiler error: unexpected panic
2020-03-06T18:37:07.5924783Z 
2020-03-06T18:37:07.5925004Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5925004Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-06T18:37:07.5925204Z 
2020-03-06T18:37:07.5925982Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-06T18:37:07.5926766Z note: rustc 1.43.0-nightly (b98862ecf 2020-03-06) running on x86_64-unknown-linux-gnu
2020-03-06T18:37:07.5927022Z 
2020-03-06T18:37:07.5927022Z 
2020-03-06T18:37:07.5927786Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath
2020-03-06T18:37:07.5928233Z 
2020-03-06T18:37:07.5928652Z ------------------------------------------
2020-03-06T18:37:07.5928827Z 
2020-03-06T18:37:07.5928924Z 
---
2020-03-06T18:37:07.5936370Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-06T18:37:07.5936955Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-06T18:37:07.5937192Z 
2020-03-06T18:37:07.5937291Z 
2020-03-06T18:37:07.5941166Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-06T18:37:07.5943774Z 
2020-03-06T18:37:07.5943893Z 
2020-03-06T18:37:07.5944132Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-06T18:37:07.5944461Z Build completed unsuccessfully in 1:06:49
2020-03-06T18:37:07.5944461Z Build completed unsuccessfully in 1:06:49
2020-03-06T18:37:07.5944923Z == clock drift check ==
2020-03-06T18:37:07.5945175Z   local time: Fri Mar  6 18:37:07 UTC 2020
2020-03-06T18:37:07.8833186Z   network time: Fri, 06 Mar 2020 18:37:07 GMT
2020-03-06T18:37:07.8843314Z == end clock drift check ==
2020-03-06T18:37:08.4030805Z 
2020-03-06T18:37:08.4105942Z ##[error]Bash exited with code '1'.
2020-03-06T18:37:08.4120817Z ##[section]Finishing: Run build
2020-03-06T18:37:08.4177150Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69779/merge to s
2020-03-06T18:37:08.4182720Z Task         : Get sources
2020-03-06T18:37:08.4183158Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-06T18:37:08.4183564Z Version      : 1.0.0
2020-03-06T18:37:08.4183841Z Author       : Microsoft
2020-03-06T18:37:08.4183841Z Author       : Microsoft
2020-03-06T18:37:08.4184275Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-06T18:37:08.4184799Z ==============================================================================
2020-03-06T18:37:08.7796924Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-06T18:37:08.7841716Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69779/merge to s
2020-03-06T18:37:08.7936749Z Cleaning up task key
2020-03-06T18:37:08.7938114Z Start cleaning up orphan processes.
2020-03-06T18:37:08.8141423Z Terminate orphan process: pid (3713) (python)
2020-03-06T18:37:08.8380047Z ##[section]Finishing: Finalize Job
