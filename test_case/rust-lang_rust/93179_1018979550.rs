plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
......F.........................i...............................
failures:

---- [mir-opt] mir-opt/issue_76432.rs stdout ----
21       let mut _19: *const T;               // in scope 0 at $DIR/issue_76432.rs:9:54: 9:68
22       let mut _20: *const T;               // in scope 0 at $DIR/issue_76432.rs:9:70: 9:84
23       let mut _21: *const T;               // in scope 0 at $DIR/issue_76432.rs:9:70: 9:84
-       let mut _22: !;                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _22: !;                      // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
25       let mut _23: &[T; 3];                // in scope 0 at $DIR/issue_76432.rs:7:19: 7:29
26       scope 1 {
27           debug v => _2;                   // in scope 1 at $DIR/issue_76432.rs:7:9: 7:10
66       }
67   
68       bb1: {
68       bb1: {
-           StorageLive(_22);                // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           core::panicking::panic(const "internal error: entered unreachable code"); // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_22);                // scope 1 at $SRC_DIR/core/src/panic.rs:LL:COL
+           core::panicking::panic(const "internal error: entered unreachable code"); // scope 1 at $SRC_DIR/core/src/panic.rs:LL:COL
71                                            // mir::Constant
-                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                            // + span: $SRC_DIR/core/src/panic.rs:LL:COL
73                                            // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(Scalar(<ZST>)) }
74                                            // ty::Const
75                                            // + ty: &str

76                                            // + val: Value(Slice { data: Allocation { bytes: [105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110, 116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99, 111, 100, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1099511627775], len: Size { raw: 40 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 40 })
77                                            // mir::Constant
-                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                            // + span: $SRC_DIR/core/src/panic.rs:LL:COL
79                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110, 116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99, 111, 100, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1099511627775], len: Size { raw: 40 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 40 }) }
81   


thread '[mir-opt] mir-opt/issue_76432.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_76432.test.SimplifyComparisonIntegral.diff', src/tools/compiletest/src/runtest.rs:3372:25


failures:
    [mir-opt] mir-opt/issue_76432.rs
    [mir-opt] mir-opt/issue_76432.rs

test result: FAILED. 160 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 4.22s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:55
