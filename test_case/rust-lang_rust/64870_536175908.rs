plain
2019-09-28T10:51:51.8624733Z test [mir-opt] mir-opt/while-storage.rs ... ok
2019-09-28T10:51:51.8624855Z 
2019-09-28T10:51:51.8624936Z failures:
2019-09-28T10:51:51.8624970Z 
2019-09-28T10:51:51.8625262Z ---- [mir-opt] mir-opt/const_prop/read_immutable_static.rs stdout ----
2019-09-28T10:51:51.8625337Z [ERROR compiletest::runtest] None
2019-09-28T10:51:51.8625655Z thread '[mir-opt] mir-opt/const_prop/read_immutable_static.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2019-09-28T10:51:51.8625780Z Current block: None
2019-09-28T10:51:51.8625871Z Actual Line: "        _4 = CheckedAdd(move _2, move _3);"
2019-09-28T10:51:51.8625965Z Expected Line: "     _1 = Add(move _2, move _3);"
2019-09-28T10:51:51.8626043Z Test Name: rustc.main.ConstProp.before.mir
2019-09-28T10:51:51.8626174Z ... (elided)
2019-09-28T10:51:51.8626247Z  bb0: {
2019-09-28T10:51:51.8626296Z ... (elided)
2019-09-28T10:51:51.8626296Z ... (elided)
2019-09-28T10:51:51.8626369Z      _2 = (FOO: u8);
2019-09-28T10:51:51.8626425Z ... (elided)
2019-09-28T10:51:51.8626496Z      _3 = (FOO: u8);
2019-09-28T10:51:51.8626571Z      _1 = Add(move _2, move _3);
2019-09-28T10:51:51.8626882Z ... (elided)
2019-09-28T10:51:51.8627014Z Actual:
2019-09-28T10:51:51.8627280Z fn  main() -> () {
2019-09-28T10:51:51.8627341Z     let mut _0: ();
2019-09-28T10:51:51.8627415Z     let _1: u8;
2019-09-28T10:51:51.8627415Z     let _1: u8;
2019-09-28T10:51:51.8627469Z     let mut _2: u8;
2019-09-28T10:51:51.8627758Z     let mut _3: u8;
2019-09-28T10:51:51.8627815Z     let mut _4: (u8, bool);
2019-09-28T10:51:51.8627891Z     scope 1 {
2019-09-28T10:51:51.8628013Z     bb0: {
2019-09-28T10:51:51.8628084Z         StorageLive(_1);
2019-09-28T10:51:51.8628229Z         StorageLive(_2);
2019-09-28T10:51:51.8628229Z         StorageLive(_2);
2019-09-28T10:51:51.8628313Z         _2 = (FOO: u8);
2019-09-28T10:51:51.8628369Z         StorageLive(_3);
2019-09-28T10:51:51.8628444Z         _3 = (FOO: u8);
2019-09-28T10:51:51.8628503Z         _4 = CheckedAdd(move _2, move _3);
2019-09-28T10:51:51.8628823Z         assert(!move (_4.1: bool), "attempt to add with overflow") -> bb1;
2019-09-28T10:51:51.8628967Z     bb1: {
2019-09-28T10:51:51.8628967Z     bb1: {
2019-09-28T10:51:51.8629037Z         _1 = move (_4.0: u8);
2019-09-28T10:51:51.8629096Z         StorageDead(_3);
2019-09-28T10:51:51.8629170Z         StorageDead(_2);
2019-09-28T10:51:51.8629226Z         _0 = ();
2019-09-28T10:51:51.8629524Z         StorageDead(_1);
2019-09-28T10:51:51.8629659Z     }
2019-09-28T10:51:51.8629659Z     }
2019-09-28T10:51:51.8629709Z     bb2 (cleanup): {
2019-09-28T10:51:51.8629781Z         resume;
2019-09-28T10:51:51.8631030Z }', src/tools/compiletest/src/runtest.rs:3282:13
2019-09-28T10:51:51.8631169Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-28T10:51:51.8631236Z 
2019-09-28T10:51:51.8631273Z 
2019-09-28T10:51:51.8631273Z 
2019-09-28T10:51:51.8631353Z failures:
2019-09-28T10:51:51.8631668Z     [mir-opt] mir-opt/const_prop/read_immutable_static.rs
2019-09-28T10:51:51.8632057Z test result: FAILED. 56 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-28T10:51:51.8632125Z 
2019-09-28T10:51:51.8632183Z 
2019-09-28T10:51:51.8632219Z 
2019-09-28T10:51:51.8632219Z 
2019-09-28T10:51:51.8634285Z command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-28T10:51:51.8635244Z 
2019-09-28T10:51:51.8635278Z 
2019-09-28T10:51:51.8635571Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-28T10:51:51.8637826Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-28T10:51:51.8637826Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-28T10:51:51.8637939Z Build completed unsuccessfully in 1:25:14
2019-09-28T10:51:51.8687360Z == clock drift check ==
2019-09-28T10:51:51.8705776Z   local time: Sat Sep 28 10:51:51 UTC 2019
2019-09-28T10:51:52.1406634Z   network time: Sat, 28 Sep 2019 10:51:52 GMT
2019-09-28T10:51:52.1411523Z == end clock drift check ==
2019-09-28T10:51:55.9695338Z ##[error]Bash exited with code '1'.
2019-09-28T10:51:55.9737896Z ##[section]Starting: Upload CPU usage statistics
2019-09-28T10:51:55.9745279Z ==============================================================================
2019-09-28T10:51:55.9745566Z Task         : Bash
2019-09-28T10:51:55.9745643Z Description  : Run a Bash script on macOS, Linux, or Windows
