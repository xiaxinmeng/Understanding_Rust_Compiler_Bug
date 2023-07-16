plain
.................................................................................................... 9000/11236
.................................................................................................... 9100/11236
.................................................................................................... 9200/11236
................................i......i............................................................ 9300/11236
.......................................................................iiiiii..iiiiii.i............. 9400/11236
.................................................................................................... 9600/11236
.................................................................................................... 9700/11236
.................................................................................................... 9800/11236
.................................................................................................... 9900/11236
---
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 151 tests
..........F...F.................i.........................................i......................... 100/151
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
24 }
24 }
25 
26 alloc0 (static: FOO, size: 8, align: 8) {
+     ╾───────alloc10───────╼                         │ ╾──────╼
28 }
29 
29 
- alloc9 (size: 180, align: 1) {
+ alloc10 (size: 180, align: 1) {
31     0x00 │ ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab │ ................
32     0x10 │ ab ab ab ab ab ab ab ab ab ab ab ab ╾──alloc4── │ ............╾───
33     0x20 │ ──────────╼ 01 ef cd ab 00 00 00 00 00 00 00 00 │ ───╼............
37     0x60 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
38     0x70 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
39     0x80 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ╾──── │ ..............╾─
39     0x80 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ╾──── │ ..............╾─
-     0x90 │ ─────alloc6─────╼ 00 00 ╾─────alloc7+0x63─────╼ │ ─────╼..╾──────╼
+     0x90 │ ─────alloc6─────╼ 00 00 ╾─────alloc8+0x63─────╼ │ ─────╼..╾──────╼
41     0xa0 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
42     0xb0 │ 00 00 00 00                                     │ ....

48 
48 
49 alloc6 (fn: main)
50 
- alloc7 (size: 100, align: 1) {
+ alloc8 (size: 100, align: 1) {
52     0x00 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
53     0x10 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
54     0x20 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................

thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3.main.ConstProp.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3452:25

---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
24 }
25 
25 
26 alloc0 (static: FOO, size: 16, align: 8) {
+     ╾───────alloc24───────╼ 03 00 00 00 00 00 00 00 │ ╾──────╼........
28 }
29 
29 
- alloc20 (size: 72, align: 8) {
+ alloc24 (size: 72, align: 8) {
31     0x00 │ 00 00 00 00 __ __ __ __ ╾───────alloc8────────╼ │ ....░░░░╾──────╼
32     0x10 │ 00 00 00 00 00 00 00 00 00 00 00 00 __ __ __ __ │ ............░░░░
-     0x20 │ ╾───────alloc12───────╼ 02 00 00 00 00 00 00 00 │ ╾──────╼........
-     0x30 │ 01 00 00 00 2a 00 00 00 ╾───────alloc19───────╼ │ ....*...╾──────╼
+     0x20 │ ╾───────alloc13───────╼ 02 00 00 00 00 00 00 00 │ ╾──────╼........
+     0x30 │ 01 00 00 00 2a 00 00 00 ╾───────alloc23───────╼ │ ....*...╾──────╼
35     0x40 │ 03 00 00 00 00 00 00 00                         │ ........
37 


38 alloc8 (size: 0, align: 8) {}
39 
- alloc12 (size: 16, align: 8) {
-     ╾───────alloc10───────╼ ╾───────alloc11───────╼ │ ╾──────╼╾──────╼
+ alloc13 (size: 16, align: 8) {
+     ╾───────alloc11───────╼ ╾───────alloc12───────╼ │ ╾──────╼╾──────╼
43 
43 
- alloc10 (size: 1, align: 1) {
+ alloc11 (size: 1, align: 1) {
46 }
47 


- alloc11 (size: 1, align: 1) {
+ alloc12 (size: 1, align: 1) {
50 }
51 


- alloc19 (size: 24, align: 8) {
-     0x00 │ ╾─────alloc15+0x3─────╼ ╾───────alloc16───────╼ │ ╾──────╼╾──────╼
-     0x10 │ ╾─────alloc18+0x2─────╼                         │ ╾──────╼
+ alloc23 (size: 24, align: 8) {
+     0x00 │ ╾─────alloc17+0x3─────╼ ╾───────alloc19───────╼ │ ╾──────╼╾──────╼
+     0x10 │ ╾─────alloc22+0x2─────╼                         │ ╾──────╼
56 
56 
- alloc15 (size: 4, align: 1) {
+ alloc17 (size: 4, align: 1) {
58     2a 45 15 6f                                     │ *E.o
60 


- alloc16 (size: 1, align: 1) {
+ alloc19 (size: 1, align: 1) {
62     2a                                              │ *
64 


- alloc18 (size: 4, align: 1) {
+ alloc22 (size: 4, align: 1) {
66     2a 45 15 6f                                     │ *E.o
68 


thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2.main.ConstProp.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3452:25

failures:
    [mir-opt] mir-opt/const_allocation2.rs
    [mir-opt] mir-opt/const_allocation3.rs
    [mir-opt] mir-opt/const_allocation3.rs

test result: FAILED. 146 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.71s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:32
