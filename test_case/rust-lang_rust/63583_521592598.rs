plain
2019-08-15T08:15:14.3792251Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-15T08:15:14.3792534Z 
2019-08-15T08:15:14.3793776Z   git checkout -b <new-branch-name>
2019-08-15T08:15:14.3793975Z 
2019-08-15T08:15:14.3794547Z HEAD is now at 5e907a4b7 Auto merge of #63583 - Centril:rollup-5s17x9k, r=Centril
2019-08-15T08:15:14.3940714Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-15T08:15:14.3944030Z ==============================================================================
2019-08-15T08:15:14.3944116Z Task         : Bash
2019-08-15T08:15:14.3944169Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-15T08:24:03.3837650Z + OUTPUT=/usr/local
2019-08-15T08:24:03.3837863Z + shift
2019-08-15T08:24:03.3838325Z + export 'CFLAGS=-fPIC '
2019-08-15T08:24:03.3838671Z + CFLAGS='-fPIC '
2019-08-15T08:24:03.3891034Z + git clone https://github.com/richfelker/musl-cross-make -b v0.9.8
2019-08-15T08:24:03.3892054Z Cloning into 'musl-cross-make'...
2019-08-15T08:24:03.7453975Z Note: checking out '629189831f61b73ba1053624eee12ff6a816438f'.
2019-08-15T08:24:03.7455293Z You are in 'detached HEAD' state. You can look around, make experimental
2019-08-15T08:24:03.7455523Z changes and commit them, and you can discard any commits you make in this
2019-08-15T08:24:03.7455711Z state without impacting any branches by performing another checkout.
2019-08-15T08:24:03.7455844Z 
---
2019-08-15T08:39:07.9779642Z musl-toolchain.sh: line 9: 25264 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-08-15T08:39:07.9779984Z + cd -
2019-08-15T08:39:07.9780497Z + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
2019-08-15T08:39:07.9780779Z /build
2019-08-15T08:39:07.9789332Z + echo /usr/local/x86_64-linux-musl/lib
2019-08-15T08:39:07.9795409Z + '[' '' = 1 ']'
2019-08-15T08:39:07.9795767Z + export CC=x86_64-linux-musl-gcc
2019-08-15T08:39:07.9796286Z + export CXX=x86_64-linux-musl-g++
2019-08-15T08:39:07.9796550Z + CXX=x86_64-linux-musl-g++
2019-08-15T08:39:07.9796625Z + LLVM=70
2019-08-15T08:39:07.9796908Z + '[' '!' -d libunwind-release_70 ']'
---
2019-08-15T10:15:07.2577965Z test [mir-opt] mir-opt/storage_live_dead_in_statics.rs ... ok
2019-08-15T10:15:07.2578253Z 
2019-08-15T10:15:07.2578344Z failures:
2019-08-15T10:15:07.2578383Z 
2019-08-15T10:15:07.2578700Z ---- [mir-opt] mir-opt/retag.rs stdout ----
2019-08-15T10:15:07.2579057Z thread '[mir-opt] mir-opt/retag.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
2019-08-15T10:15:07.2579437Z Expected Line: "        _3 = const Test::foo(move _4, move _6) -> [return: bb2, unwind: bb3];"
2019-08-15T10:15:07.2579583Z Test Name: rustc.main.EraseRegions.after.mir
2019-08-15T10:15:07.2579673Z Expected:
2019-08-15T10:15:07.2579736Z ... (elided)
2019-08-15T10:15:07.2580020Z fn main() -> () {
2019-08-15T10:15:07.2580103Z ... (elided)
2019-08-15T10:15:07.2580184Z     bb0: {
2019-08-15T10:15:07.2580245Z ... (elided)
2019-08-15T10:15:07.2580670Z         _3 = const Test::foo(move _4, move _6) -> [return: bb2, unwind: bb3];
2019-08-15T10:15:07.2580733Z     }
2019-08-15T10:15:07.2580792Z ... (elided)
2019-08-15T10:15:07.2580839Z     bb2: {
2019-08-15T10:15:07.2580900Z         Retag(_3);
2019-08-15T10:15:07.2580948Z ... (elided)
2019-08-15T10:15:07.2581009Z         _9 = move _3;
2019-08-15T10:15:07.2581059Z         Retag(_9);
2019-08-15T10:15:07.2581124Z         _8 = &mut (*_9);
2019-08-15T10:15:07.2581173Z         Retag(_8);
2019-08-15T10:15:07.2581236Z         StorageDead(_9);
2019-08-15T10:15:07.2581286Z         StorageLive(_10);
2019-08-15T10:15:07.2581546Z         _10 = move _8;
2019-08-15T10:15:07.2581596Z         Retag(_10);
2019-08-15T10:15:07.2581662Z ... (elided)
2019-08-15T10:15:07.2581709Z         _13 = &mut (*_10);
2019-08-15T10:15:07.2581774Z         Retag(_13);
2019-08-15T10:15:07.2581831Z         _12 = move _13 as *mut i32 (Misc);
2019-08-15T10:15:07.2581898Z         Retag([raw] _12);
2019-08-15T10:15:07.2581948Z ... (elided)
2019-08-15T10:15:07.2582208Z         _16 = move _17(move _18) -> bb5;
2019-08-15T10:15:07.2582321Z     bb5: {
2019-08-15T10:15:07.2582366Z         Retag(_16);
2019-08-15T10:15:07.2582366Z         Retag(_16);
2019-08-15T10:15:07.2582429Z ... (elided)
2019-08-15T10:15:07.2582669Z         _20 = const Test::foo_shr(move _21, move _23) -> [return: bb6, unwind: bb7];
2019-08-15T10:15:07.2582748Z     }
2019-08-15T10:15:07.2582793Z ... (elided)
2019-08-15T10:15:07.2582897Z Actual:
2019-08-15T10:15:07.2582958Z | User Type Annotations
2019-08-15T10:15:07.2582958Z | User Type Annotations
2019-08-15T10:15:07.2583274Z | 0: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(*mut ^0) } at /checkout/src/test/mir-opt/retag.rs:25:18: 25:29
2019-08-15T10:15:07.2583644Z | 1: Canonical { max_universe: U0, variables: [], value: Ty(for<'r> fn(&'r i32) -> &'r i32) } at /checkout/src/test/mir-opt/retag.rs:29:12: 29:28
2019-08-15T10:15:07.2583992Z | 2: Canonical { max_universe: U0, variables: [], value: Ty(for<'r> fn(&'r i32) -> &'r i32) } at /checkout/src/test/mir-opt/retag.rs:29:12: 29:28
2019-08-15T10:15:07.2584342Z | 3: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(*const ^0) } at /checkout/src/test/mir-opt/retag.rs:36:14: 36:28
2019-08-15T10:15:07.2584441Z |
2019-08-15T10:15:07.2584626Z fn  main() -> () {
2019-08-15T10:15:07.2584695Z     let mut _0: ();
2019-08-15T10:15:07.2584743Z     let mut _1: i32;
2019-08-15T10:15:07.2584808Z     let _2: ();
2019-08-15T10:15:07.2584855Z     let mut _4: &Test;
2019-08-15T10:15:07.2584919Z     let _5: Test;
2019-08-15T10:15:07.2584975Z     let mut _6: &mut i32;
2019-08-15T10:15:07.2585040Z     let mut _7: &mut i32;
2019-08-15T10:15:07.2585088Z     let mut _9: &mut i32;
2019-08-15T10:15:07.2585153Z     let mut _12: *mut i32;
2019-08-15T10:15:07.2585201Z     let mut _13: &mut i32;
2019-08-15T10:15:07.2585384Z     let mut _15: [closure@HirId { owner: DefIndex(22), local_id: 72 }];
2019-08-15T10:15:07.2585636Z     let mut _17: for<'r> fn(&'r i32) -> &'r i32;
2019-08-15T10:15:07.2585711Z     let mut _18: &i32;
2019-08-15T10:15:07.2585758Z     let mut _19: &i32;
2019-08-15T10:15:07.2585824Z     let _20: &i32;
2019-08-15T10:15:07.2585871Z     let mut _21: &Test;
2019-08-15T10:15:07.2585936Z     let _22: Test;
2019-08-15T10:15:07.2585983Z     let mut _23: &i32;
2019-08-15T10:15:07.2586048Z     let mut _24: &i32;
2019-08-15T10:15:07.2586096Z     let _25: i32;
2019-08-15T10:15:07.2586160Z     let mut _27: *const i32;
2019-08-15T10:15:07.2586209Z     let mut _28: &i32;
2019-08-15T10:15:07.2586273Z     scope 1 {
2019-08-15T10:15:07.2586320Z         let _3: &mut i32;
2019-08-15T10:15:07.2587229Z         let _14: for<'r> fn(&'r i32) -> &'r i32 as UserTypeProjection { base: UserType(1), projs: [] };
2019-08-15T10:15:07.2587327Z         scope 2 {
2019-08-15T10:15:07.2587411Z             let _8: &mut i32;
2019-08-15T10:15:07.2587489Z             scope 3 {
2019-08-15T10:15:07.2587571Z                 let _10: &mut i32;
2019-08-15T10:15:07.2587638Z                 scope 4 {
2019-08-15T10:15:07.2587722Z                     let _11: *mut i32;
2019-08-15T10:15:07.2587790Z                     scope 5 {
2019-08-15T10:15:07.2587937Z                 }
2019-08-15T10:15:07.2588016Z             }
2019-08-15T10:15:07.2588076Z         }
2019-08-15T10:15:07.2588154Z         scope 6 {
---
2019-08-15T10:15:07.2588634Z                 }
2019-08-15T10:15:07.2588712Z             }
2019-08-15T10:15:07.2588773Z         }
2019-08-15T10:15:07.2588848Z     }
2019-08-15T10:15:07.2588906Z     bb0: {
2019-08-15T10:15:07.2588984Z         StorageLive(_1);
2019-08-15T10:15:07.2589056Z         _1 = const 0i32;
2019-08-15T10:15:07.2589137Z         StorageLive(_2);
2019-08-15T10:15:07.2589201Z         StorageLive(_3);
2019-08-15T10:15:07.2589283Z         StorageLive(_4);
2019-08-15T10:15:07.2589345Z         StorageLive(_5);
2019-08-15T10:15:07.2589427Z         _5 = Test(const 0i32,);
2019-08-15T10:15:07.2589492Z         _4 = &_5;
2019-08-15T10:15:07.2589573Z         Retag(_4);
2019-08-15T10:15:07.2589636Z         StorageLive(_6);
2019-08-15T10:15:07.2589717Z         StorageLive(_7);
2019-08-15T10:15:07.2589779Z         _7 = &mut _1;
2019-08-15T10:15:07.2589860Z         Retag(_7);
2019-08-15T10:15:07.2589922Z         _6 = &mut (*_7);
2019-08-15T10:15:07.2590004Z         Retag([2phase] _6);
2019-08-15T10:15:07.2590479Z         _3 = const Test::foo(move _4, move _6) -> bb1;
2019-08-15T10:15:07.2590602Z     bb1: {
2019-08-15T10:15:07.2590663Z         Retag(_3);
2019-08-15T10:15:07.2590708Z         StorageDead(_6);
2019-08-15T10:15:07.2590772Z         StorageDead(_4);
2019-08-15T10:15:07.2590772Z         StorageDead(_4);
2019-08-15T10:15:07.2590823Z         StorageDead(_7);
2019-08-15T10:15:07.2591017Z         drop(_5) -> bb2;
2019-08-15T10:15:07.2591123Z     bb2: {
2019-08-15T10:15:07.2591166Z         StorageDead(_5);
2019-08-15T10:15:07.2591228Z         StorageLive(_8);
2019-08-15T10:15:07.2591274Z         StorageLive(_9);
2019-08-15T10:15:07.2591274Z         StorageLive(_9);
2019-08-15T10:15:07.2591336Z         _9 = move _3;
2019-08-15T10:15:07.2591381Z         Retag(_9);
2019-08-15T10:15:07.2591447Z         _8 = &mut (*_9);
2019-08-15T10:15:07.2591492Z         Retag(_8);
2019-08-15T10:15:07.2591553Z         StorageDead(_9);
2019-08-15T10:15:07.2591600Z         StorageLive(_10);
2019-08-15T10:15:07.2591664Z         _10 = move _8;
2019-08-15T10:15:07.2591709Z         Retag(_10);
2019-08-15T10:15:07.2591777Z         StorageLive(_11);
2019-08-15T10:15:07.2591824Z         StorageLive(_12);
2019-08-15T10:15:07.2591887Z         StorageLive(_13);
2019-08-15T10:15:07.2591934Z         _13 = &mut (*_10);
2019-08-15T10:15:07.2592000Z         Retag(_13);
2019-08-15T10:15:07.2592117Z         _12 = move _13 as *mut i32 (Misc);
2019-08-15T10:15:07.2592194Z         Retag([raw] _12);
2019-08-15T10:15:07.2592239Z         StorageDead(_13);
2019-08-15T10:15:07.2592302Z         _11 = _12;
2019-08-15T10:15:07.2592347Z         StorageDead(_12);
2019-08-15T10:15:07.2592410Z         _2 = ();
2019-08-15T10:15:07.2592455Z         StorageDead(_11);
2019-08-15T10:15:07.2592522Z         StorageDead(_10);
2019-08-15T10:15:07.2592568Z         StorageDead(_8);
2019-08-15T10:15:07.2592632Z         StorageDead(_3);
2019-08-15T10:15:07.2592678Z         StorageDead(_2);
2019-08-15T10:15:07.2592742Z         StorageLive(_14);
2019-08-15T10:15:07.2592787Z         StorageLive(_15);
2019-08-15T10:15:07.2592858Z         _15 = [closure@HirId { owner: DefIndex(22), local_id: 72 }];
2019-08-15T10:15:07.2592923Z         Retag(_15);
2019-08-15T10:15:07.2593216Z         _14 = move _15 as for<'r> fn(&'r i32) -> &'r i32 (Pointer(ClosureFnPointer(Normal)));
2019-08-15T10:15:07.2593292Z         StorageDead(_15);
2019-08-15T10:15:07.2593359Z         StorageLive(_16);
2019-08-15T10:15:07.2593408Z         StorageLive(_17);
2019-08-15T10:15:07.2593647Z         _17 = _14;
2019-08-15T10:15:07.2593696Z         StorageLive(_18);
2019-08-15T10:15:07.2593760Z         StorageLive(_19);
2019-08-15T10:15:07.2593808Z         _19 = &_1;
2019-08-15T10:15:07.2593872Z         Retag(_19);
2019-08-15T10:15:07.2593920Z         _18 = &(*_19);
2019-08-15T10:15:07.2593985Z         Retag(_18);
2019-08-15T10:15:07.2594192Z         _16 = move _17(move _18) -> bb3;
2019-08-15T10:15:07.2594306Z     bb3: {
2019-08-15T10:15:07.2594366Z         Retag(_16);
2019-08-15T10:15:07.2594413Z         StorageDead(_18);
2019-08-15T10:15:07.2594560Z         StorageDead(_17);
2019-08-15T10:15:07.2594560Z         StorageDead(_17);
2019-08-15T10:15:07.2594609Z         StorageDead(_19);
2019-08-15T10:15:07.2594673Z         StorageLive(_20);
2019-08-15T10:15:07.2594721Z         StorageLive(_21);
2019-08-15T10:15:07.2594786Z         StorageLive(_22);
2019-08-15T10:15:07.2594842Z         _22 = Test(const 0i32,);
2019-08-15T10:15:07.2594909Z         _21 = &_22;
2019-08-15T10:15:07.2594957Z         Retag(_21);
2019-08-15T10:15:07.2595021Z         StorageLive(_23);
2019-08-15T10:15:07.2595070Z         StorageLive(_24);
2019-08-15T10:15:07.2595137Z         _24 = &(promoted[0]: i32);
2019-08-15T10:15:07.2595188Z         Retag(_24);
2019-08-15T10:15:07.2595252Z         _23 = &(*_24);
2019-08-15T10:15:07.2595300Z         Retag(_23);
2019-08-15T10:15:07.2595558Z         _20 = const Test::foo_shr(move _21, move _23) -> bb4;
2019-08-15T10:15:07.2595677Z     bb4: {
2019-08-15T10:15:07.2595721Z         Retag(_20);
2019-08-15T10:15:07.2595786Z         StorageDead(_23);
2019-08-15T10:15:07.2595842Z         StorageDead(_21);
2019-08-15T10:15:07.2595842Z         StorageDead(_21);
2019-08-15T10:15:07.2595908Z         StorageDead(_24);
2019-08-15T10:15:07.2596096Z         drop(_22) -> bb5;
2019-08-15T10:15:07.2596207Z     bb5: {
2019-08-15T10:15:07.2596276Z         StorageDead(_22);
2019-08-15T10:15:07.2596325Z         StorageDead(_20);
2019-08-15T10:15:07.2596388Z         StorageLive(_26);
2019-08-15T10:15:07.2596388Z         StorageLive(_26);
2019-08-15T10:15:07.2596436Z         StorageLive(_27);
2019-08-15T10:15:07.2597107Z         StorageLive(_28);
2019-08-15T10:15:07.2597171Z         _28 = &(*_16);
2019-08-15T10:15:07.2597250Z         Retag(_28);
2019-08-15T10:15:07.2597317Z         _27 = move _28 as *const i32 (Misc);
2019-08-15T10:15:07.2597402Z         Retag([raw] _27);
2019-08-15T10:15:07.2597464Z         StorageDead(_28);
2019-08-15T10:15:07.2597545Z         _26 = _27;
2019-08-15T10:15:07.2597607Z         StorageDead(_27);
2019-08-15T10:15:07.2597690Z         _0 = ();
2019-08-15T10:15:07.2597752Z         StorageDead(_26);
2019-08-15T10:15:07.2597842Z         StorageDead(_16);
2019-08-15T10:15:07.2597904Z         StorageDead(_14);
2019-08-15T10:15:07.2597989Z         StorageDead(_1);
2019-08-15T10:15:07.2598130Z     }
2019-08-15T10:15:07.2598544Z }', src/tools/compiletest/src/runtest.rs:3212:13
2019-08-15T10:15:07.2598664Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-15T10:15:07.2598721Z 
---
2019-08-15T10:15:07.2599459Z test result: FAILED. 48 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out
2019-08-15T10:15:07.2599537Z 
2019-08-15T10:15:07.2599572Z 
2019-08-15T10:15:07.2599606Z 
2019-08-15T10:15:07.2601384Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "mir-opt" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-15T10:15:07.2601945Z 
2019-08-15T10:15:07.2601980Z 
2019-08-15T10:15:07.2602244Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-15T10:15:07.2602587Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-08-15T10:15:07.2602587Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-08-15T10:15:07.2602692Z Build completed unsuccessfully in 1:19:55
2019-08-15T10:15:07.2629536Z == clock drift check ==
2019-08-15T10:15:07.2652272Z   local time: Thu Aug 15 10:15:07 UTC 2019
2019-08-15T10:15:07.3393289Z   network time: Thu, 15 Aug 2019 10:15:07 GMT
2019-08-15T10:15:07.3399669Z == end clock drift check ==
2019-08-15T10:15:10.1412576Z ##[error]Bash exited with code '1'.
2019-08-15T10:15:10.1455202Z ##[section]Starting: Upload CPU usage statistics
2019-08-15T10:15:10.1472251Z ==============================================================================
2019-08-15T10:15:10.1472347Z Task         : Bash
2019-08-15T10:15:10.1472407Z Description  : Run a Bash script on macOS, Linux, or Windows
