plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
................................i.................................
failures:

---- [mir-opt] mir-opt/inline/issue-78442.rs stdout ----
2 + // MIR for `bar` after RevealAll
3   
4   fn bar(_1: P) -> () {
-       debug _baz => _1;                    // in scope 0 at $DIR/issue-78442.rs:9:5: 9:9
-       let mut _0: ();                      // return place in scope 0 at $DIR/issue-78442.rs:10:3: 10:3
-       let _2: ();                          // in scope 0 at $DIR/issue-78442.rs:11:5: 11:17
- -     let mut _3: &impl std::ops::Fn<()>;  // in scope 0 at $DIR/issue-78442.rs:11:5: 11:15
- -     let _4: impl std::ops::Fn<()>;       // in scope 0 at $DIR/issue-78442.rs:11:5: 11:15
- +     let mut _3: &fn() {foo};             // in scope 0 at $DIR/issue-78442.rs:11:5: 11:15
- +     let _4: fn() {foo};                  // in scope 0 at $DIR/issue-78442.rs:11:5: 11:15
-       let mut _5: ();                      // in scope 0 at $DIR/issue-78442.rs:11:5: 11:17
+       debug _baz => _1;                    // in scope 0 at $DIR/issue-78442.rs:10:5: 10:9
+       let mut _0: ();                      // return place in scope 0 at $DIR/issue-78442.rs:11:3: 11:3
+       let _2: ();                          // in scope 0 at $DIR/issue-78442.rs:12:5: 12:17
+ -     let mut _3: &impl std::ops::Fn<()>;  // in scope 0 at $DIR/issue-78442.rs:12:5: 12:15
+ -     let _4: impl std::ops::Fn<()>;       // in scope 0 at $DIR/issue-78442.rs:12:5: 12:15
+ +     let mut _3: &fn() {foo};             // in scope 0 at $DIR/issue-78442.rs:12:5: 12:15
+ +     let _4: fn() {foo};                  // in scope 0 at $DIR/issue-78442.rs:12:5: 12:15
+       let mut _5: ();                      // in scope 0 at $DIR/issue-78442.rs:12:5: 12:17
14       bb0: {
14       bb0: {
-           StorageLive(_2);                 // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
-           StorageLive(_3);                 // scope 0 at $DIR/issue-78442.rs:11:5: 11:15
-           StorageLive(_4);                 // scope 0 at $DIR/issue-78442.rs:11:5: 11:15
-           _4 = hide_foo() -> [return: bb1, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:11:5: 11:15
+           StorageLive(_2);                 // scope 0 at $DIR/issue-78442.rs:12:5: 12:17
+           StorageLive(_3);                 // scope 0 at $DIR/issue-78442.rs:12:5: 12:15
+           StorageLive(_4);                 // scope 0 at $DIR/issue-78442.rs:12:5: 12:15
+           _4 = hide_foo() -> [return: bb1, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:12:5: 12:15
19                                            // mir::Constant
-                                            // + span: $DIR/issue-78442.rs:11:5: 11:13
+                                            // + span: $DIR/issue-78442.rs:12:5: 12:13
21                                            // + literal: Const { ty: fn() -> impl std::ops::Fn<()> {hide_foo}, val: Value(Scalar(<ZST>)) }
23   

24       bb1: {
24       bb1: {
-           _3 = &_4;                        // scope 0 at $DIR/issue-78442.rs:11:5: 11:15
-           StorageLive(_5);                 // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
-           nop;                             // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
-           _2 = <impl Fn<()> as Fn<()>>::call(move _3, move _5) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:11:5: 11:17
+           _3 = &_4;                        // scope 0 at $DIR/issue-78442.rs:12:5: 12:15
+           StorageLive(_5);                 // scope 0 at $DIR/issue-78442.rs:12:5: 12:17
+           nop;                             // scope 0 at $DIR/issue-78442.rs:12:5: 12:17
+           _2 = <impl Fn<()> as Fn<()>>::call(move _3, move _5) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:12:5: 12:17
29                                            // mir::Constant
-                                            // + span: $DIR/issue-78442.rs:11:5: 11:15
+                                            // + span: $DIR/issue-78442.rs:12:5: 12:15
31                                            // + literal: Const { ty: for<'r> extern "rust-call" fn(&'r impl std::ops::Fn<()>, ()) -> <impl std::ops::Fn<()> as std::ops::FnOnce<()>>::Output {<impl std::ops::Fn<()> as std::ops::Fn<()>>::call}, val: Value(Scalar(<ZST>)) }
33   

34       bb2: {
34       bb2: {
-           StorageDead(_5);                 // scope 0 at $DIR/issue-78442.rs:11:16: 11:17
-           StorageDead(_3);                 // scope 0 at $DIR/issue-78442.rs:11:16: 11:17
-           StorageDead(_4);                 // scope 0 at $DIR/issue-78442.rs:11:17: 11:18
-           StorageDead(_2);                 // scope 0 at $DIR/issue-78442.rs:11:17: 11:18
-           _0 = const ();                   // scope 0 at $DIR/issue-78442.rs:10:3: 12:2
-           drop(_1) -> [return: bb3, unwind: bb5]; // scope 0 at $DIR/issue-78442.rs:12:1: 12:2
+           StorageDead(_5);                 // scope 0 at $DIR/issue-78442.rs:12:16: 12:17
+           StorageDead(_3);                 // scope 0 at $DIR/issue-78442.rs:12:16: 12:17
+           StorageDead(_4);                 // scope 0 at $DIR/issue-78442.rs:12:17: 12:18
+           StorageDead(_2);                 // scope 0 at $DIR/issue-78442.rs:12:17: 12:18
+           _0 = const ();                   // scope 0 at $DIR/issue-78442.rs:11:3: 13:2
+           drop(_1) -> [return: bb3, unwind: bb5]; // scope 0 at $DIR/issue-78442.rs:13:1: 13:2
42   
43       bb3: {


-           return;                          // scope 0 at $DIR/issue-78442.rs:12:2: 12:2
+           return;                          // scope 0 at $DIR/issue-78442.rs:13:2: 13:2
46   
46   
47       bb4 (cleanup): {

-           drop(_1) -> bb5;                 // scope 0 at $DIR/issue-78442.rs:12:1: 12:2
+           drop(_1) -> bb5;                 // scope 0 at $DIR/issue-78442.rs:13:1: 13:2
50   
50   
51       bb5 (cleanup): {

-           resume;                          // scope 0 at $DIR/issue-78442.rs:7:1: 12:2
+           resume;                          // scope 0 at $DIR/issue-78442.rs:8:1: 13:2
54   }
55   


thread '[mir-opt] mir-opt/inline/issue-78442.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/issue_78442.bar.RevealAll.diff', src/tools/compiletest/src/runtest.rs:3360:25


failures:
    [mir-opt] mir-opt/inline/issue-78442.rs
    [mir-opt] mir-opt/inline/issue-78442.rs

test result: FAILED. 162 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.53s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:06
