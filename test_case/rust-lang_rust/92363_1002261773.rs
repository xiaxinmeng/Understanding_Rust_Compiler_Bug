plain
Warning: Skipping "src/test/mir-opt": not a regular file or directory
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 166 tests
....................F..............i............F................i...............i.................. 100/166
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
........F.......................i.................................

---- [mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
---- [mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
31                                            // + val: Unevaluated(main, [], Some(promoted[0]))
32                                            // mir::Constant
33                                            // + span: $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
-                                            // + literal: Const { ty: &[i32; 3], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ bad_op_unsafe_oob_for_slices[HASH]::main), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
+                                            // + literal: Const { ty: &[i32; 3], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ bad_op_unsafe_oob_for_slices[fbcf]::main), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
35           _3 = _9;                         // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
36           _2 = &raw const (*_3);           // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
37           _1 = move _2 as *const [i32] (Pointer(Unsize)); // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35

thread '[mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3360:25

---- [mir-opt] mir-opt/const_prop/slice_len.rs stdout ----
---- [mir-opt] mir-opt/const_prop/slice_len.rs stdout ----
25                                            // + val: Unevaluated(main, [], Some(promoted[0]))
26                                            // mir::Constant
27                                            // + span: $DIR/slice_len.rs:5:6: 5:19
-                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ slice_len[HASH]::main), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
+                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ slice_len[7261]::main), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
29           _4 = _9;                         // scope 0 at $DIR/slice_len.rs:5:6: 5:19
30           _3 = _4;                         // scope 0 at $DIR/slice_len.rs:5:6: 5:19
31           StorageLive(_10);                // scope 0 at $DIR/slice_len.rs:5:6: 5:19

thread '[mir-opt] mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/slice_len.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3360:25
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
87                                            // + val: Unevaluated(main, [], Some(promoted[0]))
88                                            // mir::Constant
89                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ issue_73223[HASH]::main), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
+                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ issue_73223[9565]::main), const_param_did: None }, substs_: Some([]), promoted: Some(promoted[0]) }) }
91           _11 = _28;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
92           (_9.0: &i32) = move _10;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
93           (_9.1: &i32) = move _11;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3360:25

failures:
    [mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs
    [mir-opt] mir-opt/const_prop/slice_len.rs
    [mir-opt] mir-opt/const_prop/slice_len.rs
    [mir-opt] mir-opt/issue-73223.rs

test result: FAILED. 159 passed; 3 failed; 4 ignored; 0 measured; 0 filtered out; finished in 3.04s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:01:40
