plain
2019-08-15T14:19:50.0103953Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-15T14:19:50.0265188Z ##[command]git config gc.auto 0
2019-08-15T14:19:50.0333830Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-15T14:19:50.0396468Z ##[command]git config --get-all http.proxy
2019-08-15T14:19:50.0522615Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63531/merge:refs/remotes/pull/63531/merge
---
2019-08-15T14:20:25.0184542Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-15T14:20:25.0184570Z 
2019-08-15T14:20:25.0184740Z   git checkout -b <new-branch-name>
2019-08-15T14:20:25.0184782Z 
2019-08-15T14:20:25.0202305Z HEAD is now at d297e83fb Merge 173aafa8279a41f9b0f6f374b4705e5f446aeecd into 1cdcea920e56a5d0587307a4c9cf8fff5c77c4bc
2019-08-15T14:20:25.0357657Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-15T14:20:25.0360544Z ==============================================================================
2019-08-15T14:20:25.0360596Z Task         : Bash
2019-08-15T14:20:25.0360636Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-15T15:20:08.0541905Z .................................................................................................... 1400/8921
2019-08-15T15:20:14.1510380Z .................................................................................................... 1500/8921
2019-08-15T15:20:23.0230997Z .............................................................................................i...... 1600/8921
2019-08-15T15:20:30.7939615Z .........i.......................................................................................... 1700/8921
2019-08-15T15:20:37.4158931Z ....................................................................................iiiii........... 1800/8921
2019-08-15T15:20:57.9613673Z .................................................................................................... 2000/8921
2019-08-15T15:21:00.1771144Z .................................................................................................... 2100/8921
2019-08-15T15:21:02.5213861Z .................................................................................................... 2200/8921
2019-08-15T15:21:09.2733048Z .................................................................................................... 2300/8921
---
2019-08-15T15:23:57.2637471Z ..........................................................................................i......... 4600/8921
2019-08-15T15:24:04.6259339Z ......i............................................................................................. 4700/8921
2019-08-15T15:24:14.4217164Z .................................................................................................... 4800/8921
2019-08-15T15:24:20.3302078Z .................................................................................................... 4900/8921
2019-08-15T15:24:30.6519938Z ......................................................................ii.ii......................... 5000/8921
2019-08-15T15:24:39.6055097Z .................................................................................................... 5200/8921
2019-08-15T15:24:48.5841838Z .................................................................................................... 5300/8921
2019-08-15T15:24:55.8039775Z ...........................i........................................................................ 5400/8921
2019-08-15T15:25:01.1935381Z .................................................................................................... 5500/8921
2019-08-15T15:25:01.1935381Z .................................................................................................... 5500/8921
2019-08-15T15:25:12.5476638Z .................................................................................................... 5600/8921
2019-08-15T15:25:24.8538560Z ......................ii...i..ii...........i........................................................ 5700/8921
2019-08-15T15:25:41.0011216Z .................................................................................................... 5900/8921
2019-08-15T15:25:45.0322306Z .................................................................................................... 6000/8921
2019-08-15T15:25:45.0322306Z .................................................................................................... 6000/8921
2019-08-15T15:25:58.3241895Z .......................i..ii........................................................................ 6100/8921
2019-08-15T15:26:16.6933310Z ..................................................................i................................. 6300/8921
2019-08-15T15:26:18.5482997Z .................................................................................................... 6400/8921
2019-08-15T15:26:20.7414911Z ......................................i............................................................. 6500/8921
2019-08-15T15:26:24.3162206Z .................................................................................................... 6600/8921
---
2019-08-15T15:27:15.3673392Z .................................................................................................... 7100/8921
2019-08-15T15:27:20.5419466Z .................................................................................................... 7200/8921
2019-08-15T15:27:28.5799547Z .................................................................................................... 7300/8921
2019-08-15T15:27:39.1247548Z .................................................................................................... 7400/8921
2019-08-15T15:27:48.9484694Z .........................................................................................ii......i.. 7500/8921
2019-08-15T15:28:01.7300073Z .................................................................................................... 7700/8921
2019-08-15T15:28:18.5489095Z .................................................................................................... 7800/8921
2019-08-15T15:28:26.2228140Z .................................................................................................... 7900/8921
2019-08-15T15:28:37.3535619Z .................................................................................................... 8000/8921
---
2019-08-15T15:30:56.5248178Z  finished in 20.028
2019-08-15T15:30:56.5424596Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-15T15:30:56.7138409Z 
2019-08-15T15:30:56.7138884Z running 150 tests
2019-08-15T15:31:00.0434034Z i....iii......iii..iiii....i............................i..i..................i....i.........ii.i.i. 100/150
2019-08-15T15:31:01.9726746Z .iiii..............i..........Fiii.i......ii......
2019-08-15T15:31:01.9727188Z 
2019-08-15T15:31:01.9727597Z ---- [codegen] codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs stdout ----
2019-08-15T15:31:01.9727799Z 
2019-08-15T15:31:01.9727799Z 
2019-08-15T15:31:01.9728052Z error: verification with 'FileCheck' failed
2019-08-15T15:31:01.9728116Z status: exit code: 1
2019-08-15T15:31:01.9728509Z command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll" "/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs"
2019-08-15T15:31:01.9728773Z ------------------------------------------
2019-08-15T15:31:01.9728800Z 
2019-08-15T15:31:01.9728975Z ------------------------------------------
2019-08-15T15:31:01.9729029Z stderr:
2019-08-15T15:31:01.9729029Z stderr:
2019-08-15T15:31:01.9729206Z ------------------------------------------
2019-08-15T15:31:01.9729461Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:24:12: error: expected string not found in input
2019-08-15T15:31:01.9729909Z  // CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %{{[0-9]+}}, i8* align 4 %3, i64 16, i1 false)
2019-08-15T15:31:01.9730125Z            ^
2019-08-15T15:31:01.9731111Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:7:27: note: scanning from here
2019-08-15T15:31:01.9731212Z define void @build_array_s(<4 x float>* noalias nocapture sret dereferenceable(16), [4 x float]* noalias nocapture dereferenceable(16) %x) unnamed_addr #0 {
2019-08-15T15:31:01.9731267Z                           ^
2019-08-15T15:31:01.9731634Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:14:2: note: possible intended match here
2019-08-15T15:31:01.9731717Z  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %3, i64 16, i32 4, i1 false)
2019-08-15T15:31:01.9732066Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:32:12: error: expected string not found in input
2019-08-15T15:31:01.9732066Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:32:12: error: expected string not found in input
2019-08-15T15:31:01.9732389Z  // CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %{{[0-9]+}}, i8* align 4 %3, i64 16, i1 false)
2019-08-15T15:31:01.9732443Z            ^
2019-08-15T15:31:01.9732787Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:25:27: note: scanning from here
2019-08-15T15:31:01.9732879Z define void @build_array_t(<4 x float>* noalias nocapture sret dereferenceable(16), [4 x float]* noalias nocapture dereferenceable(16) %x) unnamed_addr #0 {
2019-08-15T15:31:01.9732933Z                           ^
2019-08-15T15:31:01.9733285Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:32:2: note: possible intended match here
2019-08-15T15:31:01.9733369Z  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %3, i64 16, i32 4, i1 false)
2019-08-15T15:31:01.9733720Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:40:12: error: expected string not found in input
2019-08-15T15:31:01.9733720Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:40:12: error: expected string not found in input
2019-08-15T15:31:01.9734165Z  // CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %{{[0-9]+}}, i8* align 4 %3, i64 16, i1 false)
2019-08-15T15:31:01.9734306Z            ^
2019-08-15T15:31:01.9734629Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:43:27: note: scanning from here
2019-08-15T15:31:01.9734709Z define void @build_array_u(<4 x float>* noalias nocapture sret dereferenceable(16), [4 x float]* noalias nocapture dereferenceable(16) %x) unnamed_addr #0 {
2019-08-15T15:31:01.9734754Z                           ^
2019-08-15T15:31:01.9735112Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:50:2: note: possible intended match here
2019-08-15T15:31:01.9735177Z  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %3, i64 16, i32 4, i1 false)
2019-08-15T15:31:01.9735239Z 
2019-08-15T15:31:01.9735442Z ------------------------------------------
2019-08-15T15:31:01.9735473Z 
2019-08-15T15:31:01.9735495Z 
---
2019-08-15T15:31:01.9738912Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-15T15:31:01.9739035Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-15T15:31:01.9739073Z 
2019-08-15T15:31:01.9739128Z 
2019-08-15T15:31:01.9740939Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-15T15:31:01.9741378Z 
2019-08-15T15:31:01.9741415Z 
2019-08-15T15:31:01.9748692Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-15T15:31:01.9748969Z Build completed unsuccessfully in 1:04:35
2019-08-15T15:31:01.9748969Z Build completed unsuccessfully in 1:04:35
2019-08-15T15:31:01.9802791Z == clock drift check ==
2019-08-15T15:31:01.9817008Z   local time: Thu Aug 15 15:31:01 UTC 2019
2019-08-15T15:31:02.0751462Z   network time: Thu, 15 Aug 2019 15:31:02 GMT
2019-08-15T15:31:02.0753715Z == end clock drift check ==
2019-08-15T15:31:05.2200320Z ##[error]Bash exited with code '1'.
2019-08-15T15:31:05.2245065Z ##[section]Starting: Checkout
2019-08-15T15:31:05.2246540Z ==============================================================================
2019-08-15T15:31:05.2246598Z Task         : Get sources
2019-08-15T15:31:05.2246653Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
