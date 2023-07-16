plain
2020-01-20T20:30:32.9151874Z ========================== Starting Command Output ===========================
2020-01-20T20:30:32.9154281Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/613b7462-9c13-4e96-841a-df68d020f4a9.sh
2020-01-20T20:30:32.9154494Z 
2020-01-20T20:30:32.9157614Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-20T20:30:32.9163520Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68402/merge to s
2020-01-20T20:30:32.9165222Z Task         : Get sources
2020-01-20T20:30:32.9165255Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-20T20:30:32.9165287Z Version      : 1.0.0
2020-01-20T20:30:32.9165320Z Author       : Microsoft
---
2020-01-20T20:30:33.9225747Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-20T20:30:33.9242086Z ##[command]git config gc.auto 0
2020-01-20T20:30:33.9246258Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-20T20:30:33.9250184Z ##[command]git config --get-all http.proxy
2020-01-20T20:30:33.9259223Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68402/merge:refs/remotes/pull/68402/merge
---
2020-01-20T21:32:21.1921680Z .................................................................................................... 1700/9539
2020-01-20T21:32:28.2789041Z .................................................................................................... 1800/9539
2020-01-20T21:32:41.5737471Z ...................i................................................................................ 1900/9539
2020-01-20T21:32:49.4497452Z .................................................................................................... 2000/9539
2020-01-20T21:33:07.3857338Z .........iiiii...................................................................................... 2100/9539
2020-01-20T21:33:17.9253905Z .................................................................................................... 2300/9539
2020-01-20T21:33:20.5826958Z .................................................................................................... 2400/9539
2020-01-20T21:33:26.3918378Z .................................................................................................... 2500/9539
2020-01-20T21:33:51.3695312Z .................................................................................................... 2600/9539
---
2020-01-20T21:36:49.4217612Z .....................................................i...............i.............................. 4900/9539
2020-01-20T21:36:58.2812506Z .................................................................................................... 5000/9539
2020-01-20T21:37:07.8709195Z ................................................................................................i... 5100/9539
2020-01-20T21:37:13.8320549Z .................................................................................................... 5200/9539
2020-01-20T21:37:26.5512057Z ....................................................................ii.ii...........i............... 5300/9539
2020-01-20T21:37:36.8308205Z .....i.............................................................................................. 5500/9539
2020-01-20T21:37:49.2061079Z .................................................................................................... 5600/9539
2020-01-20T21:37:56.7522276Z ......................................................i............................................. 5700/9539
2020-01-20T21:38:04.8007103Z .................................................................................................... 5800/9539
2020-01-20T21:38:04.8007103Z .................................................................................................... 5800/9539
2020-01-20T21:38:16.6264663Z .................................................................................................... 5900/9539
2020-01-20T21:38:24.7140819Z .............................................ii...i..ii...........i................................. 6000/9539
2020-01-20T21:38:50.3463829Z .................................................................................................... 6200/9539
2020-01-20T21:39:00.0318294Z .................................................................................................... 6300/9539
2020-01-20T21:39:00.0318294Z .................................................................................................... 6300/9539
2020-01-20T21:39:06.7092505Z .........................................................................i..ii...................... 6400/9539
2020-01-20T21:39:42.4963561Z .................................................................................................... 6600/9539
2020-01-20T21:39:45.8174245Z .................................................i.................................................. 6700/9539
2020-01-20T21:39:48.3476152Z .................................................................................................... 6800/9539
2020-01-20T21:39:51.0333529Z ................................................i................................................... 6900/9539
---
2020-01-20T21:41:46.7751447Z .................................................................................................... 7500/9539
2020-01-20T21:41:52.1732894Z .................................................................................................... 7600/9539
2020-01-20T21:41:58.7472033Z .................................................................................................... 7700/9539
2020-01-20T21:42:06.5858336Z .................................................................................................... 7800/9539
2020-01-20T21:42:19.6657039Z ...................................................................................................i 7900/9539
2020-01-20T21:42:27.0805938Z iiiiii.............................................................................................. 8000/9539
2020-01-20T21:42:45.5098611Z .................................................................................................... 8200/9539
2020-01-20T21:42:59.2656660Z .................................................................................................... 8300/9539
2020-01-20T21:43:13.3867996Z .................................................................................................... 8400/9539
2020-01-20T21:43:20.0845400Z .................................................................................................... 8500/9539
---
2020-01-20T21:46:09.0897276Z  finished in 8.496
2020-01-20T21:46:09.0897768Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-20T21:46:09.0897972Z 
2020-01-20T21:46:09.0898146Z running 166 tests
2020-01-20T21:46:12.2874069Z iiii......i........ii..iiii...i....i...........i............i.Fi...F..............i..F.i............ 100/166
2020-01-20T21:46:15.2517464Z i.i.i...iii..iiiiiiiF......................iii........F.F.ii......
2020-01-20T21:46:15.2529992Z 
2020-01-20T21:46:15.2532201Z ---- [codegen] codegen/issue-34947-pow-i32.rs stdout ----
2020-01-20T21:46:15.2532468Z 
2020-01-20T21:46:15.2532468Z 
2020-01-20T21:46:15.2533713Z error: verification with 'FileCheck' failed
2020-01-20T21:46:15.2534091Z status: exit code: 1
2020-01-20T21:46:15.2535268Z command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-34947-pow-i32/issue-34947-pow-i32.ll" "/checkout/src/test/codegen/issue-34947-pow-i32.rs"
2020-01-20T21:46:15.2535926Z ------------------------------------------
2020-01-20T21:46:15.2536117Z 
2020-01-20T21:46:15.2536473Z ------------------------------------------
2020-01-20T21:46:15.2536649Z stderr:
2020-01-20T21:46:15.2536649Z stderr:
2020-01-20T21:46:15.2537018Z ------------------------------------------
2020-01-20T21:46:15.2537491Z /checkout/src/test/codegen/issue-34947-pow-i32.rs:9:17: error: CHECK-NEXT: is not on the line after the previous match
2020-01-20T21:46:15.2538316Z  // CHECK-NEXT: mul
2020-01-20T21:46:15.2538642Z                 ^
2020-01-20T21:46:15.2539278Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-34947-pow-i32/issue-34947-pow-i32.ll:14:15: note: 'next' match was here
2020-01-20T21:46:15.2539496Z  %_15.0.1.i = mul i32 %_15.0.i, %x
2020-01-20T21:46:15.2539637Z               ^
2020-01-20T21:46:15.2540087Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-34947-pow-i32/issue-34947-pow-i32.ll:11:16: note: previous match ended here
2020-01-20T21:46:15.2540526Z  %_15.0.i = mul i32 %x, %x
2020-01-20T21:46:15.2540662Z                ^
2020-01-20T21:46:15.2541116Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-34947-pow-i32/issue-34947-pow-i32.ll:12:1: note: non-matching line after previous match is here
2020-01-20T21:46:15.2541319Z  tail call void @llvm.sideeffect() #2
2020-01-20T21:46:15.2541583Z 
2020-01-20T21:46:15.2541939Z ------------------------------------------
2020-01-20T21:46:15.2542558Z 
2020-01-20T21:46:15.2542739Z 
2020-01-20T21:46:15.2542739Z 
2020-01-20T21:46:15.2543168Z ---- [codegen] codegen/issue-45222.rs stdout ----
2020-01-20T21:46:15.2543337Z 
2020-01-20T21:46:15.2543687Z error: verification with 'FileCheck' failed
2020-01-20T21:46:15.2543862Z status: exit code: 1
2020-01-20T21:46:15.2544378Z command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll" "/checkout/src/test/codegen/issue-45222.rs"
2020-01-20T21:46:15.2545409Z ------------------------------------------
2020-01-20T21:46:15.2545587Z 
2020-01-20T21:46:15.2545944Z ------------------------------------------
2020-01-20T21:46:15.2546139Z stderr:
2020-01-20T21:46:15.2546139Z stderr:
2020-01-20T21:46:15.2546488Z ------------------------------------------
2020-01-20T21:46:15.2546933Z /checkout/src/test/codegen/issue-45222.rs:23:12: error: CHECK: expected string not found in input
2020-01-20T21:46:15.2547160Z  // CHECK: ret i64 500005000000000
2020-01-20T21:46:15.2547307Z            ^
2020-01-20T21:46:15.2547760Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:10:23: note: scanning from here
2020-01-20T21:46:15.2547992Z define i64 @check_foo2() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
2020-01-20T21:46:15.2548149Z                       ^
2020-01-20T21:46:15.2548736Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:97:21: note: possible intended match here
2020-01-20T21:46:15.2548914Z  %exitcond.i = icmp eq i64 %4, 100000
2020-01-20T21:46:15.2549438Z /checkout/src/test/codegen/issue-45222.rs:40:12: error: CHECK: expected string not found in input
2020-01-20T21:46:15.2549606Z  // CHECK: ret i64 5000050000
2020-01-20T21:46:15.2549751Z            ^
2020-01-20T21:46:15.2549751Z            ^
2020-01-20T21:46:15.2550171Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:111:31: note: scanning from here
2020-01-20T21:46:15.2550365Z define i64 @check_triangle_inc() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
2020-01-20T21:46:15.2550530Z                               ^
2020-01-20T21:46:15.2550962Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:176:2: note: possible intended match here
2020-01-20T21:46:15.2554487Z  ret i64 %count.0.i
2020-01-20T21:46:15.2556924Z 
2020-01-20T21:46:15.2576587Z ------------------------------------------
2020-01-20T21:46:15.2576889Z 
2020-01-20T21:46:15.2577035Z 
2020-01-20T21:46:15.2577035Z 
2020-01-20T21:46:15.2577440Z ---- [codegen] codegen/naked-functions.rs stdout ----
2020-01-20T21:46:15.2577612Z 
2020-01-20T21:46:15.2578002Z error: verification with 'FileCheck' failed
2020-01-20T21:46:15.2578583Z status: exit code: 1
2020-01-20T21:46:15.2579228Z command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll" "/checkout/src/test/codegen/naked-functions.rs"
2020-01-20T21:46:15.2579842Z ------------------------------------------
2020-01-20T21:46:15.2579994Z 
2020-01-20T21:46:15.2580353Z ------------------------------------------
2020-01-20T21:46:15.2581019Z stderr:
2020-01-20T21:46:15.2581019Z stderr:
2020-01-20T21:46:15.2581538Z ------------------------------------------
2020-01-20T21:46:15.2583125Z /checkout/src/test/codegen/naked-functions.rs:14:17: error: CHECK-NEXT: is not on the line after the previous match
2020-01-20T21:46:15.2583337Z  // CHECK-NEXT: ret void
2020-01-20T21:46:15.2583404Z                 ^
2020-01-20T21:46:15.2583847Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:10:2: note: 'next' match was here
2020-01-20T21:46:15.2583910Z  ret void
2020-01-20T21:46:15.2583971Z  ^
2020-01-20T21:46:15.2584264Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:8:7: note: previous match ended here
2020-01-20T21:46:15.2584372Z       ^
2020-01-20T21:46:15.2584372Z       ^
2020-01-20T21:46:15.2585072Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:9:1: note: non-matching line after previous match is here
2020-01-20T21:46:15.2585141Z  call void @llvm.sideeffect()
2020-01-20T21:46:15.2585236Z 
2020-01-20T21:46:15.2585485Z ------------------------------------------
2020-01-20T21:46:15.2585519Z 
2020-01-20T21:46:15.2585546Z 
2020-01-20T21:46:15.2585546Z 
2020-01-20T21:46:15.2585807Z ---- [codegen] codegen/repeat-trusted-len.rs stdout ----
2020-01-20T21:46:15.2585841Z 
2020-01-20T21:46:15.2586060Z error: verification with 'FileCheck' failed
2020-01-20T21:46:15.2586132Z status: exit code: 1
2020-01-20T21:46:15.2586522Z command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll" "/checkout/src/test/codegen/repeat-trusted-len.rs"
2020-01-20T21:46:15.2586842Z ------------------------------------------
2020-01-20T21:46:15.2586876Z 
2020-01-20T21:46:15.2587094Z ------------------------------------------
2020-01-20T21:46:15.2587140Z stderr:
2020-01-20T21:46:15.2587140Z stderr:
2020-01-20T21:46:15.2587375Z ------------------------------------------
2020-01-20T21:46:15.2587660Z /checkout/src/test/codegen/repeat-trusted-len.rs:16:11: error: CHECK: expected string not found in input
2020-01-20T21:46:15.2587982Z // CHECK: call void @llvm.memset.p0i8.[[USIZE]](i8* {{(nonnull )?}}align 1{{.*}} %{{[0-9]+}}, i8 42, [[USIZE]] 100000, i1 false)
2020-01-20T21:46:15.2588058Z           ^
2020-01-20T21:46:15.2588514Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:18:33: note: scanning from here
2020-01-20T21:46:15.2588596Z define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(24)) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
2020-01-20T21:46:15.2588683Z                                 ^
2020-01-20T21:46:15.2588989Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:18:33: note: with variable "USIZE" equal to "i64"
2020-01-20T21:46:15.2589091Z define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(24)) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
2020-01-20T21:46:15.2589151Z                                 ^
2020-01-20T21:46:15.2589470Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:18:33: note: with variable "USIZE" equal to "i64"
2020-01-20T21:46:15.2589746Z define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(24)) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
2020-01-20T21:46:15.2589854Z 
2020-01-20T21:46:15.2590090Z ------------------------------------------
2020-01-20T21:46:15.2590120Z 
2020-01-20T21:46:15.2590145Z 
2020-01-20T21:46:15.2590145Z 
2020-01-20T21:46:15.2590412Z ---- [codegen] codegen/unwind-extern-exports.rs stdout ----
2020-01-20T21:46:15.2590444Z 
2020-01-20T21:46:15.2590639Z error: verification with 'FileCheck' failed
2020-01-20T21:46:15.2590684Z status: exit code: 1
2020-01-20T21:46:15.2591068Z command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-extern-exports/unwind-extern-exports.ll" "/checkout/src/test/codegen/unwind-extern-exports.rs"
2020-01-20T21:46:15.2591328Z ------------------------------------------
2020-01-20T21:46:15.2591378Z 
2020-01-20T21:46:15.2591572Z ------------------------------------------
2020-01-20T21:46:15.2591620Z stderr:
2020-01-20T21:46:15.2591620Z stderr:
2020-01-20T21:46:15.2591830Z ------------------------------------------
2020-01-20T21:46:15.2592089Z /checkout/src/test/codegen/unwind-extern-exports.rs:9:15: error: CHECK-NOT: excluded string found in input
2020-01-20T21:46:15.2592274Z // CHECK-NOT: nounwind
2020-01-20T21:46:15.2592317Z               ^
2020-01-20T21:46:15.2592618Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-extern-exports/unwind-extern-exports.ll:30:39: note: found here
2020-01-20T21:46:15.2592671Z ; Function Attrs: inaccessiblememonly nounwind
2020-01-20T21:46:15.2592761Z 
2020-01-20T21:46:15.2592955Z ------------------------------------------
2020-01-20T21:46:15.2592985Z 
2020-01-20T21:46:15.2593009Z 
2020-01-20T21:46:15.2593009Z 
2020-01-20T21:46:15.2593236Z ---- [codegen] codegen/unwind-extern-imports.rs stdout ----
2020-01-20T21:46:15.2593277Z 
2020-01-20T21:46:15.2593474Z error: verification with 'FileCheck' failed
2020-01-20T21:46:15.2593524Z status: exit code: 1
2020-01-20T21:46:15.2593976Z command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-extern-imports/unwind-extern-imports.ll" "/checkout/src/test/codegen/unwind-extern-imports.rs"
2020-01-20T21:46:15.2594224Z ------------------------------------------
2020-01-20T21:46:15.2594272Z 
2020-01-20T21:46:15.2594469Z ------------------------------------------
2020-01-20T21:46:15.2594510Z stderr:
2020-01-20T21:46:15.2594510Z stderr:
2020-01-20T21:46:15.2595137Z ------------------------------------------
2020-01-20T21:46:15.2595456Z /checkout/src/test/codegen/unwind-extern-imports.rs:9:16: error: CHECK-NEXT: is not on the line after the previous match
2020-01-20T21:46:15.2595680Z // CHECK-NEXT: declare void @extern_fn
2020-01-20T21:46:15.2595747Z                ^
2020-01-20T21:46:15.2596087Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-extern-imports/unwind-extern-imports.ll:42:1: note: 'next' match was here
2020-01-20T21:46:15.2596154Z declare void @extern_fn() unnamed_addr #2
2020-01-20T21:46:15.2596199Z ^
2020-01-20T21:46:15.2596548Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-extern-imports/unwind-extern-imports.ll:38:47: note: previous match ended here
2020-01-20T21:46:15.2596608Z ; Function Attrs: inaccessiblememonly nounwind
2020-01-20T21:46:15.2596657Z                                               ^
2020-01-20T21:46:15.2597027Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-extern-imports/unwind-extern-imports.ll:39:1: note: non-matching line after previous match is here
2020-01-20T21:46:15.2597087Z declare void @llvm.sideeffect() #1
2020-01-20T21:46:15.2597178Z 
2020-01-20T21:46:15.2597398Z ------------------------------------------
2020-01-20T21:46:15.2597537Z 
2020-01-20T21:46:15.2597565Z 
---
2020-01-20T21:46:15.2599884Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-20T21:46:15.2599939Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-20T21:46:15.2599969Z 
2020-01-20T21:46:15.2599993Z 
2020-01-20T21:46:15.2601513Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-20T21:46:15.2601751Z 
2020-01-20T21:46:15.2601796Z 
2020-01-20T21:46:15.2601842Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-20T21:46:15.2601889Z Build completed unsuccessfully in 1:09:25
2020-01-20T21:46:15.2601889Z Build completed unsuccessfully in 1:09:25
2020-01-20T21:46:15.2612491Z == clock drift check ==
2020-01-20T21:46:15.2625146Z   local time: Mon Jan 20 21:46:15 UTC 2020
2020-01-20T21:46:15.5542802Z   network time: Mon, 20 Jan 2020 21:46:15 GMT
2020-01-20T21:46:15.5547440Z == end clock drift check ==
2020-01-20T21:46:17.6780965Z 
2020-01-20T21:46:17.6896088Z ##[error]Bash exited with code '1'.
2020-01-20T21:46:17.6910086Z ##[section]Finishing: Run build
2020-01-20T21:46:17.6932151Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68402/merge to s
2020-01-20T21:46:17.6933762Z Task         : Get sources
2020-01-20T21:46:17.6933813Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-20T21:46:17.6933855Z Version      : 1.0.0
2020-01-20T21:46:17.6933891Z Author       : Microsoft
2020-01-20T21:46:17.6933891Z Author       : Microsoft
2020-01-20T21:46:17.6933937Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-20T21:46:17.6933981Z ==============================================================================
2020-01-20T21:46:18.1403527Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-20T21:46:18.1448696Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68402/merge to s
2020-01-20T21:46:18.1605619Z Cleaning up task key
2020-01-20T21:46:18.1606436Z Start cleaning up orphan processes.
2020-01-20T21:46:18.1728142Z Terminate orphan process: pid (3387) (python)
2020-01-20T21:46:18.2019114Z ##[section]Finishing: Finalize Job
