plain
2019-12-20T11:13:45.0634121Z test [mir-opt] mir-opt/while-storage.rs ... ok
2019-12-20T11:13:45.0634240Z 
2019-12-20T11:13:45.0634300Z failures:
2019-12-20T11:13:45.0634350Z 
2019-12-20T11:13:45.0634619Z ---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
2019-12-20T11:13:45.0634713Z [ERROR compiletest::runtest] None
2019-12-20T11:13:45.0635023Z thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2019-12-20T11:13:45.0635143Z Current block: None
2019-12-20T11:13:45.0635222Z Actual Line: "    scope 1 {"
2019-12-20T11:13:45.0635284Z Expected Line: "let mut _3: ();"
2019-12-20T11:13:45.0635363Z Test Name: rustc.main.Inline.before.mir
2019-12-20T11:13:45.0635684Z ... (elided)
2019-12-20T11:13:45.0635740Z let mut _0: ();
2019-12-20T11:13:45.0635740Z let mut _0: ();
2019-12-20T11:13:45.0635836Z let _1: std::boxed::Box<std::vec::Vec<u32>> as UserTypeProjection { base: UserType(0), projs: [] };
2019-12-20T11:13:45.0635933Z let mut _2: std::boxed::Box<std::vec::Vec<u32>>;
2019-12-20T11:13:45.0636014Z let mut _3: ();
2019-12-20T11:13:45.0636142Z   debug _x => _1;
2019-12-20T11:13:45.0636217Z }
2019-12-20T11:13:45.0636272Z bb0: {
2019-12-20T11:13:45.0636341Z   StorageLive(_1);
2019-12-20T11:13:45.0636341Z   StorageLive(_1);
2019-12-20T11:13:45.0636401Z   StorageLive(_2);
2019-12-20T11:13:45.0636475Z   _2 = Box(std::vec::Vec<u32>);
2019-12-20T11:13:45.0637102Z   (*_2) = const std::vec::Vec::<u32>::new() -> [return: bb2, unwind: bb4];
2019-12-20T11:13:45.0637187Z }
2019-12-20T11:13:45.0637237Z bb1 (cleanup): {
2019-12-20T11:13:45.0637306Z   resume;
2019-12-20T11:13:45.0637420Z bb2: {
2019-12-20T11:13:45.0637469Z   _1 = move _2;
2019-12-20T11:13:45.0637552Z   StorageDead(_2);
2019-12-20T11:13:45.0637606Z   _0 = ();
2019-12-20T11:13:45.0637606Z   _0 = ();
2019-12-20T11:13:45.0637829Z   drop(_1) -> [return: bb3, unwind: bb1];
2019-12-20T11:13:45.0637952Z bb3: {
2019-12-20T11:13:45.0638001Z   StorageDead(_1);
2019-12-20T11:13:45.0638077Z   return;
2019-12-20T11:13:45.0638141Z }
2019-12-20T11:13:45.0638141Z }
2019-12-20T11:13:45.0638191Z bb4 (cleanup): {
2019-12-20T11:13:45.0638498Z   _3 = const alloc::alloc::box_free::<std::vec::Vec<u32>>(move (_2.0: std::ptr::Unique<std::vec::Vec<u32>>)) -> bb1;
2019-12-20T11:13:45.0638645Z Actual:
2019-12-20T11:13:45.0638696Z | User Type Annotations
2019-12-20T11:13:45.0638696Z | User Type Annotations
2019-12-20T11:13:45.0640143Z | 0: Canonical { max_universe: U0, variables: [], value: Ty(std::boxed::Box<std::vec::Vec<u32>>) } at /checkout/src/test/mir-opt/inline/inline-into-box-place.rs:5:13: 5:26
2019-12-20T11:13:45.0640913Z | 1: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(5:3852 ~ alloc[1de6]::vec[0]::{{impl}}[0]::new[0]), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(5:3850 ~ alloc[1de6]::vec[0]::{{impl}}[0]), self_ty: std::vec::Vec<^1> }) }) } at /checkout/src/test/mir-opt/inline/inline-into-box-place.rs:5:33: 5:41
2019-12-20T11:13:45.0641528Z | 2: Canonical { max_universe: U0, variables: [], value: Ty(std::boxed::Box<std::vec::Vec<u32>>) } at /checkout/src/test/mir-opt/inline/inline-into-box-place.rs:5:13: 5:26
2019-12-20T11:13:45.0641885Z fn  main() -> () {
2019-12-20T11:13:45.0641943Z     let mut _0: ();
2019-12-20T11:13:45.0641943Z     let mut _0: ();
2019-12-20T11:13:45.0642033Z     let _1: std::boxed::Box<std::vec::Vec<u32>> as UserTypeProjection { base: UserType(0), projs: [] };
2019-12-20T11:13:45.0642116Z     let mut _2: std::boxed::Box<std::vec::Vec<u32>>;
2019-12-20T11:13:45.0642247Z         debug _x => _1;
2019-12-20T11:13:45.0642317Z     }
2019-12-20T11:13:45.0642367Z     bb0: {
2019-12-20T11:13:45.0642434Z         StorageLive(_1);
2019-12-20T11:13:45.0642434Z         StorageLive(_1);
2019-12-20T11:13:45.0642498Z         StorageLive(_2);
2019-12-20T11:13:45.0642578Z         _2 = Box(std::vec::Vec<u32>);
2019-12-20T11:13:45.0642830Z         (*_2) = const std::vec::Vec::<u32>::new() -> bb1;
2019-12-20T11:13:45.0642966Z     bb1: {
2019-12-20T11:13:45.0643018Z         _1 = move _2;
2019-12-20T11:13:45.0643089Z         StorageDead(_2);
2019-12-20T11:13:45.0643145Z         _0 = ();
2019-12-20T11:13:45.0643145Z         _0 = ();
2019-12-20T11:13:45.0643350Z         drop(_1) -> bb2;
2019-12-20T11:13:45.0643475Z     bb2: {
2019-12-20T11:13:45.0643526Z         StorageDead(_1);
2019-12-20T11:13:45.0643597Z         return;
2019-12-20T11:13:45.0643648Z     }
---
2019-12-20T11:13:45.0644786Z 
2019-12-20T11:13:45.0645056Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-20T11:13:45.0651601Z 
2019-12-20T11:13:45.0652474Z 
2019-12-20T11:13:45.0654682Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "mir-opt" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-20T11:13:45.0655305Z 
2019-12-20T11:13:45.0655356Z 
2019-12-20T11:13:45.0662866Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-12-20T11:13:45.0663200Z Build completed unsuccessfully in 1:13:43
2019-12-20T11:13:45.0663200Z Build completed unsuccessfully in 1:13:43
2019-12-20T11:13:45.0733754Z == clock drift check ==
2019-12-20T11:13:45.0754914Z   local time: Fri Dec 20 11:13:45 UTC 2019
2019-12-20T11:13:45.3511125Z   network time: Fri, 20 Dec 2019 11:13:45 GMT
2019-12-20T11:13:45.3511337Z == end clock drift check ==
2019-12-20T11:13:49.6770684Z 
2019-12-20T11:13:49.6867553Z ##[error]Bash exited with code '1'.
2019-12-20T11:13:49.6904626Z ##[section]Starting: Checkout
2019-12-20T11:13:49.6906768Z ==============================================================================
2019-12-20T11:13:49.6906845Z Task         : Get sources
2019-12-20T11:13:49.6906927Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
