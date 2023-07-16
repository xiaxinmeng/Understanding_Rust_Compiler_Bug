plain
2019-08-28T12:45:17.7613005Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-28T12:45:17.7822298Z ##[command]git config gc.auto 0
2019-08-28T12:45:17.7896409Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-28T12:45:17.7959600Z ##[command]git config --get-all http.proxy
2019-08-28T12:45:17.8094849Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63531/merge:refs/remotes/pull/63531/merge
---
2019-08-28T12:45:54.4902895Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-28T12:45:54.4902923Z 
2019-08-28T12:45:54.4903143Z   git checkout -b <new-branch-name>
2019-08-28T12:45:54.4903171Z 
2019-08-28T12:45:54.4903217Z HEAD is now at 03cac775b Merge 06037a3ce56aa244f69fe6c69794a7a6951c29ac into ac21131f7859836cd3fcb39231c0162fd892d960
2019-08-28T12:45:54.5106911Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-28T12:45:54.5109606Z ==============================================================================
2019-08-28T12:45:54.5109657Z Task         : Bash
2019-08-28T12:45:54.5109696Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-28T13:51:04.6884048Z .................................................................................................... 1500/8970
2019-08-28T13:51:10.8470094Z .................................................................................................... 1600/8970
2019-08-28T13:51:24.5864661Z .............................................i...............i...................................... 1700/8970
2019-08-28T13:51:33.3349171Z .................................................................................................... 1800/8970
2019-08-28T13:51:48.4708207Z ....................................iiiii........................................................... 1900/8970
2019-08-28T13:52:00.1030756Z .................................................................................................... 2100/8970
2019-08-28T13:52:02.8812049Z .................................................................................................... 2200/8970
2019-08-28T13:52:07.5755611Z .................................................................................................... 2300/8970
2019-08-28T13:52:15.4847553Z .................................................................................................... 2400/8970
---
2019-08-28T13:55:28.7935610Z .......................i...............i............................................................ 4700/8970
2019-08-28T13:55:41.2935072Z .................................................................................................... 4800/8970
2019-08-28T13:55:47.9391559Z .................................................................................................... 4900/8970
2019-08-28T13:55:59.7381425Z .................................................................................................... 5000/8970
2019-08-28T13:56:05.4470515Z ....ii.ii........................................................................................... 5100/8970
2019-08-28T13:56:20.6057870Z .................................................................................................... 5300/8970
2019-08-28T13:56:28.4519273Z ...................................................................i................................ 5400/8970
2019-08-28T13:56:36.4091399Z .................................................................................................... 5500/8970
2019-08-28T13:56:43.9140907Z .................................................................................................... 5600/8970
2019-08-28T13:56:43.9140907Z .................................................................................................... 5600/8970
2019-08-28T13:56:55.0688298Z .............................................................ii...i..ii...........i................. 5700/8970
2019-08-28T13:57:22.0887333Z .................................................................................................... 5900/8970
2019-08-28T13:57:27.4920126Z .................................................................................................... 6000/8970
2019-08-28T13:57:27.4920126Z .................................................................................................... 6000/8970
2019-08-28T13:57:34.8932588Z ..............................................................i..ii................................. 6100/8970
2019-08-28T13:58:05.1602336Z .................................................................................................... 6300/8970
2019-08-28T13:58:07.4650721Z .................i.................................................................................. 6400/8970
2019-08-28T13:58:09.8285471Z .........................................................................................i.......... 6500/8970
2019-08-28T13:58:12.7974559Z .................................................................................................... 6600/8970
---
2019-08-28T14:03:14.8072923Z  finished in 22.299
2019-08-28T14:03:14.8286325Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-28T14:03:15.0044797Z 
2019-08-28T14:03:15.0051305Z running 151 tests
2019-08-28T14:03:18.5356056Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/151
2019-08-28T14:03:20.7994227Z ..iiii..............i..........Fiii.i......ii......
2019-08-28T14:03:20.7994485Z 
2019-08-28T14:03:20.7994821Z ---- [codegen] codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs stdout ----
2019-08-28T14:03:20.7994862Z 
2019-08-28T14:03:20.7994862Z 
2019-08-28T14:03:20.7995132Z error: verification with 'FileCheck' failed
2019-08-28T14:03:20.7995188Z status: exit code: 1
2019-08-28T14:03:20.7995690Z command: "/usr/lib/llvm-6.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll" "/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs"
2019-08-28T14:03:20.7996032Z ------------------------------------------
2019-08-28T14:03:20.7996065Z 
2019-08-28T14:03:20.7996314Z ------------------------------------------
2019-08-28T14:03:20.7996363Z stderr:
2019-08-28T14:03:20.7996363Z stderr:
2019-08-28T14:03:20.7996597Z ------------------------------------------
2019-08-28T14:03:20.7996954Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:24:12: error: expected string not found in input
2019-08-28T14:03:20.7997280Z  // CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %{{[0-9]+}}, i8* align 4 %3, i64 16, i1 false)
2019-08-28T14:03:20.7997336Z            ^
2019-08-28T14:03:20.7997991Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:7:27: note: scanning from here
2019-08-28T14:03:20.7998837Z define void @build_array_s(<4 x float>* noalias nocapture sret dereferenceable(16), [4 x float]* noalias nocapture dereferenceable(16) %x) unnamed_addr #0 {
2019-08-28T14:03:20.7998942Z                           ^
2019-08-28T14:03:20.7999484Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:14:2: note: possible intended match here
2019-08-28T14:03:20.7999557Z  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %3, i64 16, i32 4, i1 false)
2019-08-28T14:03:20.8000147Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:32:12: error: expected string not found in input
2019-08-28T14:03:20.8000147Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:32:12: error: expected string not found in input
2019-08-28T14:03:20.8000468Z  // CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %{{[0-9]+}}, i8* align 4 %3, i64 16, i1 false)
2019-08-28T14:03:20.8000524Z            ^
2019-08-28T14:03:20.8000910Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:25:27: note: scanning from here
2019-08-28T14:03:20.8001000Z define void @build_array_t(<4 x float>* noalias nocapture sret dereferenceable(16), [4 x float]* noalias nocapture dereferenceable(16) %x) unnamed_addr #0 {
2019-08-28T14:03:20.8001060Z                           ^
2019-08-28T14:03:20.8001467Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:32:2: note: possible intended match here
2019-08-28T14:03:20.8001545Z  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %3, i64 16, i32 4, i1 false)
2019-08-28T14:03:20.8001936Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:40:12: error: expected string not found in input
2019-08-28T14:03:20.8001936Z /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:40:12: error: expected string not found in input
2019-08-28T14:03:20.8002256Z  // CHECK: call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %{{[0-9]+}}, i8* align 4 %3, i64 16, i1 false)
2019-08-28T14:03:20.8002328Z            ^
2019-08-28T14:03:20.8002707Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:43:27: note: scanning from here
2019-08-28T14:03:20.8002786Z define void @build_array_u(<4 x float>* noalias nocapture sret dereferenceable(16), [4 x float]* noalias nocapture dereferenceable(16) %x) unnamed_addr #0 {
2019-08-28T14:03:20.8002863Z                           ^
2019-08-28T14:03:20.8003247Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:50:2: note: possible intended match here
2019-08-28T14:03:20.8003325Z  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %3, i64 16, i32 4, i1 false)
2019-08-28T14:03:20.8003423Z 
2019-08-28T14:03:20.8003658Z ------------------------------------------
2019-08-28T14:03:20.8003693Z 
2019-08-28T14:03:20.8003721Z 
---
2019-08-28T14:03:20.8005231Z test result: FAILED. 119 passed; 1 failed; 31 ignored; 0 measured; 0 filtered out
2019-08-28T14:03:20.8005269Z 
2019-08-28T14:03:20.8005298Z 
2019-08-28T14:03:20.8005325Z 
2019-08-28T14:03:20.8007131Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-28T14:03:20.8007496Z 
2019-08-28T14:03:20.8007546Z 
2019-08-28T14:03:20.8007883Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-28T14:03:20.8007950Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-28T14:03:20.8007950Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-28T14:03:20.8044310Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-28T14:03:20.8044423Z Build completed unsuccessfully in 1:10:40
2019-08-28T14:03:20.8087148Z == clock drift check ==
2019-08-28T14:03:20.8103779Z   local time: Wed Aug 28 14:03:20 UTC 2019
2019-08-28T14:03:20.8564993Z   network time: Wed, 28 Aug 2019 14:03:20 GMT
2019-08-28T14:03:20.8574464Z == end clock drift check ==
2019-08-28T14:03:25.1182552Z ##[error]Bash exited with code '1'.
2019-08-28T14:03:25.1228196Z ##[section]Starting: Checkout
2019-08-28T14:03:25.1230619Z ==============================================================================
2019-08-28T14:03:25.1230680Z Task         : Get sources
2019-08-28T14:03:25.1230742Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
