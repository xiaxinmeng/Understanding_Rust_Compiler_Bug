plain
2020-02-03T17:50:09.8978571Z test [ui] ui/optimization-fuel-1.rs ... ok
2020-02-03T17:50:09.9959214Z test [ui] ui/option-ext.rs ... ok
2020-02-03T17:50:10.0755133Z test [ui] ui/or-patterns/already-bound-name.rs ... ok
2020-02-03T17:50:10.1050753Z test [ui] ui/option-unwrap.rs ... ok
2020-02-03T17:50:10.2522256Z test [ui] ui/or-patterns/basic-switch.rs ... ok
2020-02-03T17:50:10.3023647Z test [ui] ui/or-patterns/basic-switchint.rs ... ok
2020-02-03T17:50:10.4182009Z test [ui] ui/or-patterns/bindings-runpass-1.rs ... ok
2020-02-03T17:50:10.4701702Z test [ui] ui/or-patterns/consistent-bindings.rs ... ok
2020-02-03T17:50:10.4721831Z test [ui] ui/or-patterns/bindings-runpass-2.rs ... ok
2020-02-03T17:50:10.5236341Z test [ui] ui/or-patterns/exhaustiveness-non-exhaustive.rs ... ok
2020-02-03T17:50:10.5798582Z test [ui] ui/or-patterns/feature-gate-const-fn.rs ... ok
2020-02-03T17:50:10.5819152Z test [ui] ui/or-patterns/exhaustiveness-unreachable-pattern.rs ... ok
2020-02-03T17:50:10.6168266Z test [ui] ui/or-patterns/feature-gate-or_patterns-leading-for.rs ... ok
---
2020-02-03T17:50:11.3436105Z test [ui] ui/or-patterns/or-pattern-mismatch.rs ... ok
2020-02-03T17:50:11.3824195Z test [ui] ui/or-patterns/or-patterns-syntactic-pass.rs ... ok
2020-02-03T17:50:11.4370475Z test [ui] ui/or-patterns/or-patterns-syntactic-fail.rs ... ok
2020-02-03T17:50:11.4812754Z test [ui] ui/or-patterns/remove-leading-vert.rs ... ok
2020-02-03T17:50:11.6629475Z test [ui] ui/or-patterns/struct-like.rs ... ok
2020-02-03T17:50:11.6854264Z test [ui] ui/or-patterns/search-via-bindings.rs ... ok
2020-02-03T17:50:11.7231910Z test [ui] ui/order-dependent-cast-inference.rs ... ok
2020-02-03T17:50:11.7594346Z test [ui] ui/osx-frameworks.rs ... ok
2020-02-03T17:50:11.7979439Z test [ui] ui/orphan-check-diagnostics.rs ... ok
2020-02-03T17:50:11.8491660Z test [ui] ui/out-of-order-shadowing.rs ... ok
---
2020-02-03T17:53:54.5199048Z test [mir-opt] mir-opt/while-storage.rs ... ok
2020-02-03T17:53:54.5199594Z 
2020-02-03T17:53:54.5199795Z failures:
2020-02-03T17:53:54.5210006Z 
2020-02-03T17:53:54.5210811Z ---- [mir-opt] mir-opt/exponential-or.rs stdout ----
2020-02-03T17:53:54.5211041Z [ERROR compiletest::runtest] Some("bb0: {")
2020-02-03T17:53:54.5211446Z thread '[mir-opt] mir-opt/exponential-or.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-02-03T17:53:54.5211686Z Current block: bb0: {
2020-02-03T17:53:54.5212032Z Actual Line: "        switchInt((_1.0: u32)) -> [1u32: bb3, 4u32: bb3, otherwise: bb2];"
2020-02-03T17:53:54.5212506Z Expected Line: "    switchInt((_1.0: u32)) -> [1u32: bb2, 4u32: bb2, otherwise: bb1];"
2020-02-03T17:53:54.5212954Z Test Name: rustc.match_tuple.SimplifyCfg-initial.after.mir
2020-02-03T17:53:54.5213368Z ... (elided)
2020-02-03T17:53:54.5213436Z scope 1 {
2020-02-03T17:53:54.5213624Z     debug y => _7;
2020-02-03T17:53:54.5213822Z     debug z => _8;
2020-02-03T17:53:54.5213822Z     debug z => _8;
2020-02-03T17:53:54.5213917Z }
2020-02-03T17:53:54.5213996Z bb0: {
2020-02-03T17:53:54.5214168Z     FakeRead(ForMatchedPlace, _1);
2020-02-03T17:53:54.5214665Z     switchInt((_1.0: u32)) -> [1u32: bb2, 4u32: bb2, otherwise: bb1];
2020-02-03T17:53:54.5214945Z bb1: {
2020-02-03T17:53:54.5215081Z     _0 = const 0u32;
2020-02-03T17:53:54.5215329Z     goto -> bb10;
2020-02-03T17:53:54.5215413Z }
2020-02-03T17:53:54.5215413Z }
2020-02-03T17:53:54.5215475Z bb2: {
2020-02-03T17:53:54.5215690Z     _2 = discriminant((_1.2: std::option::Option<i32>));
2020-02-03T17:53:54.5216109Z     switchInt(move _2) -> [0isize: bb4, 1isize: bb3, otherwise: bb1];
2020-02-03T17:53:54.5216513Z bb3: {
2020-02-03T17:53:54.5216513Z bb3: {
2020-02-03T17:53:54.5216901Z     switchInt((((_1.2: std::option::Option<i32>) as Some).0: i32)) -> [1i32: bb4, 8i32: bb4, otherwise: bb1];
2020-02-03T17:53:54.5217227Z bb4: {
2020-02-03T17:53:54.5217227Z bb4: {
2020-02-03T17:53:54.5217339Z     _5 = Le(const 6u32, (_1.3: u32));
2020-02-03T17:53:54.5217630Z     switchInt(move _5) -> [false: bb6, otherwise: bb5];
2020-02-03T17:53:54.5217784Z bb5: {
2020-02-03T17:53:54.5217784Z bb5: {
2020-02-03T17:53:54.5218152Z     _6 = Le((_1.3: u32), const 9u32);
2020-02-03T17:53:54.5218512Z     switchInt(move _6) -> [false: bb6, otherwise: bb8];
2020-02-03T17:53:54.5218673Z bb6: {
2020-02-03T17:53:54.5218673Z bb6: {
2020-02-03T17:53:54.5218879Z     _3 = Le(const 13u32, (_1.3: u32));
2020-02-03T17:53:54.5219288Z     switchInt(move _3) -> [false: bb1, otherwise: bb7];
2020-02-03T17:53:54.5219691Z bb7: {
2020-02-03T17:53:54.5219691Z bb7: {
2020-02-03T17:53:54.5219801Z     _4 = Le((_1.3: u32), const 16u32);
2020-02-03T17:53:54.5220317Z     switchInt(move _4) -> [false: bb1, otherwise: bb8];
2020-02-03T17:53:54.5220717Z bb8: {
2020-02-03T17:53:54.5220717Z bb8: {
2020-02-03T17:53:54.5221015Z     falseEdges -> [real: bb9, imaginary: bb1];
2020-02-03T17:53:54.5221286Z bb9: {
2020-02-03T17:53:54.5221457Z     StorageLive(_7);
2020-02-03T17:53:54.5221457Z     StorageLive(_7);
2020-02-03T17:53:54.5221553Z     _7 = (_1.0: u32);
2020-02-03T17:53:54.5221643Z     StorageLive(_8);
2020-02-03T17:53:54.5221832Z     _8 = (_1.3: u32);
2020-02-03T17:53:54.5222068Z     StorageLive(_9);
2020-02-03T17:53:54.5222178Z     _9 = _7;
2020-02-03T17:53:54.5222270Z     StorageLive(_10);
2020-02-03T17:53:54.5222358Z     StorageLive(_11);
2020-02-03T17:53:54.5222531Z     _11 = _8;
2020-02-03T17:53:54.5222721Z     _10 = Mul(const 2u32, move _11);
2020-02-03T17:53:54.5222815Z     StorageDead(_11);
2020-02-03T17:53:54.5222994Z     _0 = Add(move _9, move _10);
2020-02-03T17:53:54.5223186Z     StorageDead(_10);
2020-02-03T17:53:54.5223287Z     StorageDead(_9);
2020-02-03T17:53:54.5223376Z     StorageDead(_8);
2020-02-03T17:53:54.5223454Z     StorageDead(_7);
2020-02-03T17:53:54.5223717Z     goto -> bb10;
2020-02-03T17:53:54.5224076Z bb10: {
2020-02-03T17:53:54.5224185Z     return;
2020-02-03T17:53:54.5224253Z }
2020-02-03T17:53:54.5224431Z Actual:
2020-02-03T17:53:54.5224431Z Actual:
2020-02-03T17:53:54.5224917Z fn  match_tuple(_1: (u32, bool, std::option::Option<i32>, u32)) -> u32 {
2020-02-03T17:53:54.5225281Z     let mut _0: u32;
2020-02-03T17:53:54.5225380Z     let mut _2: isize;
2020-02-03T17:53:54.5225573Z     let mut _3: bool;
2020-02-03T17:53:54.5225751Z     let mut _4: bool;
2020-02-03T17:53:54.5225751Z     let mut _4: bool;
2020-02-03T17:53:54.5225840Z     let mut _5: bool;
2020-02-03T17:53:54.5226013Z     let mut _6: bool;
2020-02-03T17:53:54.5226138Z     let _7: u32;
2020-02-03T17:53:54.5226207Z     let _8: u32;
2020-02-03T17:53:54.5226398Z     let mut _9: u32;
2020-02-03T17:53:54.5226625Z     let mut _10: u32;
2020-02-03T17:53:54.5226719Z     let mut _11: u32;
2020-02-03T17:53:54.5226807Z     let mut _12: (u32, bool);
2020-02-03T17:53:54.5226998Z     let mut _13: (u32, bool);
2020-02-03T17:53:54.5227302Z         debug y => _7;
2020-02-03T17:53:54.5227414Z         debug z => _8;
2020-02-03T17:53:54.5227521Z     }
2020-02-03T17:53:54.5227586Z     bb0: {
2020-02-03T17:53:54.5227586Z     bb0: {
2020-02-03T17:53:54.5228180Z         FakeRead(ForMatchedPlace, _1);
2020-02-03T17:53:54.5228654Z         switchInt((_1.0: u32)) -> [1u32: bb3, 4u32: bb3, otherwise: bb2];
2020-02-03T17:53:54.5228866Z     }
2020-02-03T17:53:54.5228991Z     bb1 (cleanup): {
2020-02-03T17:53:54.5229084Z         resume;
2020-02-03T17:53:54.5229445Z     bb2: {
2020-02-03T17:53:54.5229553Z         _0 = const 0u32;
2020-02-03T17:53:54.5229830Z         goto -> bb13;
2020-02-03T17:53:54.5230026Z     }
2020-02-03T17:53:54.5230026Z     }
2020-02-03T17:53:54.5230238Z     bb3: {
2020-02-03T17:53:54.5230356Z         _2 = discriminant((_1.2: std::option::Option<i32>));
2020-02-03T17:53:54.5230717Z         switchInt(move _2) -> [0isize: bb5, 1isize: bb4, otherwise: bb2];
2020-02-03T17:53:54.5231101Z     bb4: {
2020-02-03T17:53:54.5231101Z     bb4: {
2020-02-03T17:53:54.5231485Z         switchInt((((_1.2: std::option::Option<i32>) as Some).0: i32)) -> [1i32: bb5, 8i32: bb5, otherwise: bb2];
2020-02-03T17:53:54.5231679Z     bb5: {
2020-02-03T17:53:54.5231679Z     bb5: {
2020-02-03T17:53:54.5231867Z         _5 = Le(const 6u32, (_1.3: u32));
2020-02-03T17:53:54.5232224Z         switchInt(move _5) -> [false: bb7, otherwise: bb6];
2020-02-03T17:53:54.5232620Z     bb6: {
2020-02-03T17:53:54.5232620Z     bb6: {
2020-02-03T17:53:54.5232769Z         _6 = Le((_1.3: u32), const 9u32);
2020-02-03T17:53:54.5233207Z         switchInt(move _6) -> [false: bb7, otherwise: bb9];
2020-02-03T17:53:54.5233542Z     bb7: {
2020-02-03T17:53:54.5233542Z     bb7: {
2020-02-03T17:53:54.5233650Z         _3 = Le(const 13u32, (_1.3: u32));
2020-02-03T17:53:54.5234057Z         switchInt(move _3) -> [false: bb2, otherwise: bb8];
2020-02-03T17:53:54.5234541Z     bb8: {
2020-02-03T17:53:54.5234541Z     bb8: {
2020-02-03T17:53:54.5234795Z         _4 = Le((_1.3: u32), const 16u32);
2020-02-03T17:53:54.5235224Z         switchInt(move _4) -> [false: bb2, otherwise: bb9];
2020-02-03T17:53:54.5235596Z     bb9: {
2020-02-03T17:53:54.5235596Z     bb9: {
2020-02-03T17:53:54.5235874Z         falseEdges -> [real: bb10, imaginary: bb2];
2020-02-03T17:53:54.5236213Z     bb10: {
2020-02-03T17:53:54.5236282Z         StorageLive(_7);
2020-02-03T17:53:54.5236282Z         StorageLive(_7);
2020-02-03T17:53:54.5236474Z         _7 = (_1.0: u32);
2020-02-03T17:53:54.5236703Z         StorageLive(_8);
2020-02-03T17:53:54.5236903Z         _8 = (_1.3: u32);
2020-02-03T17:53:54.5237036Z         StorageLive(_9);
2020-02-03T17:53:54.5237151Z         _9 = _7;
2020-02-03T17:53:54.5237240Z         StorageLive(_10);
2020-02-03T17:53:54.5237418Z         StorageLive(_11);
2020-02-03T17:53:54.5237683Z         _11 = _8;
2020-02-03T17:53:54.5237785Z         _12 = CheckedMul(const 2u32, move _11);
2020-02-03T17:53:54.5238268Z         assert(!move (_12.1: bool), "attempt to multiply with overflow") -> [success: bb11, unwind: bb1];
2020-02-03T17:53:54.5238616Z     bb11: {
2020-02-03T17:53:54.5238616Z     bb11: {
2020-02-03T17:53:54.5238701Z         _10 = move (_12.0: u32);
2020-02-03T17:53:54.5238877Z         StorageDead(_11);
2020-02-03T17:53:54.5239084Z         _13 = CheckedAdd(move _9, move _10);
2020-02-03T17:53:54.5239433Z         assert(!move (_13.1: bool), "attempt to add with overflow") -> [success: bb12, unwind: bb1];
2020-02-03T17:53:54.5239851Z     bb12: {
2020-02-03T17:53:54.5239851Z     bb12: {
2020-02-03T17:53:54.5239947Z         _0 = move (_13.0: u32);
2020-02-03T17:53:54.5240038Z         StorageDead(_10);
2020-02-03T17:53:54.5240215Z         StorageDead(_9);
2020-02-03T17:53:54.5240434Z         StorageDead(_8);
2020-02-03T17:53:54.5240527Z         StorageDead(_7);
2020-02-03T17:53:54.5240817Z         goto -> bb13;
2020-02-03T17:53:54.5241122Z     bb13: {
2020-02-03T17:53:54.5241230Z         return;
2020-02-03T17:53:54.5241299Z     }
2020-02-03T17:53:54.5241703Z }', src/tools/compiletest/src/runtest.rs:3121:13
---
2020-02-03T17:53:54.5243080Z test result: FAILED. 81 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-03T17:53:54.5243274Z 
2020-02-03T17:53:54.5243391Z 
2020-02-03T17:53:54.5243452Z 
2020-02-03T17:53:54.5245554Z command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-03T17:53:54.5246390Z 
2020-02-03T17:53:54.5251228Z 
2020-02-03T17:53:54.5253044Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-03T17:53:54.5257412Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-03T17:53:54.5257412Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-03T17:53:54.5257528Z Build completed unsuccessfully in 1:22:15
2020-02-03T17:53:54.5311177Z == clock drift check ==
2020-02-03T17:53:54.5331315Z   local time: Mon Feb  3 17:53:54 UTC 2020
2020-02-03T17:53:55.0785937Z   network time: Mon, 03 Feb 2020 17:53:55 GMT
2020-02-03T17:53:55.0786136Z == end clock drift check ==
2020-02-03T17:53:57.2336728Z 
2020-02-03T17:53:57.2420833Z ##[error]Bash exited with code '1'.
2020-02-03T17:53:57.2465016Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-03T17:53:57.2466943Z ==============================================================================
2020-02-03T17:53:57.2467049Z Task         : Get sources
2020-02-03T17:53:57.2467143Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
