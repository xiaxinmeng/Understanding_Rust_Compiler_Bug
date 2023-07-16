plain
.................................................................................................... 9300/11531
.................................................................................................... 9400/11531
.......................................................................i......i..................... 9500/11531
.................................................................................................... 9600/11531
..........iiiiiii..iiiiii.i......................................................................... 9700/11531
.................................................................................................... 9900/11531
.................................................................................................... 10000/11531
.................................................................................................... 10100/11531
.................................................................................................... 10200/11531
---
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....F....................i................................
failures:

---- [mir-opt] mir-opt/issues/issue-59352.rs stdout ----
6     let mut _2: std::option::Option<u32>; // in scope 0 at $DIR/issue-59352.rs:14:26: 14:41
7     let mut _3: char;                    // in scope 0 at $DIR/issue-59352.rs:14:26: 14:29
8     let mut _4: u32;                     // in scope 0 at $DIR/issue-59352.rs:14:8: 14:23
-     let mut _9: isize;                   // in scope 0 at $DIR/issue-59352.rs:14:8: 14:23
10     scope 1 (inlined char::methods::<impl char>::is_digit) { // at $DIR/issue-59352.rs:14:8: 14:23
11         debug self => _7;                // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
12         debug radix => _4;               // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23

15         let mut _7: char;                // in scope 1 at $DIR/issue-59352.rs:14:8: 14:23
16         scope 2 (inlined Option::<u32>::is_some) { // at $DIR/issue-59352.rs:14:8: 14:23
17             debug self => _5;            // in scope 2 at $DIR/issue-59352.rs:14:8: 14:23
+             let mut _8: isize;           // in scope 2 at $DIR/issue-59352.rs:14:8: 14:23
19     }
19     }
20     scope 3 (inlined #[track_caller] Option::<u32>::unwrap) { // at $DIR/issue-59352.rs:14:26: 14:50

21         debug self => _2;                // in scope 3 at $DIR/issue-59352.rs:14:26: 14:50
-         let mut _8: isize;               // in scope 3 at $DIR/issue-59352.rs:14:26: 14:50
+         let mut _9: isize;               // in scope 3 at $DIR/issue-59352.rs:14:26: 14:50
23         scope 4 {
24             debug val => _0;             // in scope 4 at $DIR/issue-59352.rs:14:26: 14:50

54 
55     bb3: {
55     bb3: {
56         StorageDead(_3);                 // scope 0 at $DIR/issue-59352.rs:14:40: 14:41
-         StorageLive(_8);                 // scope 0 at $DIR/issue-59352.rs:14:26: 14:50
-         _8 = discriminant(_2);           // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
-         switchInt(move _8) -> [0_isize: bb6, 1_isize: bb8, otherwise: bb7]; // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
+         StorageLive(_9);                 // scope 0 at $DIR/issue-59352.rs:14:26: 14:50
+         _9 = discriminant(_2);           // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
+         switchInt(move _9) -> [0_isize: bb6, 1_isize: bb8, otherwise: bb7]; // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
61 
62     bb4: {

65 
65 
66     bb5: {
67         _5 = &_6;                        // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
-         _9 = discriminant((*_5));        // scope 2 at $DIR/issue-59352.rs:14:8: 14:23
+         _8 = discriminant((*_5));        // scope 2 at $DIR/issue-59352.rs:14:8: 14:23
69         StorageDead(_5);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
70         StorageDead(_6);                 // scope 1 at $DIR/issue-59352.rs:14:8: 14:23
71         StorageDead(_4);                 // scope 0 at $DIR/issue-59352.rs:14:8: 14:23

-         switchInt(move _9) -> [1_isize: bb1, otherwise: bb2]; // scope 0 at $DIR/issue-59352.rs:14:5: 14:63
+         switchInt(move _8) -> [1_isize: bb1, otherwise: bb2]; // scope 0 at $DIR/issue-59352.rs:14:5: 14:63
74 
75     bb6: {

91 
91 
92     bb8: {
93         _0 = move ((_2 as Some).0: u32); // scope 3 at $DIR/issue-59352.rs:14:26: 14:50
-         StorageDead(_8);                 // scope 0 at $DIR/issue-59352.rs:14:26: 14:50
+         StorageDead(_9);                 // scope 0 at $DIR/issue-59352.rs:14:26: 14:50
95         StorageDead(_2);                 // scope 0 at $DIR/issue-59352.rs:14:49: 14:50
96         goto -> bb4;                     // scope 0 at $DIR/issue-59352.rs:14:5: 14:63


thread '[mir-opt] mir-opt/issues/issue-59352.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issues/issue_59352.num_to_digit.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3514:25


failures:
    [mir-opt] mir-opt/issues/issue-59352.rs
    [mir-opt] mir-opt/issues/issue-59352.rs

test result: FAILED. 154 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 4.69s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:45
