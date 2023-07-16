plain
2020-02-25T21:10:35.2221682Z failures:
2020-02-25T21:10:35.2221816Z 
2020-02-25T21:10:35.2229798Z ---- [codegen] codegen/repeat-trusted-len.rs stdout ----
2020-02-25T21:10:35.2230769Z 
2020-02-25T21:10:35.2232164Z error: verification with 'FileCheck' failed
2020-02-25T21:10:35.2232429Z status: exit code: 1
2020-02-25T21:10:35.2233314Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll" "/checkout/src/test/codegen/repeat-trusted-len.rs"
2020-02-25T21:10:35.2234244Z ------------------------------------------
2020-02-25T21:10:35.2234405Z 
2020-02-25T21:10:35.2234917Z ------------------------------------------
2020-02-25T21:10:35.2235364Z stderr:
2020-02-25T21:10:35.2235364Z stderr:
2020-02-25T21:10:35.2237259Z ------------------------------------------
2020-02-25T21:10:35.2239588Z /checkout/src/test/codegen/repeat-trusted-len.rs:16:11: error: CHECK: expected string not found in input
2020-02-25T21:10:35.2242042Z // CHECK:***@llvm.memset.p0i8.[[USIZE]](i8* {{(nonnull )?}}align 1{{.*}} %{{[0-9]+}}, i8 42, [[USIZE]] 100000, i1 false)
2020-02-25T21:10:35.2243315Z           ^
2020-02-25T21:10:35.2243993Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:17:33: note: scanning from here
2020-02-25T21:10:35.2244787Z define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #1 personality i32 (i32, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
2020-02-25T21:10:35.2245412Z                                 ^
2020-02-25T21:10:35.2246032Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:17:33: note: with "USIZE" equal to "i32"
2020-02-25T21:10:35.2246825Z define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #1 personality i32 (i32, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
2020-02-25T21:10:35.2247412Z                                 ^
2020-02-25T21:10:35.2248043Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:17:33: note: with "USIZE" equal to "i32"
2020-02-25T21:10:35.2248822Z define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #1 personality i32 (i32, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
2020-02-25T21:10:35.2249410Z                                 ^
2020-02-25T21:10:35.2250675Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:30:2: note: possible intended match here
2020-02-25T21:10:35.2253279Z  tail call void @llvm.memset.p0i8.i64(i8* nonnull align 1 %1, i8 42, i64 100000, i1 false) #4, !noalias !11
2020-02-25T21:10:35.2253758Z 
2020-02-25T21:10:35.2254284Z ------------------------------------------
2020-02-25T21:10:35.2254483Z 
2020-02-25T21:10:35.2254612Z 
---
2020-02-25T21:10:35.2257508Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-25T21:10:35.2258084Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-25T21:10:35.2258320Z 
2020-02-25T21:10:35.2258562Z 
2020-02-25T21:10:35.2262834Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "codegen" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
2020-02-25T21:10:35.2267640Z 
2020-02-25T21:10:35.2267755Z 
2020-02-25T21:10:35.2331702Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
2020-02-25T21:10:35.2332131Z Build completed unsuccessfully in 2:28:57
2020-02-25T21:10:35.2332131Z Build completed unsuccessfully in 2:28:57
2020-02-25T21:10:35.2332432Z == clock drift check ==
2020-02-25T21:10:35.2352985Z   local time: Tue Feb 25 21:10:35 UTC 2020
2020-02-25T21:10:35.5270086Z   network time: Tue, 25 Feb 2020 21:10:35 GMT
2020-02-25T21:10:35.5303784Z == end clock drift check ==
2020-02-25T21:10:38.2734805Z 
2020-02-25T21:10:38.2830126Z ##[error]Bash exited with code '1'.
2020-02-25T21:10:38.2906572Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-25T21:10:38.2911284Z ==============================================================================
2020-02-25T21:10:38.2911650Z Task         : Get sources
2020-02-25T21:10:38.2912000Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
