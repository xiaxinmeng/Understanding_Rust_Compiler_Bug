plain
 finished in 0.576 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 170 tests
.F.F...F................................i............................................i.............. 100/170
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [mir-opt] mir-opt/boxy_thing/or_pattern.rs stdout ----
21       bb2: {
21       bb2: {
22 -         switchInt(_2) -> [1_u32: bb6, 2_u32: bb7, 3_u32: bb8, otherwise: bb5]; // scope 1 at $DIR/or_pattern.rs:5:24: 5:31
- +         switchInt(_2) -> [2_u32: bb7, otherwise: bb6]; // scope 1 at $DIR/or_pattern.rs:5:24: 5:31
+ +         switchInt(_2) -> [1_u32: bb6, 2_u32: bb7, otherwise: bb5]; // scope 1 at $DIR/or_pattern.rs:5:24: 5:31
25   
26       bb3: {


thread '[mir-opt] mir-opt/boxy_thing/or_pattern.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/boxy_thing/or_pattern.foo.BoxyThing.diff', src/tools/compiletest/src/runtest.rs:3360:25


---- [mir-opt] mir-opt/boxy_thing/double_match.rs stdout ----
22           StorageLive(_2);                 // scope 0 at $DIR/double_match.rs:5:9: 5:14
23           _2 = _1;                         // scope 0 at $DIR/double_match.rs:5:9: 5:14
24 -         switchInt(_2) -> [1_u32: bb4, 2_u32: bb5, otherwise: bb3]; // scope 1 at $DIR/double_match.rs:5:18: 5:25
- +         goto -> bb4;                     // scope 1 at $DIR/double_match.rs:5:18: 5:25
+ +         switchInt(_2) -> [1_u32: bb4, otherwise: bb3]; // scope 1 at $DIR/double_match.rs:5:18: 5:25
27   
28       bb3: {


thread '[mir-opt] mir-opt/boxy_thing/double_match.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/boxy_thing/double_match.foo.BoxyThing.diff', src/tools/compiletest/src/runtest.rs:3360:25

---- [mir-opt] mir-opt/boxy_thing/refine_from_removal.rs stdout ----
26       bb2: {
26       bb2: {
27 -         switchInt(_3) -> [1_u32: bb6, 2_u32: bb7, 3_u32: bb8, otherwise: bb5]; // scope 2 at $DIR/refine_from_removal.rs:5:24: 5:31
- +         switchInt(_3) -> [2_u32: bb7, otherwise: bb6]; // scope 2 at $DIR/refine_from_removal.rs:5:24: 5:31
+ +         switchInt(_3) -> [1_u32: bb6, 2_u32: bb7, otherwise: bb5]; // scope 2 at $DIR/refine_from_removal.rs:5:24: 5:31
30   
31       bb3: {


thread '[mir-opt] mir-opt/boxy_thing/refine_from_removal.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/boxy_thing/refine_from_removal.foo.BoxyThing.diff', src/tools/compiletest/src/runtest.rs:3360:25

failures:
failures:
    [mir-opt] mir-opt/boxy_thing/double_match.rs
    [mir-opt] mir-opt/boxy_thing/or_pattern.rs
    [mir-opt] mir-opt/boxy_thing/refine_from_removal.rs
test result: FAILED. 164 passed; 3 failed; 3 ignored; 0 measured; 0 filtered out; finished in 6.62s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:59
