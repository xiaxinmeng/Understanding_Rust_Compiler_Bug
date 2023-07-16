plain

---- [mir-opt] mir-opt/issue-41888.rs stdout ----
37     }
38 
39     bb2: {
- <<<<<<< HEAD
- ||||||| parent of 8faab6b3c00... Revert "Use `record_operands_moved` more aggresively"
-         _0 = const ();                   // scope 1 at $DIR/issue-41888.rs:14:6: 14:6
-         goto -> bb7;                     // scope 1 at $DIR/issue-41888.rs:8:5: 14:6
- 
-     bb3: {
- =======
- =======
-         _0 = const ();                   // scope 1 at $DIR/issue-41888.rs:14:6: 14:6
-         goto -> bb8;                     // scope 1 at $DIR/issue-41888.rs:8:5: 14:6
- 
-     bb3: {
-     bb3: {
- >>>>>>> 8faab6b3c00... Revert "Use `record_operands_moved` more aggresively"
54         StorageLive(_3);                 // scope 1 at $DIR/issue-41888.rs:9:13: 9:20
55         StorageLive(_4);                 // scope 1 at $DIR/issue-41888.rs:9:18: 9:19
56         _4 = K;                          // scope 1 at $DIR/issue-41888.rs:9:18: 9:19
61 
62     bb3: {
62     bb3: {
63         _0 = const ();                   // scope 1 at $DIR/issue-41888.rs:14:6: 14:6
-         goto -> bb7;                     // scope 1 at $DIR/issue-41888.rs:8:5: 14:6
+         goto -> bb8;                     // scope 1 at $DIR/issue-41888.rs:8:5: 14:6
66 
67     bb4: {


88         goto -> bb8;                     // scope 1 at $DIR/issue-41888.rs:10:9: 13:10
90 
90 
- <<<<<<< HEAD
-     bb7: {
-         StorageDead(_2);                 // scope 1 at $DIR/issue-41888.rs:14:5: 14:6
-         goto -> bb18;                    // scope 0 at $DIR/issue-41888.rs:15:1: 15:2
- ||||||| parent of 8faab6b3c00... Revert "Use `record_operands_moved` more aggresively"
-     bb7: {
-         goto -> bb18;                    // scope 0 at $DIR/issue-41888.rs:15:1: 15:2
- =======
99     bb8: {
+         StorageDead(_2);                 // scope 1 at $DIR/issue-41888.rs:14:5: 14:6
100         goto -> bb20;                    // scope 0 at $DIR/issue-41888.rs:15:1: 15:2
- >>>>>>> 8faab6b3c00... Revert "Use `record_operands_moved` more aggresively"
103 
104     bb9: {


thread '[mir-opt] mir-opt/issue-41888.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_41888.main.ElaborateDrops.after.mir', src/tools/compiletest/src/runtest.rs:3452:25


failures:
    [mir-opt] mir-opt/issue-41888.rs
    [mir-opt] mir-opt/issue-41888.rs

test result: FAILED. 148 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 4.08s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:06
