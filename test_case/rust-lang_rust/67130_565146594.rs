plain
2019-12-12T19:19:06.6252450Z test [mir-opt] mir-opt/unusual-item-types.rs ... ok
2019-12-12T19:19:06.6254180Z 
2019-12-12T19:19:06.6254960Z failures:
2019-12-12T19:19:06.6255480Z 
2019-12-12T19:19:06.6256890Z ---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
2019-12-12T19:19:06.6257100Z [ERROR compiletest::runtest] Some("bb3 (cleanup): {")
2019-12-12T19:19:06.6257980Z thread '[mir-opt] mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2019-12-12T19:19:06.6258160Z Current block: bb3 (cleanup): {
2019-12-12T19:19:06.6258250Z Actual Line: "}"
2019-12-12T19:19:06.6258340Z Expected Line: "bb3 (cleanup): {"
2019-12-12T19:19:06.6258430Z Test Name: rustc.main.ConstProp.before.mir
2019-12-12T19:19:06.6258590Z ... (elided)
2019-12-12T19:19:06.6258680Z let mut _0: ();
2019-12-12T19:19:06.6258750Z let _1: i32;
2019-12-12T19:19:06.6258840Z let mut _2: (i32, bool);
2019-12-12T19:19:06.6258840Z let mut _2: (i32, bool);
2019-12-12T19:19:06.6258930Z let mut _4: [i32; 6];
2019-12-12T19:19:06.6259020Z let _5: usize;
2019-12-12T19:19:06.6259120Z let mut _6: usize;
2019-12-12T19:19:06.6259200Z let mut _7: bool;
2019-12-12T19:19:06.6259290Z let mut _9: Point;
2019-12-12T19:19:06.6259450Z   debug x => _1;
2019-12-12T19:19:06.6259520Z   let _3: i32;
2019-12-12T19:19:06.6259610Z   scope 2 {
2019-12-12T19:19:06.6259690Z     debug y => _3;
2019-12-12T19:19:06.6259690Z     debug y => _3;
2019-12-12T19:19:06.6259780Z     let _8: u32;
2019-12-12T19:19:06.6259860Z     scope 3 {
2019-12-12T19:19:06.6259940Z       debug z => _8;
2019-12-12T19:19:06.6260030Z     }
2019-12-12T19:19:06.6260100Z   }
2019-12-12T19:19:06.6260280Z }
2019-12-12T19:19:06.6260350Z bb0: {
2019-12-12T19:19:06.6260440Z   StorageLive(_1);
2019-12-12T19:19:06.6260530Z   _2 = CheckedAdd(const 2i32, const 2i32);
2019-12-12T19:19:06.6261310Z   assert(!move (_2.1: bool), "attempt to add with overflow") -> bb1;
2019-12-12T19:19:06.6261520Z bb1: {
2019-12-12T19:19:06.6261520Z bb1: {
2019-12-12T19:19:06.6261650Z   _1 = move (_2.0: i32);
2019-12-12T19:19:06.6261790Z   StorageLive(_3);
2019-12-12T19:19:06.6261950Z   StorageLive(_4);
2019-12-12T19:19:06.6262120Z   _4 = [const 0i32, const 1i32, const 2i32, const 3i32, const 4i32, const 5i32];
2019-12-12T19:19:06.6262290Z   StorageLive(_5);
2019-12-12T19:19:06.6262570Z   _6 = const 6usize;
2019-12-12T19:19:06.6262570Z   _6 = const 6usize;
2019-12-12T19:19:06.6262730Z   _7 = Lt(_5, _6);
2019-12-12T19:19:06.6263760Z   assert(move _7, "index out of bounds: the len is move _6 but the index is _5") -> bb2;
2019-12-12T19:19:06.6264100Z bb2: {
2019-12-12T19:19:06.6264100Z bb2: {
2019-12-12T19:19:06.6264230Z   _3 = _4[_5];
2019-12-12T19:19:06.6264360Z   StorageDead(_5);
2019-12-12T19:19:06.6264510Z   StorageDead(_4);
2019-12-12T19:19:06.6264640Z   StorageLive(_8);
2019-12-12T19:19:06.6264780Z   StorageLive(_9);
2019-12-12T19:19:06.6264940Z   _9 = Point { x: const 12u32, y: const 42u32 };
2019-12-12T19:19:06.6265090Z   _8 = (_9.1: u32);
2019-12-12T19:19:06.6265230Z   StorageDead(_9);
2019-12-12T19:19:06.6265500Z   StorageDead(_8);
2019-12-12T19:19:06.6266100Z   StorageDead(_3);
2019-12-12T19:19:06.6266290Z   StorageDead(_1);
2019-12-12T19:19:06.6266420Z   return;
2019-12-12T19:19:06.6266420Z   return;
2019-12-12T19:19:06.6266550Z }
2019-12-12T19:19:06.6266670Z bb3 (cleanup): {
2019-12-12T19:19:06.6266810Z   resume;
2019-12-12T19:19:06.6267070Z Actual:
2019-12-12T19:19:06.6268270Z fn  main() -> () {
2019-12-12T19:19:06.6268430Z     let mut _0: ();
2019-12-12T19:19:06.6268580Z     let _1: i32;
---
2019-12-12T19:19:06.6270640Z                 debug z => _8;
2019-12-12T19:19:06.6270790Z             }
2019-12-12T19:19:06.6270920Z         }
2019-12-12T19:19:06.6271050Z     }
2019-12-12T19:19:06.6271180Z     bb0: {
2019-12-12T19:19:06.6271320Z         StorageLive(_1);
2019-12-12T19:19:06.6271480Z         _2 = CheckedAdd(const 2i32, const 2i32);
2019-12-12T19:19:06.6272460Z         assert(!move (_2.1: bool), "attempt to add with overflow") -> bb1;
2019-12-12T19:19:06.6272800Z     bb1: {
2019-12-12T19:19:06.6272800Z     bb1: {
2019-12-12T19:19:06.6272940Z         _1 = move (_2.0: i32);
2019-12-12T19:19:06.6273070Z         StorageLive(_3);
2019-12-12T19:19:06.6273220Z         StorageLive(_4);
2019-12-12T19:19:06.6273390Z         _4 = [const 0i32, const 1i32, const 2i32, const 3i32, const 4i32, const 5i32];
2019-12-12T19:19:06.6273590Z         StorageLive(_5);
2019-12-12T19:19:06.6273900Z         _6 = const 6usize;
2019-12-12T19:19:06.6273900Z         _6 = const 6usize;
2019-12-12T19:19:06.6274050Z         _7 = Lt(_5, _6);
2019-12-12T19:19:06.6275060Z         assert(move _7, "index out of bounds: the len is move _6 but the index is _5") -> bb2;
2019-12-12T19:19:06.6275400Z     bb2: {
2019-12-12T19:19:06.6275400Z     bb2: {
2019-12-12T19:19:06.6275550Z         _3 = _4[_5];
2019-12-12T19:19:06.6275690Z         StorageDead(_5);
2019-12-12T19:19:06.6275830Z         StorageDead(_4);
2019-12-12T19:19:06.6275970Z         StorageLive(_8);
2019-12-12T19:19:06.6276120Z         StorageLive(_9);
2019-12-12T19:19:06.6276280Z         _9 = Point { x: const 12u32, y: const 42u32 };
2019-12-12T19:19:06.6276430Z         _8 = (_9.1: u32);
2019-12-12T19:19:06.6276590Z         StorageDead(_9);
2019-12-12T19:19:06.6276870Z         StorageDead(_8);
2019-12-12T19:19:06.6277000Z         StorageDead(_3);
2019-12-12T19:19:06.6277150Z         StorageDead(_1);
2019-12-12T19:19:06.6277280Z         return;
2019-12-12T19:19:06.6277280Z         return;
2019-12-12T19:19:06.6277430Z     }
2019-12-12T19:19:06.6278380Z }', src/tools/compiletest/src/runtest.rs:3319:13
2019-12-12T19:19:06.6278580Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-12T19:19:06.6278710Z 
2019-12-12T19:19:06.6278800Z 
2019-12-12T19:19:06.6278940Z failures:
2019-12-12T19:19:06.6279850Z     [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs
2019-12-12T19:19:06.6281000Z test result: FAILED. 70 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-12-12T19:19:06.6281150Z 
2019-12-12T19:19:06.6282130Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-12T19:19:06.6282270Z 
2019-12-12T19:19:06.6282270Z 
2019-12-12T19:19:06.6282350Z 
2019-12-12T19:19:06.6286630Z command did not execute successfully: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.163.1/work/1/s/src/test/mir-opt" "--build-base" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/mir-opt" "--stage-id" "stage2-i686-apple-darwin" "--mode" "mir-opt" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-12T19:19:06.6287840Z 
2019-12-12T19:19:06.6287910Z 
2019-12-12T19:19:06.6288040Z failed to run: /Users/runner/runners/2.163.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-12T19:19:06.6288170Z Build completed unsuccessfully in 1:00:45
2019-12-12T19:19:06.6288170Z Build completed unsuccessfully in 1:00:45
2019-12-12T19:19:06.6345160Z == clock drift check ==
2019-12-12T19:19:06.6412430Z   local time: Thu Dec 12 19:19:06 UTC 2019
2019-12-12T19:19:06.7091500Z   network time: Thu, 12 Dec 2019 19:19:06 GMT
2019-12-12T19:19:06.7092860Z == end clock drift check ==
2019-12-12T19:19:06.7141440Z 
2019-12-12T19:19:06.7279880Z ##[error]Bash exited with code '1'.
2019-12-12T19:19:06.7334120Z ##[section]Starting: Checkout
2019-12-12T19:19:06.7337320Z ==============================================================================
2019-12-12T19:19:06.7337460Z Task         : Get sources
2019-12-12T19:19:06.7337570Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
