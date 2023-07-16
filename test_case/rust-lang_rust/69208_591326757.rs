plain
2020-02-26T09:24:51.0663124Z failures:
2020-02-26T09:24:51.0663383Z 
2020-02-26T09:24:51.0664110Z ---- [codegen] codegen/repeat-trusted-len.rs stdout ----
2020-02-26T09:24:51.0664346Z 
2020-02-26T09:24:51.0664871Z error: verification with 'FileCheck' failed
2020-02-26T09:24:51.0665148Z status: exit code: 1
2020-02-26T09:24:51.0666177Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll" "/checkout/src/test/codegen/repeat-trusted-len.rs"
2020-02-26T09:24:51.0667336Z ------------------------------------------
2020-02-26T09:24:51.0667630Z 
2020-02-26T09:24:51.0667968Z ------------------------------------------
2020-02-26T09:24:51.0668309Z stderr:
2020-02-26T09:24:51.0668309Z stderr:
2020-02-26T09:24:51.0668680Z ------------------------------------------
2020-02-26T09:24:51.0669323Z /checkout/src/test/codegen/repeat-trusted-len.rs:16:11: error: CHECK: expected string not found in input
2020-02-26T09:24:51.0670276Z // CHECK:***@llvm.memset.p0i8.[[USIZE]](i8* {{(nonnull )?}}align 1{{.*}} %{{[0-9]+}}, i8 42, [[USIZE]] 100000, i1 false)
2020-02-26T09:24:51.0670673Z           ^
2020-02-26T09:24:51.0671322Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:15:33: note: scanning from here
2020-02-26T09:24:51.0672135Z define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #1 personality i32 (...)* @rust_eh_personality {
2020-02-26T09:24:51.0672780Z                                 ^
2020-02-26T09:24:51.0673554Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:15:33: note: with "USIZE" equal to "i32"
2020-02-26T09:24:51.0674399Z define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #1 personality i32 (...)* @rust_eh_personality {
2020-02-26T09:24:51.0674893Z                                 ^
2020-02-26T09:24:51.0675813Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:15:33: note: with "USIZE" equal to "i32"
2020-02-26T09:24:51.0676449Z define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #1 personality i32 (...)* @rust_eh_personality {
2020-02-26T09:24:51.0676910Z                                 ^
2020-02-26T09:24:51.0677584Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:28:2: note: possible intended match here
2020-02-26T09:24:51.0678120Z  tail call void @llvm.memset.p0i8.i64(i8* nonnull align 1 %1, i8 42, i64 100000, i1 false) #5, !noalias !9
2020-02-26T09:24:51.0678532Z 
2020-02-26T09:24:51.0678887Z ------------------------------------------
2020-02-26T09:24:51.0679052Z 
2020-02-26T09:24:51.0679265Z 
---
2020-02-26T09:24:51.0681637Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-26T09:24:51.0682081Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-26T09:24:51.0689852Z 
2020-02-26T09:24:51.0690307Z 
2020-02-26T09:24:51.0698420Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-wasm32-unknown-emscripten" "--mode" "codegen" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/12.9.1_64bit/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-26T09:24:51.0701295Z 
2020-02-26T09:24:51.0701400Z 
2020-02-26T09:24:51.0707719Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-emscripten --exclude src/libcore --exclude src/liballoc --exclude src/libproc_macro --exclude src/libstd --exclude src/libterm --exclude src/libtest
2020-02-26T09:24:51.0708507Z Build completed unsuccessfully in 2:38:03
2020-02-26T09:24:51.0708507Z Build completed unsuccessfully in 2:38:03
2020-02-26T09:24:51.0785243Z == clock drift check ==
2020-02-26T09:24:51.0800262Z   local time: Wed Feb 26 09:24:51 UTC 2020
2020-02-26T09:24:51.3642136Z   network time: Wed, 26 Feb 2020 09:24:51 GMT
2020-02-26T09:24:51.3644383Z == end clock drift check ==
2020-02-26T09:24:52.5900387Z 
2020-02-26T09:24:52.5994904Z ##[error]Bash exited with code '1'.
2020-02-26T09:24:52.6081215Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-26T09:24:52.6086571Z ==============================================================================
2020-02-26T09:24:52.6087008Z Task         : Get sources
2020-02-26T09:24:52.6087452Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
