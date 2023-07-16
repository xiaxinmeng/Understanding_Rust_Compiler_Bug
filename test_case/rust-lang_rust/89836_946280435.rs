plain
......F........................i.................................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
30 -         _3 = [move _4];                  // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
31 -         _2 = &_3;                        // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
32 +                                          // + span: $DIR/const-promotion-extern-static.rs:9:31: 9:44
- +                                          // + literal: Const { ty: &[&i32; 1], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:6 ~ const_promotion_extern_static[4ac5]::BAR), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
+ +                                          // + literal: Const { ty: &[&i32; 1], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:6 ~ const_promotion_extern_static[2dd8]::BAR), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
34 +         _2 = &(*_6);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
35           _1 = move _2 as &[&i32] (Pointer(Unsize)); // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
36 -         StorageDead(_4);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:34: 9:35

thread '[mir-opt] mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_promotion_extern_static.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3368:25

---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
87                                            // + val: Unevaluated(main, [], Some(promoted[0]))
88                                            // mir::Constant
89                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ issue_73223[e23c]::main), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
+                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ issue_73223[dfa6]::main), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
91           _11 = _28;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
92           (_9.0: &i32) = move _10;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
93           (_9.1: &i32) = move _11;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3368:25

failures:
    [mir-opt] mir-opt/const-promotion-extern-static.rs
    [mir-opt] mir-opt/issue-73223.rs
    [mir-opt] mir-opt/issue-73223.rs

test result: FAILED. 160 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out; finished in 3.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:16
