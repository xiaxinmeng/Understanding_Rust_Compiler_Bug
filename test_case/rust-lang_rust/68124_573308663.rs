plain
2020-01-11T11:30:52.5685671Z failures:
2020-01-11T11:30:52.5691236Z 
2020-01-11T11:30:52.5691934Z ---- [codegen] codegen/consts.rs stdout ----
2020-01-11T11:30:52.5692063Z 
2020-01-11T11:30:52.5692326Z error: verification with 'FileCheck' failed
2020-01-11T11:30:52.5692385Z status: exit code: 1
2020-01-11T11:30:52.5692767Z command: "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/consts/consts.ll" "/checkout/src/test/codegen/consts.rs"
2020-01-11T11:30:52.5693565Z ------------------------------------------
2020-01-11T11:30:52.5695092Z 
2020-01-11T11:30:52.5695428Z ------------------------------------------
2020-01-11T11:30:52.5695668Z stderr:
2020-01-11T11:30:52.5695668Z stderr:
2020-01-11T11:30:52.5696968Z ------------------------------------------
2020-01-11T11:30:52.5697074Z /checkout/src/test/codegen/consts.rs:17:11: error: CHECK: expected string not found in input
2020-01-11T11:30:52.5698556Z // CHECK: [[LOW_HIGH:@[0-9]+]] = {{.*}} getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* @2, i32 0, i32 0, i32 0), {{.*}}, align 8
2020-01-11T11:30:52.5698693Z           ^
2020-01-11T11:30:52.5699037Z /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/consts/consts.ll:12:1: note: scanning from here
2020-01-11T11:30:52.5699163Z @1 = private unnamed_addr constant <{ i8*, [0 x i8] }> <{ i8* getelementptr inbounds (<{ [4 x i8] }>, <{ [4 x i8] }>* @0, i32 0, i32 0, i32 0), [0 x i8] zeroinitializer }>, align 4
2020-01-11T11:30:52.5699260Z ^
2020-01-11T11:30:52.5699516Z /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/consts/consts.ll:14:42: note: possible intended match here
2020-01-11T11:30:52.5699637Z @3 = private unnamed_addr constant <{ i8*, [0 x i8] }> <{ i8* getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* @2, i32 0, i32 0, i32 0), [0 x i8] zeroinitializer }>, align 4
2020-01-11T11:30:52.5699810Z /checkout/src/test/codegen/consts.rs:47:11: error: CHECK: expected string not found in input
2020-01-11T11:30:52.5699810Z /checkout/src/test/codegen/consts.rs:47:11: error: CHECK: expected string not found in input
2020-01-11T11:30:52.5699898Z // CHECK: load %"E<i16, [i16; 3]>"*, %"E<i16, [i16; 3]>"** bitcast (<{ i8*, [0 x i8] }>* [[LOW_HIGH]] to %"E<i16, [i16; 3]>"**), align 8
2020-01-11T11:30:52.5699994Z           ^
2020-01-11T11:30:52.5700260Z /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/consts/consts.ll:38:29: note: scanning from here
2020-01-11T11:30:52.5700349Z define void @low_align_const(%"E<i16, [i16; 3]>"* noalias nocapture sret dereferenceable(8)) unnamed_addr #0 {
2020-01-11T11:30:52.5700433Z                             ^
2020-01-11T11:30:52.5700693Z /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/consts/consts.ll:38:29: note: uses undefined variable(s): "LOW_HIGH"
2020-01-11T11:30:52.5700801Z define void @low_align_const(%"E<i16, [i16; 3]>"* noalias nocapture sret dereferenceable(8)) unnamed_addr #0 {
2020-01-11T11:30:52.5701053Z                             ^
2020-01-11T11:30:52.5701516Z /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/consts/consts.ll:40:8: note: possible intended match here
2020-01-11T11:30:52.5701644Z  %_2 = load %"E<i16, [i16; 3]>"*, %"E<i16, [i16; 3]>"** bitcast (<{ i8*, [0 x i8] }>* @3 to %"E<i16, [i16; 3]>"**), align 4, !nonnull !2
2020-01-11T11:30:52.5701853Z /checkout/src/test/codegen/consts.rs:55:11: error: CHECK: expected string not found in input
2020-01-11T11:30:52.5701853Z /checkout/src/test/codegen/consts.rs:55:11: error: CHECK: expected string not found in input
2020-01-11T11:30:52.5701937Z // CHECK: load %"E<i16, i32>"*, %"E<i16, i32>"** bitcast (<{ i8*, [0 x i8] }>* [[LOW_HIGH]] to %"E<i16, i32>"**), align 8
2020-01-11T11:30:52.5702022Z           ^
2020-01-11T11:30:52.5702315Z /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/consts/consts.ll:48:30: note: scanning from here
2020-01-11T11:30:52.5702403Z define void @high_align_const(%"E<i16, i32>"* noalias nocapture sret dereferenceable(8)) unnamed_addr #0 {
2020-01-11T11:30:52.5702491Z                              ^
2020-01-11T11:30:52.5702799Z /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/consts/consts.ll:48:30: note: uses undefined variable(s): "LOW_HIGH"
2020-01-11T11:30:52.5702910Z define void @high_align_const(%"E<i16, i32>"* noalias nocapture sret dereferenceable(8)) unnamed_addr #0 {
2020-01-11T11:30:52.5702989Z                              ^
2020-01-11T11:30:52.5703281Z /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/consts/consts.ll:50:2: note: possible intended match here
2020-01-11T11:30:52.5703392Z  %_2 = load %"E<i16, i32>"*, %"E<i16, i32>"** bitcast (<{ i8*, [0 x i8] }>* @3 to %"E<i16, i32>"**), align 4, !nonnull !2
2020-01-11T11:30:52.5703495Z 
2020-01-11T11:30:52.5703712Z ------------------------------------------
2020-01-11T11:30:52.5703751Z 
2020-01-11T11:30:52.5703794Z 
---
2020-01-11T11:30:52.5704939Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:386:22
2020-01-11T11:30:52.5705047Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-11T11:30:52.5710013Z 
2020-01-11T11:30:52.5710800Z 
2020-01-11T11:30:52.5714574Z command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-i686-unknown-linux-gnu" "--mode" "codegen" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-11T11:30:52.5715106Z 
2020-01-11T11:30:52.5715137Z 
2020-01-11T11:30:52.5730121Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/bootstrap --exclude src/test/rustdoc-js --exclude src/tools/error_index_generator --exclude src/tools/linkchecker
2020-01-11T11:30:52.5730250Z Build completed unsuccessfully in 1:10:56
2020-01-11T11:30:52.5730250Z Build completed unsuccessfully in 1:10:56
2020-01-11T11:30:52.5773524Z == clock drift check ==
2020-01-11T11:30:52.5788884Z   local time: Sat Jan 11 11:30:52 UTC 2020
2020-01-11T11:30:52.7141537Z   network time: Sat, 11 Jan 2020 11:30:52 GMT
2020-01-11T11:30:52.7144212Z == end clock drift check ==
2020-01-11T11:30:54.8171194Z 
2020-01-11T11:30:54.8258292Z ##[error]Bash exited with code '1'.
2020-01-11T11:30:54.8295595Z ##[section]Starting: Checkout
2020-01-11T11:30:54.8297864Z ==============================================================================
2020-01-11T11:30:54.8297959Z Task         : Get sources
2020-01-11T11:30:54.8298154Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
