plain
2020-03-05T03:42:26.5137563Z test [mir-opt] mir-opt/while-storage.rs ... ok
2020-03-05T03:42:26.5137888Z 
2020-03-05T03:42:26.5138080Z failures:
2020-03-05T03:42:26.5148020Z 
2020-03-05T03:42:26.5148765Z ---- [mir-opt] mir-opt/no-drop-for-inactive-variant.rs stdout ----
2020-03-05T03:42:26.5151034Z thread '[mir-opt] mir-opt/no-drop-for-inactive-variant.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
2020-03-05T03:42:26.5152706Z Expected Line: "        switchInt(move _2) -> [0isize: bb2, 1isize: bb4, otherwise: bb3];"
2020-03-05T03:42:26.5153356Z Test Name: rustc.unwrap.SimplifyCfg-elaborate-drops.after.mir
2020-03-05T03:42:26.5153800Z ... (elided)
2020-03-05T03:42:26.5153800Z ... (elided)
2020-03-05T03:42:26.5154237Z fn unwrap(_1: std::option::Option<T>) -> T {
2020-03-05T03:42:26.5154491Z ... (elided)
2020-03-05T03:42:26.5154679Z     bb0: {
2020-03-05T03:42:26.5154853Z ... (elided)
2020-03-05T03:42:26.5155364Z         switchInt(move _2) -> [0isize: bb2, 1isize: bb4, otherwise: bb3];
2020-03-05T03:42:26.5155668Z     }
2020-03-05T03:42:26.5155865Z     bb1 (cleanup): {
2020-03-05T03:42:26.5156069Z         resume;
2020-03-05T03:42:26.5156409Z     bb2: {
2020-03-05T03:42:26.5156597Z ... (elided)
2020-03-05T03:42:26.5156597Z ... (elided)
2020-03-05T03:42:26.5157116Z         const std::rt::begin_panic::<&'static str>(const "explicit panic") -> bb5;
2020-03-05T03:42:26.5157601Z     bb3: {
2020-03-05T03:42:26.5157790Z         unreachable;
2020-03-05T03:42:26.5157978Z     }
2020-03-05T03:42:26.5158137Z     bb4: {
2020-03-05T03:42:26.5158137Z     bb4: {
2020-03-05T03:42:26.5158320Z ... (elided)
2020-03-05T03:42:26.5158498Z         return;
2020-03-05T03:42:26.5158676Z     }
2020-03-05T03:42:26.5158860Z     bb5 (cleanup): {
2020-03-05T03:42:26.5159238Z         drop(_1) -> bb1;
2020-03-05T03:42:26.5159580Z }
2020-03-05T03:42:26.5159721Z Actual:
2020-03-05T03:42:26.5159721Z Actual:
2020-03-05T03:42:26.5160132Z fn  unwrap(_1: std::option::Option<T>) -> T {
2020-03-05T03:42:26.5160407Z     debug opt => _1;
2020-03-05T03:42:26.5160844Z     let mut _2: isize;
2020-03-05T03:42:26.5161053Z     let _3: T;
2020-03-05T03:42:26.5161271Z     let mut _4: !;
2020-03-05T03:42:26.5161489Z     let mut _5: isize;
2020-03-05T03:42:26.5161489Z     let mut _5: isize;
2020-03-05T03:42:26.5161726Z     let mut _6: isize;
2020-03-05T03:42:26.5162268Z     scope 1 {
2020-03-05T03:42:26.5162479Z         debug x => _3;
2020-03-05T03:42:26.5162655Z     }
2020-03-05T03:42:26.5162826Z     bb0: {
2020-03-05T03:42:26.5163040Z         _2 = discriminant(_1);
2020-03-05T03:42:26.5164387Z         switchInt(move _2) -> [0isize: bb1, 1isize: bb3, otherwise: bb2];
2020-03-05T03:42:26.5164869Z     bb1: {
2020-03-05T03:42:26.5165252Z         StorageLive(_4);
2020-03-05T03:42:26.5165252Z         StorageLive(_4);
2020-03-05T03:42:26.5165788Z         const std::rt::begin_panic::<&'static str>(const "explicit panic");
2020-03-05T03:42:26.5166342Z     bb2: {
2020-03-05T03:42:26.5166543Z         unreachable;
2020-03-05T03:42:26.5166717Z     }
2020-03-05T03:42:26.5166887Z     bb3: {
2020-03-05T03:42:26.5166887Z     bb3: {
2020-03-05T03:42:26.5167079Z         StorageLive(_3);
2020-03-05T03:42:26.5167349Z         _3 = move ((_1 as Some).0: T);
2020-03-05T03:42:26.5167614Z         _0 = move _3;
2020-03-05T03:42:26.5167824Z         StorageDead(_3);
2020-03-05T03:42:26.5168064Z         _5 = discriminant(_1);
2020-03-05T03:42:26.5168452Z     }
2020-03-05T03:42:26.5168865Z }', src/tools/compiletest/src/runtest.rs:3133:13
2020-03-05T03:42:26.5169241Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-05T03:42:26.5169485Z 
---
2020-03-05T03:42:26.5171169Z 
2020-03-05T03:42:26.5171655Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-05T03:42:26.5171938Z 
2020-03-05T03:42:26.5172042Z 
2020-03-05T03:42:26.5175961Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "mir-opt" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-05T03:42:26.5178819Z 
2020-03-05T03:42:26.5178924Z 
2020-03-05T03:42:26.5179735Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2020-03-05T03:42:26.5180318Z Build completed unsuccessfully in 1:22:27
2020-03-05T03:42:26.5180318Z Build completed unsuccessfully in 1:22:27
2020-03-05T03:42:26.5227014Z == clock drift check ==
2020-03-05T03:42:26.5246792Z   local time: Thu Mar  5 03:42:26 UTC 2020
2020-03-05T03:42:27.0674582Z   network time: Thu, 05 Mar 2020 03:42:27 GMT
2020-03-05T03:42:27.0675047Z == end clock drift check ==
2020-03-05T03:42:29.4035683Z 
2020-03-05T03:42:29.4107198Z ##[error]Bash exited with code '1'.
2020-03-05T03:42:29.4188485Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-05T03:42:29.4193699Z ==============================================================================
2020-03-05T03:42:29.4194089Z Task         : Get sources
2020-03-05T03:42:29.4194482Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
