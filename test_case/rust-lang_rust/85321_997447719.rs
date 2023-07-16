plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
................................i.................................
failures:

---- [mir-opt] mir-opt/inline/inline-cycle-generic.rs stdout ----
6       let _1: ();                          // in scope 0 at $DIR/inline-cycle-generic.rs:9:5: 9:24
7 +     scope 1 (inlined <C as Call>::call) { // at $DIR/inline-cycle-generic.rs:9:5: 9:24
8 +         scope 2 (inlined <B<A> as Call>::call) { // at $DIR/inline-cycle-generic.rs:9:5: 9:24
- +             scope 3 (inlined <A as Call>::call) { // at $DIR/inline-cycle-generic.rs:9:5: 9:24
- +             }
12 +     }
13   

14       bb0: {
14       bb0: {
15           StorageLive(_1);                 // scope 0 at $DIR/inline-cycle-generic.rs:9:5: 9:24
16 -         _1 = <C as Call>::call() -> bb1; // scope 0 at $DIR/inline-cycle-generic.rs:9:5: 9:24
- +         _1 = <B<C> as Call>::call() -> bb1; // scope 3 at $DIR/inline-cycle-generic.rs:9:5: 9:24
+ +         _1 = <A as Call>::call() -> bb1; // scope 2 at $DIR/inline-cycle-generic.rs:9:5: 9:24
18                                            // mir::Constant
19 -                                          // + span: $DIR/inline-cycle-generic.rs:9:5: 9:22
20 -                                          // + literal: Const { ty: fn() {<C as Call>::call}, val: Value(Scalar(<ZST>)) }

21 +                                          // + span: $DIR/inline-cycle-generic.rs:9:5: 9:24
- +                                          // + literal: Const { ty: fn() {<B<C> as Call>::call}, val: Value(Scalar(<ZST>)) }
+ +                                          // + literal: Const { ty: fn() {<A as Call>::call}, val: Value(Scalar(<ZST>)) }
24   
25       bb1: {


thread '[mir-opt] mir-opt/inline/inline-cycle-generic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_cycle_generic.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3360:25


failures:
    [mir-opt] mir-opt/inline/inline-cycle-generic.rs
    [mir-opt] mir-opt/inline/inline-cycle-generic.rs

test result: FAILED. 162 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.58s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:58
