plain
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 165 tests
...................................i............................................i..................F 100/165
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...............................i............F........F..F........

---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
150           StorageDead(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
151           StorageDead(_14);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
152           StorageDead(_13);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_9);                 // scope 3 at $DIR/issue-73223.rs:8:25: 8:26
+           StorageDead(_8);                 // scope 3 at $DIR/issue-73223.rs:8:25: 8:26
155           nop;                             // scope 0 at $DIR/issue-73223.rs:1:11: 9:2
156           StorageDead(_6);                 // scope 1 at $DIR/issue-73223.rs:9:1: 9:2
157           StorageDead(_1);                 // scope 0 at $DIR/issue-73223.rs:9:1: 9:2

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3360:25

---- [mir-opt] mir-opt/retag.rs stdout ----
---- [mir-opt] mir-opt/retag.rs stdout ----
185         StorageDead(_22);                // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
186         StorageDead(_21);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
187         StorageDead(_20);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_16);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_15);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_13);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         StorageDead(_12);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         StorageDead(_16);                // scope 5 at $DIR/retag.rs:64:40: 64:41
+         StorageDead(_15);                // scope 5 at $DIR/retag.rs:64:40: 64:41
+         StorageDead(_13);                // scope 5 at $DIR/retag.rs:64:40: 64:41
+         StorageDead(_12);                // scope 5 at $DIR/retag.rs:64:40: 64:41
192         _0 = const ();                   // scope 0 at $DIR/retag.rs:57:18: 65:2
193         StorageDead(_9);                 // scope 4 at $DIR/retag.rs:65:1: 65:2
194         StorageDead(_8);                 // scope 2 at $DIR/retag.rs:65:1: 65:2

thread '[mir-opt] mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.array_casts.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3360:25
---- [mir-opt] mir-opt/unreachable_asm.rs stdout ----
---- [mir-opt] mir-opt/unreachable_asm.rs stdout ----
59           StorageDead(_6);                 // scope 2 at $DIR/unreachable_asm.rs:18:9: 18:10
60           StorageDead(_5);                 // scope 2 at $DIR/unreachable_asm.rs:18:9: 18:10
61           StorageLive(_7);                 // scope 2 at $DIR/unreachable_asm.rs:21:9: 21:37
-           llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []); // scope 3 at $DIR/unreachable_asm.rs:21:18: 21:35
+           llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []); // scope 3 at $DIR/unreachable_asm.rs:21:18: 21:34
63           _7 = const ();                   // scope 3 at $DIR/unreachable_asm.rs:21:9: 21:37
64           StorageDead(_7);                 // scope 2 at $DIR/unreachable_asm.rs:21:36: 21:37
65           StorageLive(_8);                 // scope 2 at $DIR/unreachable_asm.rs:22:9: 22:21

thread '[mir-opt] mir-opt/unreachable_asm.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unreachable_asm.main.UnreachablePropagation.diff', src/tools/compiletest/src/runtest.rs:3360:25
---- [mir-opt] mir-opt/unreachable_asm_2.rs stdout ----
49   
50       bb3: {
50       bb3: {
51           StorageLive(_7);                 // scope 2 at $DIR/unreachable_asm_2.rs:16:13: 16:41
-           llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []); // scope 3 at $DIR/unreachable_asm_2.rs:16:22: 16:39
+           llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []); // scope 3 at $DIR/unreachable_asm_2.rs:16:22: 16:38
53           _7 = const ();                   // scope 3 at $DIR/unreachable_asm_2.rs:16:13: 16:41
54           StorageDead(_7);                 // scope 2 at $DIR/unreachable_asm_2.rs:16:40: 16:41
55           _4 = const 21_i32;               // scope 2 at $DIR/unreachable_asm_2.rs:17:13: 17:20
60   
61       bb4: {
61       bb4: {
62           StorageLive(_8);                 // scope 2 at $DIR/unreachable_asm_2.rs:20:13: 20:41
-           llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []); // scope 4 at $DIR/unreachable_asm_2.rs:20:22: 20:39
+           llvm_asm!(LlvmInlineAsmInner { asm: "NOP", asm_str_style: Cooked, outputs: [], inputs: [], clobbers: [], volatile: true, alignstack: false, dialect: Att } : [] : []); // scope 4 at $DIR/unreachable_asm_2.rs:20:22: 20:38
64           _8 = const ();                   // scope 4 at $DIR/unreachable_asm_2.rs:20:13: 20:41
65           StorageDead(_8);                 // scope 2 at $DIR/unreachable_asm_2.rs:20:40: 20:41
66           _4 = const 42_i32;               // scope 2 at $DIR/unreachable_asm_2.rs:21:13: 21:20

thread '[mir-opt] mir-opt/unreachable_asm_2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unreachable_asm_2.main.UnreachablePropagation.diff', src/tools/compiletest/src/runtest.rs:3360:25

failures:
    [mir-opt] mir-opt/issue-73223.rs
    [mir-opt] mir-opt/retag.rs
    [mir-opt] mir-opt/retag.rs
    [mir-opt] mir-opt/unreachable_asm.rs
    [mir-opt] mir-opt/unreachable_asm_2.rs

test result: FAILED. 158 passed; 4 failed; 3 ignored; 0 measured; 0 filtered out; finished in 16.10s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:07
