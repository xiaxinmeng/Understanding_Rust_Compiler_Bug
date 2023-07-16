plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 165 tests
...................................i............................................i................... 100/165
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...............................i...........F......F...F..........

---- [mir-opt] mir-opt/separate_const_switch.rs stdout ----
23                   scope 9 {
23                   scope 9 {
24                       debug e => _16;      // in scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
25                       scope 10 (inlined <i32 as From<i32>>::from) { // at $DIR/separate_const_switch.rs:29:8: 29:10
-                           debug t => _18;  // in scope 10 at $DIR/separate_const_switch.rs:29:8: 29:10
+                           debug value => _18; // in scope 10 at $DIR/separate_const_switch.rs:29:8: 29:10
28                   }
29               }


thread '[mir-opt] mir-opt/separate_const_switch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/separate_const_switch.identity.SeparateConstSwitch.diff', src/tools/compiletest/src/runtest.rs:3360:25

---- [mir-opt] mir-opt/simplify_try.rs stdout ----
---- [mir-opt] mir-opt/simplify_try.rs stdout ----
22 -         debug e => _6;                   // in scope 2 at $DIR/simplify_try.rs:22:13: 22:14
23 +         debug e => ((_0 as Err).0: i32); // in scope 2 at $DIR/simplify_try.rs:22:13: 22:14
24           scope 5 (inlined <i32 as From<i32>>::from) { // at $DIR/simplify_try.rs:22:37: 22:50
- -             debug t => _9;               // in scope 5 at $DIR/simplify_try.rs:22:37: 22:50
- +             debug t => ((_0 as Err).0: i32); // in scope 5 at $DIR/simplify_try.rs:22:37: 22:50
+ -             debug value => _9;           // in scope 5 at $DIR/simplify_try.rs:22:37: 22:50
+ +             debug value => ((_0 as Err).0: i32); // in scope 5 at $DIR/simplify_try.rs:22:37: 22:50
27           }
28           scope 6 (inlined from_error::<u32, i32>) { // at $DIR/simplify_try.rs:22:26: 22:51
29 -             debug e => _8;               // in scope 6 at $DIR/simplify_try.rs:22:26: 22:51

thread '[mir-opt] mir-opt/simplify_try.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_try.try_identity.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3360:25
---- [mir-opt] mir-opt/simplify-arm.rs stdout ----
---- [mir-opt] mir-opt/simplify-arm.rs stdout ----
22 -         debug e => _6;                   // in scope 2 at $DIR/simplify-arm.rs:37:13: 37:14
23 +         debug e => ((_0 as Err).0: i32); // in scope 2 at $DIR/simplify-arm.rs:37:13: 37:14
24           scope 5 (inlined <i32 as From<i32>>::from) { // at $DIR/simplify-arm.rs:37:37: 37:50
- -             debug t => _9;               // in scope 5 at $DIR/simplify-arm.rs:37:37: 37:50
- +             debug t => ((_0 as Err).0: i32); // in scope 5 at $DIR/simplify-arm.rs:37:37: 37:50
+ -             debug value => _9;           // in scope 5 at $DIR/simplify-arm.rs:37:37: 37:50
+ +             debug value => ((_0 as Err).0: i32); // in scope 5 at $DIR/simplify-arm.rs:37:37: 37:50
27           }
28           scope 6 (inlined from_error::<u8, i32>) { // at $DIR/simplify-arm.rs:37:26: 37:51
29 -             debug e => _8;               // in scope 6 at $DIR/simplify-arm.rs:37:26: 37:51

thread '[mir-opt] mir-opt/simplify-arm.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_arm.id_try.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3360:25

failures:
    [mir-opt] mir-opt/separate_const_switch.rs
    [mir-opt] mir-opt/simplify-arm.rs
    [mir-opt] mir-opt/simplify-arm.rs
    [mir-opt] mir-opt/simplify_try.rs

test result: FAILED. 159 passed; 3 failed; 3 ignored; 0 measured; 0 filtered out; finished in 5.17s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:11
