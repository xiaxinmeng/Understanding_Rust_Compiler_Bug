plain
.................................................................................................... 9300/11532
.................................................................................................... 9400/11532
.................................................................................i......i........... 9500/11532
.................................................................................................... 9600/11532
....................iiiiiii..iiiiii.i............................................................... 9700/11532
.................................................................................................... 9900/11532
.................................................................................................... 10000/11532
.................................................................................................... 10100/11532
.................................................................................................... 10200/11532
---
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.066 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.27s

 finished in 2.337 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 21 tests
iiiiiiiiiiii.........

 finished in 0.607 seconds
Build completed successfully in 0:28:50
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
.........................i...........................F....
failures:

---- [mir-opt] mir-opt/while_let_loops.rs stdout ----
9       let mut _4: isize;                   // in scope 0 at $DIR/while_let_loops.rs:7:15: 7:25
10       let mut _5: !;                       // in scope 0 at $DIR/while_let_loops.rs:7:33: 10:6
11       let mut _6: !;                       // in scope 0 at $DIR/while_let_loops.rs:7:5: 10:6
+       let _7: ();                          // in scope 0 at $DIR/while_let_loops.rs:7:5: 10:6
+       let mut _8: !;                       // in scope 0 at $DIR/while_let_loops.rs:7:5: 10:6
12       scope 1 {
13           debug _x => _1;                  // in scope 1 at $DIR/while_let_loops.rs:6:9: 6:15

25       }
26   
27       bb1: {
27       bb1: {
+           StorageLive(_7);                 // scope 1 at $DIR/while_let_loops.rs:7:5: 10:6
28           _0 = const ();                   // scope 1 at $DIR/while_let_loops.rs:7:5: 10:6
-           goto -> bb4;                     // scope 1 at $DIR/while_let_loops.rs:7:5: 10:6
+           StorageDead(_7);                 // scope 1 at $DIR/while_let_loops.rs:10:5: 10:6
+           goto -> bb4;                     // scope 1 at $DIR/while_let_loops.rs:1:1: 1:1
31   
32       bb2: {


thread '[mir-opt] mir-opt/while_let_loops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/while_let_loops.change_loop_body.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3514:25


failures:
    [mir-opt] mir-opt/while_let_loops.rs
    [mir-opt] mir-opt/while_let_loops.rs

test result: FAILED. 153 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out; finished in 2.20s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:38
