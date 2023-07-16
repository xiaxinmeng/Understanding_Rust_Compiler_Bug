plain
test [mir-opt] mir-opt/storage_live_dead_in_statics.rs ... ok

failures:

---- [mir-opt] mir-opt/inline/issue-78442.rs stdout ----
15           StorageLive(_2);                 // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
16           StorageLive(_3);                 // scope 0 at $DIR/issue-78442.rs:11:5: 11:15
17           StorageLive(_4);                 // scope 0 at $DIR/issue-78442.rs:11:5: 11:15
-           _4 = hide_foo() -> [return: bb1, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:11:5: 11:15
+           _4 = hide_foo() -> bb1;          // scope 0 at $DIR/issue-78442.rs:11:5: 11:15
19                                            // mir::Constant
20                                            // + span: $DIR/issue-78442.rs:11:5: 11:13
21                                            // + literal: Const { ty: fn() -> impl std::ops::Fn<()> {hide_foo}, val: Value(Scalar(<ZST>)) }

25           _3 = &_4;                        // scope 0 at $DIR/issue-78442.rs:11:5: 11:15
26           StorageLive(_5);                 // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
27           nop;                             // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
-           _2 = <impl Fn<()> as Fn<()>>::call(move _3, move _5) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
+           _2 = <impl Fn<()> as Fn<()>>::call(move _3, move _5) -> bb2; // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
29                                            // mir::Constant
30                                            // + span: $DIR/issue-78442.rs:11:5: 11:15
31                                            // + literal: Const { ty: for<'r> extern "rust-call" fn(&'r impl std::ops::Fn<()>, ()) -> <impl std::ops::Fn<()> as std::ops::FnOnce<()>>::Output {<impl std::ops::Fn<()> as std::ops::Fn<()>>::call}, val: Value(Scalar(<ZST>)) }

37           StorageDead(_4);                 // scope 0 at $DIR/issue-78442.rs:11:17: 11:18
38           StorageDead(_2);                 // scope 0 at $DIR/issue-78442.rs:11:17: 11:18
39           _0 = const ();                   // scope 0 at $DIR/issue-78442.rs:10:3: 12:2
-           drop(_1) -> [return: bb3, unwind: bb5]; // scope 0 at $DIR/issue-78442.rs:12:1: 12:2
+           drop(_1) -> bb3;                 // scope 0 at $DIR/issue-78442.rs:12:1: 12:2
42   
43       bb3: {


44           return;                          // scope 0 at $DIR/issue-78442.rs:12:2: 12:2
-   
-   
-       bb4 (cleanup): {
-           drop(_1) -> bb5;                 // scope 0 at $DIR/issue-78442.rs:12:1: 12:2
-   
-   
-       bb5 (cleanup): {
-           resume;                          // scope 0 at $DIR/issue-78442.rs:7:1: 12:2
54   }
55   


thread '[mir-opt] mir-opt/inline/issue-78442.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/issue_78442.bar.RevealAll.diff', src/tools/compiletest/src/runtest.rs:3360:25


failures:
    [mir-opt] mir-opt/inline/issue-78442.rs
    [mir-opt] mir-opt/inline/issue-78442.rs

test result: FAILED. 142 passed; 1 failed; 23 ignored; 0 measured; 0 filtered out; finished in 6.09s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-wasm32-unknown-unknown" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v15.14.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:18:37
