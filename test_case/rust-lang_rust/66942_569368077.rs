plain
2019-12-28T00:16:02.0899527Z status: exit code: 2
2019-12-28T00:16:02.0899616Z command: "make" "make"
2019-12-28T00:16:02.0899673Z stdout:
2019-12-28T00:16:02.0900838Z ------------------------------------------
2019-12-28T00:16:02.0902868Z # We don't compile `opaque` with either optimizations or instrumentation.
2019-12-28T00:16:02.0904839Z # We don't compile `opaque` with either optimizations or instrumentation.
2019-12-28T00:16:02.0908361Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   opaque.rs
2019-12-28T00:16:02.0908655Z # Compile the test program with instrumentation
2019-12-28T00:16:02.0909127Z mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir
2019-12-28T00:16:02.0910281Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   interesting.rs \
2019-12-28T00:16:02.0911194Z  -Cprofile-generate="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir -O -Ccodegen-units=1
2019-12-28T00:16:02.0912666Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   main.rs -Cprofile-generate="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir -O
2019-12-28T00:16:02.0913693Z # The argument below generates to the expected branch weights
2019-12-28T00:16:02.0915668Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights/main aaaaaaaaaaaa2bbbbbbbbbbbb2bbbbbbbbbbbbbbbbcc || exit 1
2019-12-28T00:16:02.0917248Z "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-profdata merge \
2019-12-28T00:16:02.0918490Z  -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir/merged.profdata \
2019-12-28T00:16:02.0919111Z  "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir
2019-12-28T00:16:02.0921388Z LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights   interesting.rs \
2019-12-28T00:16:02.0922094Z  -Cprofile-use="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/prof_data_dir/merged.profdata -O \
2019-12-28T00:16:02.0922377Z  -Ccodegen-units=1 --emit=llvm-ir
2019-12-28T00:16:02.0922814Z cat "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pgo-branch-weights/pgo-branch-weights"/interesting.ll | "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" filecheck-patterns.txt
2019-12-28T00:16:02.0923184Z ------------------------------------------
2019-12-28T00:16:02.0923270Z stderr:
2019-12-28T00:16:02.0923521Z ------------------------------------------
2019-12-28T00:16:02.0923521Z ------------------------------------------
2019-12-28T00:16:02.0923835Z filecheck-patterns.txt:5:8: error: CHECK: expected string not found in input
2019-12-28T00:16:02.0924185Z CHECK: define void @function_called_twice(i32 %c) {{.*}} !prof [[function_called_twice_id:![0-9]+]] {
2019-12-28T00:16:02.0924357Z <stdin>:1:1: note: scanning from here
2019-12-28T00:16:02.0924357Z <stdin>:1:1: note: scanning from here
2019-12-28T00:16:02.0924737Z ; ModuleID = 'interesting.3a1fbbbh-cgu.0'
2019-12-28T00:16:02.0924895Z <stdin>:7:1: note: possible intended match here
2019-12-28T00:16:02.0924895Z <stdin>:7:1: note: possible intended match here
2019-12-28T00:16:02.0924982Z define void @function_called_twice(i32 %c) unnamed_addr #0 {
2019-12-28T00:16:02.0925050Z ^
2019-12-28T00:16:02.0925118Z make: *** [Makefile:30: all] Error 1
2019-12-28T00:16:02.0925451Z ------------------------------------------
2019-12-28T00:16:02.0925497Z 
2019-12-28T00:16:02.0925528Z 
2019-12-28T00:16:02.0925573Z 
---
2019-12-28T00:16:02.0927399Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-28T00:16:02.0927509Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-28T00:16:02.0927560Z 
2019-12-28T00:16:02.0927607Z 
2019-12-28T00:16:02.0933343Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitstreamreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/checkout/src/llvm-project/llvm/include -I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/include -std=c++11   -fno-exceptions -fno-rtti -D_GNU_SOURCE -D_DEBUG -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-28T00:16:02.0935311Z 
2019-12-28T00:16:02.0935348Z 
2019-12-28T00:16:02.0935821Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-28T00:16:02.0935928Z Build completed unsuccessfully in 2:17:38
2019-12-28T00:16:02.0935928Z Build completed unsuccessfully in 2:17:38
2019-12-28T00:16:02.1030184Z == clock drift check ==
2019-12-28T00:16:02.1051091Z   local time: Sat Dec 28 00:16:02 UTC 2019
2019-12-28T00:16:02.5827402Z   network time: Sat, 28 Dec 2019 00:16:02 GMT
2019-12-28T00:16:02.5832471Z == end clock drift check ==
2019-12-28T00:16:09.8516062Z 
2019-12-28T00:16:09.8619228Z ##[error]Bash exited with code '1'.
2019-12-28T00:16:09.8671902Z ##[section]Starting: Checkout
2019-12-28T00:16:09.8674483Z ==============================================================================
2019-12-28T00:16:09.8674597Z Task         : Get sources
2019-12-28T00:16:09.8674679Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
