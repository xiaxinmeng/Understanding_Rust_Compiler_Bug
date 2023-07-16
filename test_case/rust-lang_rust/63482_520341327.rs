plain
2019-08-12T07:01:12.8894983Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-12T07:01:12.8895232Z 
2019-08-12T07:01:12.8895494Z   git checkout -b <new-branch-name>
2019-08-12T07:01:12.8895556Z 
2019-08-12T07:01:12.8899726Z HEAD is now at 0e0328cc7 Auto merge of #63482 - Centril:rollup-iyakgmg, r=Centril
2019-08-12T07:01:12.9066751Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-12T07:01:12.9069396Z ==============================================================================
2019-08-12T07:01:12.9069485Z Task         : Bash
2019-08-12T07:01:12.9069538Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-12T08:36:27.2935400Z test [mir-opt] mir-opt/storage_live_dead_in_statics.rs ... ok
2019-08-12T08:36:27.2935518Z 
2019-08-12T08:36:27.2935604Z failures:
2019-08-12T08:36:27.2935640Z 
2019-08-12T08:36:27.2935948Z ---- [mir-opt] mir-opt/retag.rs stdout ----
2019-08-12T08:36:27.2936300Z thread '[mir-opt] mir-opt/retag.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
2019-08-12T08:36:27.2936665Z Expected Line: "        _3 = const Test::foo(move _4, move _6) -> [return: bb2, unwind: bb3];"
2019-08-12T08:36:27.2936770Z Test Name: rustc.main.EraseRegions.after.mir
2019-08-12T08:36:27.2936857Z Expected:
2019-08-12T08:36:27.2936920Z ... (elided)
2019-08-12T08:36:27.2937193Z fn main() -> () {
2019-08-12T08:36:27.2937260Z ... (elided)
2019-08-12T08:36:27.2937337Z     bb0: {
2019-08-12T08:36:27.2937426Z ... (elided)
2019-08-12T08:36:27.2937756Z         _3 = const Test::foo(move _4, move _6) -> [return: bb2, unwind: bb3];
2019-08-12T08:36:27.2938364Z     }
2019-08-12T08:36:27.2938597Z ... (elided)
2019-08-12T08:36:27.2938646Z     bb2: {
2019-08-12T08:36:27.2938710Z         Retag(_3);
2019-08-12T08:36:27.2938761Z ... (elided)
2019-08-12T08:36:27.2938826Z         _9 = move _3;
2019-08-12T08:36:27.2938878Z         Retag(_9);
2019-08-12T08:36:27.2938945Z         _8 = &mut (*_9);
2019-08-12T08:36:27.2938997Z         Retag(_8);
2019-08-12T08:36:27.2939064Z         StorageDead(_9);
2019-08-12T08:36:27.2939116Z         StorageLive(_10);
2019-08-12T08:36:27.2939186Z         _10 = move _8;
2019-08-12T08:36:27.2939238Z         Retag(_10);
2019-08-12T08:36:27.2939304Z ... (elided)
2019-08-12T08:36:27.2939353Z         _13 = &mut (*_10);
2019-08-12T08:36:27.2939423Z         Retag(_13);
2019-08-12T08:36:27.2939475Z         _12 = move _13 as *mut i32 (Misc);
2019-08-12T08:36:27.2939546Z         Retag([raw] _12);
2019-08-12T08:36:27.2939608Z ... (elided)
2019-08-12T08:36:27.2939882Z         _16 = move _17(move _18) -> bb5;
2019-08-12T08:36:27.2940001Z     bb5: {
2019-08-12T08:36:27.2940049Z         Retag(_16);
2019-08-12T08:36:27.2940049Z         Retag(_16);
2019-08-12T08:36:27.2940116Z ... (elided)
2019-08-12T08:36:27.2940363Z         _20 = const Test::foo_shr(move _21, move _23) -> [return: bb6, unwind: bb7];
2019-08-12T08:36:27.2940444Z     }
2019-08-12T08:36:27.2940492Z ... (elided)
2019-08-12T08:36:27.2940602Z Actual:
2019-08-12T08:36:27.2940666Z | User Type Annotations
2019-08-12T08:36:27.2940666Z | User Type Annotations
2019-08-12T08:36:27.2940999Z | 0: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(*mut ^0) } at /checkout/src/test/mir-opt/retag.rs:25:18: 25:29
2019-08-12T08:36:27.2941363Z | 1: Canonical { max_universe: U0, variables: [], value: Ty(for<'r> fn(&'r i32) -> &'r i32) } at /checkout/src/test/mir-opt/retag.rs:29:12: 29:28
2019-08-12T08:36:27.2942046Z | 2: Canonical { max_universe: U0, variables: [], value: Ty(for<'r> fn(&'r i32) -> &'r i32) } at /checkout/src/test/mir-opt/retag.rs:29:12: 29:28
2019-08-12T08:36:27.2942465Z | 3: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(*const ^0) } at /checkout/src/test/mir-opt/retag.rs:36:14: 36:28
2019-08-12T08:36:27.2942571Z |
2019-08-12T08:36:27.2942929Z fn  main() -> () {
2019-08-12T08:36:27.2943002Z     let mut _0: ();
2019-08-12T08:36:27.2943232Z     let mut _1: i32;
2019-08-12T08:36:27.2943611Z     let _2: ();
2019-08-12T08:36:27.2943673Z     let mut _4: &Test;
2019-08-12T08:36:27.2943757Z     let _5: Test;
2019-08-12T08:36:27.2943820Z     let mut _6: &mut i32;
2019-08-12T08:36:27.2943901Z     let mut _7: &mut i32;
2019-08-12T08:36:27.2943965Z     let mut _9: &mut i32;
2019-08-12T08:36:27.2944046Z     let mut _12: *mut i32;
2019-08-12T08:36:27.2944110Z     let mut _13: &mut i32;
2019-08-12T08:36:27.2944202Z     let mut _15: [closure@HirId { owner: DefIndex(22), local_id: 72 }];
2019-08-12T08:36:27.2944524Z     let mut _17: for<'r> fn(&'r i32) -> &'r i32;
2019-08-12T08:36:27.2944756Z     let mut _18: &i32;
2019-08-12T08:36:27.2944818Z     let mut _19: &i32;
2019-08-12T08:36:27.2944899Z     let _20: &i32;
2019-08-12T08:36:27.2944962Z     let mut _21: &Test;
2019-08-12T08:36:27.2945044Z     let _22: Test;
2019-08-12T08:36:27.2945105Z     let mut _23: &i32;
2019-08-12T08:36:27.2945184Z     let mut _24: &i32;
2019-08-12T08:36:27.2945248Z     let _25: i32;
2019-08-12T08:36:27.2945328Z     let mut _27: *const i32;
2019-08-12T08:36:27.2945393Z     let mut _28: &i32;
2019-08-12T08:36:27.2945472Z     scope 1 {
2019-08-12T08:36:27.2945533Z         let _3: &mut i32;
2019-08-12T08:36:27.2945903Z         let _14: for<'r> fn(&'r i32) -> &'r i32 as UserTypeProjection { base: UserType(1), projs: [] };
2019-08-12T08:36:27.2945992Z         scope 2 {
2019-08-12T08:36:27.2946070Z             let _8: &mut i32;
2019-08-12T08:36:27.2946136Z             scope 3 {
2019-08-12T08:36:27.2946218Z                 let _10: &mut i32;
2019-08-12T08:36:27.2946294Z                 scope 4 {
2019-08-12T08:36:27.2946384Z                     let _11: *mut i32;
2019-08-12T08:36:27.2946451Z                     scope 5 {
2019-08-12T08:36:27.2946597Z                 }
2019-08-12T08:36:27.2946673Z             }
2019-08-12T08:36:27.2946734Z         }
2019-08-12T08:36:27.2947011Z         scope 6 {
---
2019-08-12T08:36:27.2948583Z                 }
2019-08-12T08:36:27.2948652Z             }
2019-08-12T08:36:27.2948705Z         }
2019-08-12T08:36:27.2948953Z     }
2019-08-12T08:36:27.2950924Z     bb0: {
2019-08-12T08:36:27.2951005Z         StorageLive(_1);
2019-08-12T08:36:27.2951068Z         _1 = const 0i32;
2019-08-12T08:36:27.2951146Z         StorageLive(_2);
2019-08-12T08:36:27.2951221Z         StorageLive(_3);
2019-08-12T08:36:27.2951298Z         StorageLive(_4);
2019-08-12T08:36:27.2951367Z         StorageLive(_5);
2019-08-12T08:36:27.2951444Z         _5 = Test(const 0i32,);
2019-08-12T08:36:27.2951699Z         _4 = &_5;
2019-08-12T08:36:27.2951932Z         Retag(_4);
2019-08-12T08:36:27.2951994Z         StorageLive(_6);
2019-08-12T08:36:27.2952072Z         StorageLive(_7);
2019-08-12T08:36:27.2952132Z         _7 = &mut _1;
2019-08-12T08:36:27.2952207Z         Retag(_7);
2019-08-12T08:36:27.2952267Z         _6 = &mut (*_7);
2019-08-12T08:36:27.2952568Z         Retag([2phase] _6);
2019-08-12T08:36:27.2952936Z         _3 = const Test::foo(move _4, move _6) -> bb1;
2019-08-12T08:36:27.2953304Z     bb1: {
2019-08-12T08:36:27.2953413Z         Retag(_3);
2019-08-12T08:36:27.2953475Z         StorageDead(_6);
2019-08-12T08:36:27.2953848Z         StorageDead(_4);
2019-08-12T08:36:27.2953848Z         StorageDead(_4);
2019-08-12T08:36:27.2953915Z         StorageDead(_7);
2019-08-12T08:36:27.2954781Z         drop(_5) -> bb2;
2019-08-12T08:36:27.2955088Z     bb2: {
2019-08-12T08:36:27.2955146Z         StorageDead(_5);
2019-08-12T08:36:27.2955226Z         StorageLive(_8);
2019-08-12T08:36:27.2955289Z         StorageLive(_9);
2019-08-12T08:36:27.2955289Z         StorageLive(_9);
2019-08-12T08:36:27.2955368Z         _9 = move _3;
2019-08-12T08:36:27.2955430Z         Retag(_9);
2019-08-12T08:36:27.2955508Z         _8 = &mut (*_9);
2019-08-12T08:36:27.2955570Z         Retag(_8);
2019-08-12T08:36:27.2955647Z         StorageDead(_9);
2019-08-12T08:36:27.2955712Z         StorageLive(_10);
2019-08-12T08:36:27.2955796Z         _10 = move _8;
2019-08-12T08:36:27.2955859Z         Retag(_10);
2019-08-12T08:36:27.2956368Z         StorageLive(_11);
2019-08-12T08:36:27.2956447Z         StorageLive(_12);
2019-08-12T08:36:27.2956528Z         StorageLive(_13);
2019-08-12T08:36:27.2956593Z         _13 = &mut (*_10);
2019-08-12T08:36:27.2956673Z         Retag(_13);
2019-08-12T08:36:27.2956740Z         _12 = move _13 as *mut i32 (Misc);
2019-08-12T08:36:27.2956834Z         Retag([raw] _12);
2019-08-12T08:36:27.2957002Z         StorageDead(_13);
2019-08-12T08:36:27.2957082Z         _11 = _12;
2019-08-12T08:36:27.2957697Z         StorageDead(_12);
2019-08-12T08:36:27.2957789Z         _2 = ();
2019-08-12T08:36:27.2957850Z         StorageDead(_11);
2019-08-12T08:36:27.2958011Z         StorageDead(_10);
2019-08-12T08:36:27.2958075Z         StorageDead(_8);
2019-08-12T08:36:27.2958155Z         StorageDead(_3);
2019-08-12T08:36:27.2958217Z         StorageDead(_2);
2019-08-12T08:36:27.2958297Z         StorageLive(_14);
2019-08-12T08:36:27.2958361Z         StorageLive(_15);
2019-08-12T08:36:27.2958452Z         _15 = [closure@HirId { owner: DefIndex(22), local_id: 72 }];
2019-08-12T08:36:27.2958527Z         Retag(_15);
2019-08-12T08:36:27.2959383Z         _14 = move _15 as for<'r> fn(&'r i32) -> &'r i32 (Pointer(ClosureFnPointer(Normal)));
2019-08-12T08:36:27.2959472Z         StorageDead(_15);
2019-08-12T08:36:27.2959720Z         StorageLive(_16);
2019-08-12T08:36:27.2959799Z         StorageLive(_17);
2019-08-12T08:36:27.2959878Z         _17 = _14;
2019-08-12T08:36:27.2959948Z         StorageLive(_18);
2019-08-12T08:36:27.2960026Z         StorageLive(_19);
2019-08-12T08:36:27.2960089Z         _19 = &_1;
2019-08-12T08:36:27.2960166Z         Retag(_19);
2019-08-12T08:36:27.2960230Z         _18 = &(*_19);
2019-08-12T08:36:27.2960308Z         Retag(_18);
2019-08-12T08:36:27.2960569Z         _16 = move _17(move _18) -> bb3;
2019-08-12T08:36:27.2960715Z     bb3: {
2019-08-12T08:36:27.2960788Z         Retag(_16);
2019-08-12T08:36:27.2960849Z         StorageDead(_18);
2019-08-12T08:36:27.2960929Z         StorageDead(_17);
2019-08-12T08:36:27.2960929Z         StorageDead(_17);
2019-08-12T08:36:27.2960993Z         StorageDead(_19);
2019-08-12T08:36:27.2961073Z         StorageLive(_20);
2019-08-12T08:36:27.2961136Z         StorageLive(_21);
2019-08-12T08:36:27.2961215Z         StorageLive(_22);
2019-08-12T08:36:27.2961280Z         _22 = Test(const 0i32,);
2019-08-12T08:36:27.2961361Z         _21 = &_22;
2019-08-12T08:36:27.2961431Z         Retag(_21);
2019-08-12T08:36:27.2961546Z         StorageLive(_23);
2019-08-12T08:36:27.2961610Z         StorageLive(_24);
2019-08-12T08:36:27.2961923Z         _24 = &(promoted[0]: i32);
2019-08-12T08:36:27.2961988Z         Retag(_24);
2019-08-12T08:36:27.2962243Z         _23 = &(*_24);
2019-08-12T08:36:27.2962480Z         Retag(_23);
2019-08-12T08:36:27.2962958Z         _20 = const Test::foo_shr(move _21, move _23) -> bb4;
2019-08-12T08:36:27.2963916Z     bb4: {
2019-08-12T08:36:27.2964031Z         Retag(_20);
2019-08-12T08:36:27.2964110Z         StorageDead(_23);
2019-08-12T08:36:27.2964173Z         StorageDead(_21);
2019-08-12T08:36:27.2964173Z         StorageDead(_21);
2019-08-12T08:36:27.2964252Z         StorageDead(_24);
2019-08-12T08:36:27.2964521Z         drop(_22) -> bb5;
2019-08-12T08:36:27.2964657Z     bb5: {
2019-08-12T08:36:27.2964731Z         StorageDead(_22);
2019-08-12T08:36:27.2964794Z         StorageDead(_20);
2019-08-12T08:36:27.2964992Z         StorageLive(_26);
2019-08-12T08:36:27.2964992Z         StorageLive(_26);
2019-08-12T08:36:27.2965064Z         StorageLive(_27);
2019-08-12T08:36:27.2965153Z         StorageLive(_28);
2019-08-12T08:36:27.2965216Z         _28 = &(*_16);
2019-08-12T08:36:27.2965296Z         Retag(_28);
2019-08-12T08:36:27.2965363Z         _27 = move _28 as *const i32 (Misc);
2019-08-12T08:36:27.2965448Z         Retag([raw] _27);
2019-08-12T08:36:27.2965512Z         StorageDead(_28);
2019-08-12T08:36:27.2965592Z         _26 = _27;
2019-08-12T08:36:27.2965654Z         StorageDead(_27);
2019-08-12T08:36:27.2965733Z         _0 = ();
2019-08-12T08:36:27.2965795Z         StorageDead(_26);
2019-08-12T08:36:27.2965873Z         StorageDead(_16);
2019-08-12T08:36:27.2965936Z         StorageDead(_14);
2019-08-12T08:36:27.2966016Z         StorageDead(_1);
2019-08-12T08:36:27.2966157Z     }
2019-08-12T08:36:27.2966447Z }', src/tools/compiletest/src/runtest.rs:3212:13
2019-08-12T08:36:27.2966561Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-12T08:36:27.2966618Z 
---
2019-08-12T08:36:27.2967682Z test result: FAILED. 48 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out
2019-08-12T08:36:27.2967914Z 
2019-08-12T08:36:27.2968117Z 
2019-08-12T08:36:27.2968150Z 
2019-08-12T08:36:27.2970633Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "mir-opt" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-12T08:36:27.2971241Z 
2019-08-12T08:36:27.2971275Z 
2019-08-12T08:36:27.2971698Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-08-12T08:36:27.2971975Z Build completed unsuccessfully in 1:25:28
2019-08-12T08:36:27.2971975Z Build completed unsuccessfully in 1:25:28
2019-08-12T08:36:27.2972262Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-12T08:36:30.8269375Z ##[error]Bash exited with code '1'.
2019-08-12T08:36:30.8317496Z ##[section]Starting: Upload CPU usage statistics
2019-08-12T08:36:30.8331477Z ==============================================================================
2019-08-12T08:36:30.8331593Z Task         : Bash
2019-08-12T08:36:30.8331682Z Description  : Run a Bash script on macOS, Linux, or Windows
