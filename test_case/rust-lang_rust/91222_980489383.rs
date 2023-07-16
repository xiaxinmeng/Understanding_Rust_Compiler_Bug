plain
.................F...............i.................................
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu

---- [mir-opt] mir-opt/matches_reduce_branches.rs stdout ----
1 - // MIR for `match_nested_if` before MatchBranchSimplification
2 + // MIR for `match_nested_if` after MatchBranchSimplification
+   
4   fn match_nested_if() -> bool {
4   fn match_nested_if() -> bool {
5       let mut _0: bool;                    // return place in scope 0 at $DIR/matches_reduce_branches.rs:39:25: 39:29
6       let _1: bool;                        // in scope 0 at $DIR/matches_reduce_branches.rs:40:9: 40:12
13       scope 1 {
13       scope 1 {
14           debug val => _1;                 // in scope 1 at $DIR/matches_reduce_branches.rs:40:9: 40:12
- 
+   
17       bb0: {
17       bb0: {
18           StorageLive(_1);                 // scope 0 at $DIR/matches_reduce_branches.rs:40:9: 40:12
19           StorageLive(_2);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:81

24                                            // + span: $DIR/matches_reduce_branches.rs:41:24: 41:31
25                                            // + literal: Const { ty: fn() -> bool {unknown}, val: Value(Scalar(<ZST>)) }
- 
+   
28       bb1: {
28       bb1: {
29 -         switchInt(move _4) -> [false: bb3, otherwise: bb2]; // scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:33
30 -     }
- -
+ - 
32 -     bb2: {
32 -     bb2: {
33 -         _3 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:41:36: 41:40
34 -         goto -> bb4;                     // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:57
35 -     }
- -
+ - 
37 -     bb3: {
37 -     bb3: {
38 -         _3 = const false;                // scope 0 at $DIR/matches_reduce_branches.rs:41:50: 41:55
39 -         goto -> bb4;                     // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:57
40 -     }
- -
+ - 
42 -     bb4: {
42 -     bb4: {
43 +         StorageLive(_5);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:33
44 +         _5 = move _4;                    // scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:33

47           StorageDead(_4);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:56: 41:57
48 -         switchInt(move _3) -> [false: bb6, otherwise: bb5]; // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:57
49 -     }
+ - 
51 -     bb5: {
51 -     bb5: {
52 -         _2 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:41:60: 41:64
53 -         goto -> bb7;                     // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:81
54 -     }
- -
+ - 
56 -     bb6: {
56 -     bb6: {
57 -         _2 = const false;                // scope 0 at $DIR/matches_reduce_branches.rs:41:74: 41:79
58 -         goto -> bb7;                     // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:81
59 -     }
- -
+ - 
61 -     bb7: {
61 -     bb7: {
62 +         StorageLive(_6);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:57
63 +         _6 = move _3;                    // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:57

66           StorageDead(_3);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:80: 41:81
67 -         switchInt(move _2) -> [false: bb9, otherwise: bb8]; // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:81
68 -     }
+ - 
70 -     bb8: {
70 -     bb8: {
71 +         StorageLive(_7);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:81
72 +         _7 = move _2;                    // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:81

74 -         _1 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:47:13: 47:17
75 -         goto -> bb10;                    // scope 0 at $DIR/matches_reduce_branches.rs:47:13: 47:17
76 -     }
+ - 
78 -     bb9: {
78 -     bb9: {
79 -         StorageDead(_2);                 // scope 0 at $DIR/matches_reduce_branches.rs:45:9: 45:10
80 -         _1 = const false;                // scope 0 at $DIR/matches_reduce_branches.rs:49:14: 49:19

81 -         goto -> bb10;                    // scope 0 at $DIR/matches_reduce_branches.rs:49:14: 49:19
82 -     }
+ - 
84 -     bb10: {
84 -     bb10: {
85 +         _1 = Ne(_7, const false);        // scope 0 at $DIR/matches_reduce_branches.rs:49:14: 49:19
86 +         StorageDead(_7);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:81

thread '[mir-opt] mir-opt/matches_reduce_branches.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/matches_reduce_branches.match_nested_if.MatchBranchSimplification.32bit.diff', src/tools/compiletest/src/runtest.rs:3357:25


failures:
    [mir-opt] mir-opt/matches_reduce_branches.rs
    [mir-opt] mir-opt/matches_reduce_branches.rs

test result: FAILED. 162 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out; finished in 2.26s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:01:45
