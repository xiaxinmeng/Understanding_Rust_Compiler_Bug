plain
2019-08-29T18:38:39.3621218Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-29T18:38:39.3841870Z ##[command]git config gc.auto 0
2019-08-29T18:38:39.3922094Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-29T18:38:39.3974844Z ##[command]git config --get-all http.proxy
2019-08-29T18:38:39.4136502Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63531/merge:refs/remotes/pull/63531/merge
---
2019-08-29T19:42:26.7865769Z .................................................................................................... 1500/8975
2019-08-29T19:42:32.4941824Z .................................................................................................... 1600/8975
2019-08-29T19:42:45.2380995Z ................................................i...............i................................... 1700/8975
2019-08-29T19:42:53.5483879Z .................................................................................................... 1800/8975
2019-08-29T19:43:07.9502942Z .......................................iiiii........................................................ 1900/8975
2019-08-29T19:43:18.7076168Z .................................................................................................... 2100/8975
2019-08-29T19:43:21.3042952Z .................................................................................................... 2200/8975
2019-08-29T19:43:25.3761380Z .................................................................................................... 2300/8975
2019-08-29T19:43:32.9627540Z .................................................................................................... 2400/8975
---
2019-08-29T19:46:33.0267296Z ..........................i...............i......................................................... 4700/8975
2019-08-29T19:46:45.0344792Z .................................................................................................... 4800/8975
2019-08-29T19:46:51.2350479Z .................................................................................................... 4900/8975
2019-08-29T19:47:02.1203296Z .................................................................................................... 5000/8975
2019-08-29T19:47:07.6843283Z .......ii.ii........................................................................................ 5100/8975
2019-08-29T19:47:20.8612898Z .................................................................................................... 5300/8975
2019-08-29T19:47:29.2280776Z ......................................................................i............................. 5400/8975
2019-08-29T19:47:36.5824176Z .................................................................................................... 5500/8975
2019-08-29T19:47:43.5132976Z .................................................................................................... 5600/8975
2019-08-29T19:47:43.5132976Z .................................................................................................... 5600/8975
2019-08-29T19:47:54.0319482Z ................................................................i.i..i..ii...........i.............. 5700/8975
2019-08-29T19:48:20.5303875Z .................................................................................................... 5900/8975
2019-08-29T19:48:25.6849742Z .................................................................................................... 6000/8975
2019-08-29T19:48:25.6849742Z .................................................................................................... 6000/8975
2019-08-29T19:48:31.8737689Z .................................................................i..ii.............................. 6100/8975
2019-08-29T19:49:01.1224197Z .................................................................................................... 6300/8975
2019-08-29T19:49:03.2699697Z ....................i............................................................................... 6400/8975
2019-08-29T19:49:05.5410494Z ............................................................................................i....... 6500/8975
2019-08-29T19:49:08.2295564Z .................................................................................................... 6600/8975
---
2019-08-29T19:53:51.6375803Z  finished in 21.063
2019-08-29T19:53:51.6558353Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-29T19:53:51.8389912Z 
2019-08-29T19:53:51.8390203Z running 151 tests
2019-08-29T19:53:55.1052907Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/151
2019-08-29T19:53:57.1610708Z ..iiii..............i..........Fiii.i......ii......
2019-08-29T19:53:57.1611793Z 
2019-08-29T19:53:57.1612390Z ---- [codegen] codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs stdout ----
2019-08-29T19:53:57.1612667Z 
2019-08-29T19:53:57.1612667Z 
2019-08-29T19:53:57.1613164Z error: verification with 'FileCheck' failed
2019-08-29T19:53:57.1613462Z status: exit code: 1
2019-08-29T19:53:57.1614137Z command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll" "/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs"
2019-08-29T19:53:57.1614924Z ------------------------------------------
2019-08-29T19:53:57.1615181Z 
2019-08-29T19:53:57.1615646Z ------------------------------------------
2019-08-29T19:53:57.1616173Z stderr:
2019-08-29T19:53:57.1616173Z stderr:
2019-08-29T19:53:57.1616645Z ------------------------------------------
2019-08-29T19:53:57.1617263Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:24:12: error: expected string not found in input
2019-08-29T19:53:57.1617853Z  // CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* {{.*}} %{{[0-9]+}}, i8* {{.*}} %3, i64 16, i1 false)
2019-08-29T19:53:57.1618477Z            ^
2019-08-29T19:53:57.1619108Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:7:27: note: scanning from here
2019-08-29T19:53:57.1619426Z define void @build_array_s(<4 x float>* noalias nocapture sret dereferenceable(16), [4 x float]* noalias nocapture dereferenceable(16) %x) unnamed_addr #0 {
2019-08-29T19:53:57.1619670Z                           ^
2019-08-29T19:53:57.1620244Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:14:2: note: possible intended match here
2019-08-29T19:53:57.1620578Z  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %3, i64 16, i32 4, i1 false)
2019-08-29T19:53:57.1621324Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:32:12: error: expected string not found in input
2019-08-29T19:53:57.1621324Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:32:12: error: expected string not found in input
2019-08-29T19:53:57.1621928Z  // CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* {{.*}} %{{[0-9]+}}, i8* {{.*}} %3, i64 16, i1 false)
2019-08-29T19:53:57.1622209Z            ^
2019-08-29T19:53:57.1622786Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:25:27: note: scanning from here
2019-08-29T19:53:57.1623093Z define void @build_array_t(<4 x float>* noalias nocapture sret dereferenceable(16), [4 x float]* noalias nocapture dereferenceable(16) %x) unnamed_addr #0 {
2019-08-29T19:53:57.1623313Z                           ^
2019-08-29T19:53:57.1623918Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:32:2: note: possible intended match here
2019-08-29T19:53:57.1624229Z  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %3, i64 16, i32 4, i1 false)
2019-08-29T19:53:57.1625219Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:40:12: error: expected string not found in input
2019-08-29T19:53:57.1625219Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:40:12: error: expected string not found in input
2019-08-29T19:53:57.1625806Z  // CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* {{.*}} %{{[0-9]+}}, i8* {{.*}} %3, i64 16, i1 false)
2019-08-29T19:53:57.1626882Z            ^
2019-08-29T19:53:57.1627554Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:43:27: note: scanning from here
2019-08-29T19:53:57.1627781Z define void @build_array_u(<4 x float>* noalias nocapture sret dereferenceable(16), [4 x float]* noalias nocapture dereferenceable(16) %x) unnamed_addr #0 {
2019-08-29T19:53:57.1628224Z                           ^
2019-08-29T19:53:57.1628832Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:50:2: note: possible intended match here
2019-08-29T19:53:57.1629046Z  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %3, i64 16, i32 4, i1 false)
2019-08-29T19:53:57.1629327Z 
2019-08-29T19:53:57.1629692Z ------------------------------------------
2019-08-29T19:53:57.1629869Z 
2019-08-29T19:53:57.1629986Z 
---
2019-08-29T19:53:57.1635534Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-29T19:53:57.1636005Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-29T19:53:57.1643378Z 
2019-08-29T19:53:57.1643448Z 
2019-08-29T19:53:57.1648301Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-29T19:53:57.1683548Z 
2019-08-29T19:53:57.1683678Z 
2019-08-29T19:53:57.1683784Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-29T19:53:57.1683836Z Build completed unsuccessfully in 1:07:50
2019-08-29T19:53:57.1683836Z Build completed unsuccessfully in 1:07:50
2019-08-29T19:53:57.1688993Z == clock drift check ==
2019-08-29T19:53:57.1702872Z   local time: Thu Aug 29 19:53:57 UTC 2019
2019-08-29T19:53:57.4490100Z   network time: Thu, 29 Aug 2019 19:53:57 GMT
2019-08-29T19:53:57.4492869Z == end clock drift check ==
2019-08-29T19:54:00.3484947Z ##[error]Bash exited with code '1'.
2019-08-29T19:54:00.3521889Z ##[section]Starting: Checkout
2019-08-29T19:54:00.3523757Z ==============================================================================
2019-08-29T19:54:00.3524235Z Task         : Get sources
2019-08-29T19:54:00.3524553Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
