plain
Suite(test::src/test/mir-opt) not skipped for "bootstrap::test::MirOpt" -- not in [src/tools/tidy]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 164 tests
...................................i...................................F....F....i........F......... 100/164
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] mir-opt/inline/cycle.rs stdout ----
4   fn g() -> () {
4   fn g() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/cycle.rs:11:8: 11:8
6       let _1: ();                          // in scope 0 at $DIR/cycle.rs:12:5: 12:12
+ +     let mut _2: fn() {main};             // in scope 0 at $DIR/cycle.rs:12:5: 12:12
+ +     let mut _5: ();                      // in scope 0 at $DIR/cycle.rs:12:5: 12:12
+ +     scope 1 (inlined f::<fn() {main}>) { // at $DIR/cycle.rs:12:5: 12:12
+ +         debug g => _2;                   // in scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +         let _3: ();                      // in scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +         let mut _4: &fn() {main};        // in scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +         scope 2 (inlined <fn() {main} as Fn<()>>::call - shim(fn() {main})) { // at $DIR/cycle.rs:12:5: 12:12
+ +         }
+ +     }
8       bb0: {
8       bb0: {
9           StorageLive(_1);                 // scope 0 at $DIR/cycle.rs:12:5: 12:12

-           _1 = f::<fn() {main}>(main) -> bb1; // scope 0 at $DIR/cycle.rs:12:5: 12:12
+ -         _1 = f::<fn() {main}>(main) -> bb1; // scope 0 at $DIR/cycle.rs:12:5: 12:12
+ +         StorageLive(_2);                 // scope 0 at $DIR/cycle.rs:12:5: 12:12
+ +         _2 = main;                       // scope 0 at $DIR/cycle.rs:12:5: 12:12
11                                            // mir::Constant
-                                            // + span: $DIR/cycle.rs:12:5: 12:6
-                                            // + literal: Const { ty: fn(fn() {main}) {f::<fn() {main}>}, val: Value(Scalar(<ZST>)) }
-                                            // mir::Constant
+ -                                          // + span: $DIR/cycle.rs:12:5: 12:6
+ -                                          // + literal: Const { ty: fn(fn() {main}) {f::<fn() {main}>}, val: Value(Scalar(<ZST>)) }
+ -                                          // mir::Constant
15                                            // + span: $DIR/cycle.rs:12:7: 12:11
16                                            // + literal: Const { ty: fn() {main}, val: Value(Scalar(<ZST>)) }
+ +         StorageLive(_3);                 // scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +         StorageLive(_4);                 // scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +         _4 = &_2;                        // scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +         StorageLive(_5);                 // scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +         _5 = const ();                   // scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +         _3 = move (*_4)() -> [return: bb4, unwind: bb2]; // scope 2 at $DIR/cycle.rs:12:5: 12:12
18   
19       bb1: {


+ +         StorageDead(_2);                 // scope 0 at $DIR/cycle.rs:12:5: 12:12
20           StorageDead(_1);                 // scope 0 at $DIR/cycle.rs:12:12: 12:13
21           _0 = const ();                   // scope 0 at $DIR/cycle.rs:11:8: 13:2
22           return;                          // scope 0 at $DIR/cycle.rs:13:2: 13:2
+ +     }
+ + 
+ + 
+ +     bb2 (cleanup): {
+ +         drop(_2) -> bb3;                 // scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +     }
+ + 
+ +     bb3 (cleanup): {
+ +         resume;                          // scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +     }
+ +     bb4: {
+ +     bb4: {
+ +         StorageDead(_5);                 // scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +         StorageDead(_4);                 // scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +         StorageDead(_3);                 // scope 1 at $DIR/cycle.rs:12:5: 12:12
+ +         drop(_2) -> bb1;                 // scope 1 at $DIR/cycle.rs:12:5: 12:12
24   }
25   


thread '[mir-opt] mir-opt/inline/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/cycle.g.Inline.diff', src/tools/compiletest/src/runtest.rs:3372:25

---- [mir-opt] mir-opt/inline/inline-cycle-generic.rs stdout ----
4   fn main() -> () {
4   fn main() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/inline-cycle-generic.rs:8:11: 8:11
6       let _1: ();                          // in scope 0 at $DIR/inline-cycle-generic.rs:9:5: 9:24
- +     scope 1 (inlined <C as Call>::call) { // at $DIR/inline-cycle-generic.rs:9:5: 9:24
- +         scope 2 (inlined <B<A> as Call>::call) { // at $DIR/inline-cycle-generic.rs:9:5: 9:24
- +             scope 3 (inlined <A as Call>::call) { // at $DIR/inline-cycle-generic.rs:9:5: 9:24
- +             }
- +         }
- +     }
14       bb0: {
14       bb0: {
15           StorageLive(_1);                 // scope 0 at $DIR/inline-cycle-generic.rs:9:5: 9:24

- -         _1 = <C as Call>::call() -> bb1; // scope 0 at $DIR/inline-cycle-generic.rs:9:5: 9:24
- +         _1 = <B<C> as Call>::call() -> bb1; // scope 3 at $DIR/inline-cycle-generic.rs:9:5: 9:24
+           _1 = <C as Call>::call() -> bb1; // scope 0 at $DIR/inline-cycle-generic.rs:9:5: 9:24
18                                            // mir::Constant
- -                                          // + span: $DIR/inline-cycle-generic.rs:9:5: 9:22
- -                                          // + literal: Const { ty: fn() {<C as Call>::call}, val: Value(Scalar(<ZST>)) }
- +                                          // + span: $DIR/inline-cycle-generic.rs:9:5: 9:24
- +                                          // + literal: Const { ty: fn() {<B<C> as Call>::call}, val: Value(Scalar(<ZST>)) }
+                                            // + span: $DIR/inline-cycle-generic.rs:9:5: 9:22
+                                            // + literal: Const { ty: fn() {<C as Call>::call}, val: Value(Scalar(<ZST>)) }
24   
25       bb1: {


thread '[mir-opt] mir-opt/inline/inline-cycle-generic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_cycle_generic.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3372:25
---- [mir-opt] mir-opt/inline/inline-cycle.rs stdout ----
4   fn one() -> () {
4   fn one() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/inline-cycle.rs:13:10: 13:10
6       let _1: ();                          // in scope 0 at $DIR/inline-cycle.rs:14:5: 14:24
- +     scope 1 (inlined <C as Call>::call) { // at $DIR/inline-cycle.rs:14:5: 14:24
- +     }
10       bb0: {
10       bb0: {
11           StorageLive(_1);                 // scope 0 at $DIR/inline-cycle.rs:14:5: 14:24

- -         _1 = <C as Call>::call() -> bb1; // scope 0 at $DIR/inline-cycle.rs:14:5: 14:24
- +         _1 = <A<C> as Call>::call() -> bb1; // scope 1 at $DIR/inline-cycle.rs:14:5: 14:24
+           _1 = <C as Call>::call() -> bb1; // scope 0 at $DIR/inline-cycle.rs:14:5: 14:24
14                                            // mir::Constant
- -                                          // + span: $DIR/inline-cycle.rs:14:5: 14:22
- -                                          // + literal: Const { ty: fn() {<C as Call>::call}, val: Value(Scalar(<ZST>)) }
- +                                          // + span: $DIR/inline-cycle.rs:14:5: 14:24
- +                                          // + literal: Const { ty: fn() {<A<C> as Call>::call}, val: Value(Scalar(<ZST>)) }
+                                            // + span: $DIR/inline-cycle.rs:14:5: 14:22
+                                            // + literal: Const { ty: fn() {<C as Call>::call}, val: Value(Scalar(<ZST>)) }
20   
21       bb1: {


thread '[mir-opt] mir-opt/inline/inline-cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_cycle.one.Inline.diff', src/tools/compiletest/src/runtest.rs:3372:25

failures:
    [mir-opt] mir-opt/inline/cycle.rs
    [mir-opt] mir-opt/inline/inline-cycle-generic.rs
    [mir-opt] mir-opt/inline/inline-cycle-generic.rs
    [mir-opt] mir-opt/inline/inline-cycle.rs

test result: FAILED. 158 passed; 3 failed; 3 ignored; 0 measured; 0 filtered out; finished in 5.63s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:09
