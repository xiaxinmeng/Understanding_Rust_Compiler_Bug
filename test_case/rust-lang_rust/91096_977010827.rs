plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.................................i................................
failures:

---- [mir-opt] mir-opt/inline/issue-78442.rs stdout ----
5       debug _baz => _1;                    // in scope 0 at $DIR/issue-78442.rs:9:5: 9:9
6       let mut _0: ();                      // return place in scope 0 at $DIR/issue-78442.rs:10:3: 10:3
7       let _2: ();                          // in scope 0 at $DIR/issue-78442.rs:11:5: 11:17
- -     let mut _3: &impl Fn() -> ();        // in scope 0 at $DIR/issue-78442.rs:11:5: 11:15
- -     let _4: impl Fn() -> ();             // in scope 0 at $DIR/issue-78442.rs:11:5: 11:15
+ -     let mut _3: &impl Fn();              // in scope 0 at $DIR/issue-78442.rs:11:5: 11:15
+ -     let _4: impl Fn();                   // in scope 0 at $DIR/issue-78442.rs:11:5: 11:15
10 +     let mut _3: &fn() {foo};             // in scope 0 at $DIR/issue-78442.rs:11:5: 11:15
11 +     let _4: fn() {foo};                  // in scope 0 at $DIR/issue-78442.rs:11:5: 11:15
12       let mut _5: ();                      // in scope 0 at $DIR/issue-78442.rs:11:5: 11:17

18           _4 = hide_foo() -> [return: bb1, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:11:5: 11:15
19                                            // mir::Constant
20                                            // + span: $DIR/issue-78442.rs:11:5: 11:13
-                                            // + literal: Const { ty: fn() -> impl Fn() -> () {hide_foo}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: fn() -> impl Fn() {hide_foo}, val: Value(Scalar(<ZST>)) }
23   
24       bb1: {


25           _3 = &_4;                        // scope 0 at $DIR/issue-78442.rs:11:5: 11:15
26           StorageLive(_5);                 // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
27           nop;                             // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
-           _2 = <impl Fn() -> () as Fn<()>>::call(move _3, move _5) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
+           _2 = <impl Fn() as Fn<()>>::call(move _3, move _5) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
29                                            // mir::Constant
30                                            // + span: $DIR/issue-78442.rs:11:5: 11:15
-                                            // + literal: Const { ty: for<'r> extern "rust-call" fn(&'r impl Fn() -> (), ()) -> <impl Fn() -> () as std::ops::FnOnce<()>>::Output {<impl Fn() -> () as std::ops::Fn<()>>::call}, val: Value(Scalar(<ZST>)) }
+                                            // + literal: Const { ty: for<'r> extern "rust-call" fn(&'r impl Fn(), ()) -> <impl Fn() as std::ops::FnOnce<()>>::Output {<impl Fn() as std::ops::Fn<()>>::call}, val: Value(Scalar(<ZST>)) }
33   
34       bb2: {


thread '[mir-opt] mir-opt/inline/issue-78442.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/issue_78442.bar.RevealAll.diff', src/tools/compiletest/src/runtest.rs:3357:25


failures:
    [mir-opt] mir-opt/inline/issue-78442.rs
    [mir-opt] mir-opt/inline/issue-78442.rs

test result: FAILED. 162 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 5.23s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:04
