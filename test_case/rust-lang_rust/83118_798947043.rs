plain
.................................................................................................... 9300/11553
.................................................................................................... 9400/11553
.................................................................................i......i........... 9500/11553
.................................................................................................... 9600/11553
...........................iiiiiii..iiiiii.i........................................................ 9700/11553
.................................................................................................... 9900/11553
.................................................................................................... 10000/11553
.................................................................................................... 10100/11553
.................................................................................................... 10200/11553
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.052 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i..i.i..i.....ii.i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 1.93s

 finished in 1.979 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiii.........

 finished in 0.500 seconds
Build completed successfully in 0:25:16
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
...............F..........i................................
failures:

---- [mir-opt] mir-opt/matches_reduce_branches.rs stdout ----
5       debug bar => _1;                     // in scope 0 at $DIR/matches_reduce_branches.rs:7:8: 7:11
6       let mut _0: ();                      // return place in scope 0 at $DIR/matches_reduce_branches.rs:7:25: 7:25
7       let mut _2: isize;                   // in scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
+ +     let mut _3: isize;                   // in scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
9       bb0: {
9       bb0: {
10           _2 = discriminant(_1);           // scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26

-           switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
-   
-       bb1: {
-       bb1: {
-           _0 = const ();                   // scope 0 at $DIR/matches_reduce_branches.rs:10:6: 10:6
-           goto -> bb3;                     // scope 0 at $DIR/matches_reduce_branches.rs:8:5: 10:6
-   
-       bb2: {
-       bb2: {
-           goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-   
-       bb3: {
-       bb3: {
+ -         switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
+ -     }
+ -     bb1: {
+ -     bb1: {
+ -         goto -> bb3;                     // scope 0 at $DIR/matches_reduce_branches.rs:8:5: 10:6
+ -     }
+ -     bb2: {
+ -     bb2: {
+ -         goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -     }
+ -     bb3: {
+ -     bb3: {
+ +         StorageLive(_3);                 // scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
+ +         _3 = move _2;                    // scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
+ +         StorageDead(_3);                 // scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
24           return;                          // scope 0 at $DIR/matches_reduce_branches.rs:11:2: 11:2
26   }


thread '[mir-opt] mir-opt/matches_reduce_branches.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/matches_reduce_branches.foo.MatchBranchSimplification.32bit.diff', src/tools/compiletest/src/runtest.rs:3516:25


failures:
    [mir-opt] mir-opt/matches_reduce_branches.rs
    [mir-opt] mir-opt/matches_reduce_branches.rs

test result: FAILED. 154 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out; finished in 1.71s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:28
