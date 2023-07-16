plain
2019-12-26T09:53:07.6224613Z status: exit code: 2
2019-12-26T09:53:07.6224708Z command: "make" "make"
2019-12-26T09:53:07.6224781Z stdout:
2019-12-26T09:53:07.6225186Z ------------------------------------------
2019-12-26T09:53:07.6225539Z # We don't compile `opaque` with either optimizations or instrumentation.
2019-12-26T09:53:07.6225906Z # We don't compile `opaque` with either optimizations or instrumentation.
2019-12-26T09:53:07.6227079Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   opaque.rs
2019-12-26T09:53:07.6227580Z # Compile the test program with instrumentation
2019-12-26T09:53:07.6228260Z mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir
2019-12-26T09:53:07.6229566Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   interesting.rs \
2019-12-26T09:53:07.6230309Z  -Cprofile-generate="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir -O -Ccodegen-units=1
2019-12-26T09:53:07.6231924Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   main.rs -Cprofile-generate="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir -O
2019-12-26T09:53:07.6232322Z # The argument below generates to the expected branch weights
2019-12-26T09:53:07.6233402Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights/main aaaaaaaaaaaa2bbbbbbbbbbbb2bbbbbbbbbbbbbbbbcc || exit 1
2019-12-26T09:53:07.6233988Z "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge \
2019-12-26T09:53:07.6234460Z  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir/merged.profdata \
2019-12-26T09:53:07.6238549Z  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir
2019-12-26T09:53:07.6239938Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   interesting.rs \
2019-12-26T09:53:07.6240659Z  -Cprofile-use="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir/merged.profdata -O \
2019-12-26T09:53:07.6240998Z  -Ccodegen-units=1 --emit=llvm-ir
2019-12-26T09:53:07.6241595Z cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/interesting.ll | "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" filecheck-patterns.txt
2019-12-26T09:53:07.6242216Z ------------------------------------------
2019-12-26T09:53:07.6242346Z stderr:
2019-12-26T09:53:07.6242690Z ------------------------------------------
2019-12-26T09:53:07.6242690Z ------------------------------------------
2019-12-26T09:53:07.6243064Z filecheck-patterns.txt:5:8: error: CHECK: expected string not found in input
2019-12-26T09:53:07.6243470Z CHECK: define void @function_called_twice(i32 %c) {{.*}} !prof [[function_called_twice_id:![0-9]+]] {
2019-12-26T09:53:07.6243695Z <stdin>:1:1: note: scanning from here
2019-12-26T09:53:07.6243695Z <stdin>:1:1: note: scanning from here
2019-12-26T09:53:07.6244000Z ; ModuleID = 'interesting.3a1fbbbh-cgu.0'
2019-12-26T09:53:07.6244180Z <stdin>:7:1: note: possible intended match here
2019-12-26T09:53:07.6244180Z <stdin>:7:1: note: possible intended match here
2019-12-26T09:53:07.6244292Z define void @function_called_twice(i32 %c) unnamed_addr #0 {
2019-12-26T09:53:07.6244379Z ^
2019-12-26T09:53:07.6244582Z make: *** [Makefile:30: all] Error 1
2019-12-26T09:53:07.6244984Z ------------------------------------------
2019-12-26T09:53:07.6245045Z 
2019-12-26T09:53:07.6245099Z 
2019-12-26T09:53:07.6245156Z 
---
2019-12-26T09:53:07.6246429Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-26T09:53:07.6246564Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-26T09:53:07.6246683Z 
2019-12-26T09:53:07.6246744Z 
2019-12-26T09:53:07.6252648Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitstreamreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/checkout/src/llvm-project/llvm/include -I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/include -std=c++11   -fno-exceptions -fno-rtti -D_GNU_SOURCE -D_DEBUG -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-26T09:53:07.6280652Z 
2019-12-26T09:53:07.6280717Z 
2019-12-26T09:53:07.6280802Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-26T09:53:07.6280912Z Build completed unsuccessfully in 2:35:17
2019-12-26T09:53:07.6280912Z Build completed unsuccessfully in 2:35:17
2019-12-26T09:53:07.6320095Z == clock drift check ==
2019-12-26T09:53:07.6337935Z   local time: Thu Dec 26 09:53:07 UTC 2019
2019-12-26T09:53:07.8085088Z   network time: Thu, 26 Dec 2019 09:53:07 GMT
2019-12-26T09:53:07.8086175Z == end clock drift check ==
2019-12-26T09:53:17.8930303Z 
2019-12-26T09:53:17.9048381Z ##[error]Bash exited with code '1'.
2019-12-26T09:53:17.9099516Z ##[section]Starting: Checkout
2019-12-26T09:53:17.9101812Z ==============================================================================
2019-12-26T09:53:17.9101935Z Task         : Get sources
2019-12-26T09:53:17.9102042Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
