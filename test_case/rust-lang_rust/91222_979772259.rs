plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 167 tests
....................................i............................................i.................. 100/167
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.................................i.F...............................


---- [mir-opt] mir-opt/simplify_if_generic_constant.rs stdout ----
4   fn use_associated_const() -> u8 {
4   fn use_associated_const() -> u8 {
5       let mut _0: u8;                      // return place in scope 0 at $DIR/simplify_if_generic_constant.rs:8:51: 8:53
-       let mut _1: bool;                    // in scope 0 at $DIR/simplify_if_generic_constant.rs:9:5: 9:27
+       let mut _1: bool;                    // in scope 0 at $DIR/simplify_if_generic_constant.rs:9:8: 9:30
8       bb0: {
8       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/simplify_if_generic_constant.rs:9:5: 9:27
- -         _1 = const <T as HasBoolConst>::B; // scope 0 at $DIR/simplify_if_generic_constant.rs:9:5: 9:27
- -         switchInt(move _1) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/simplify_if_generic_constant.rs:9:5: 9:27
- +         switchInt(const <T as HasBoolConst>::B) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/simplify_if_generic_constant.rs:9:5: 9:27
+           StorageLive(_1);                 // scope 0 at $DIR/simplify_if_generic_constant.rs:9:8: 9:30
+ -         _1 = const <T as HasBoolConst>::B; // scope 0 at $DIR/simplify_if_generic_constant.rs:9:8: 9:30
+ -         switchInt(move _1) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/simplify_if_generic_constant.rs:9:8: 9:30
+ +         switchInt(const <T as HasBoolConst>::B) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/simplify_if_generic_constant.rs:9:8: 9:30
14   
15       bb1: {


-           _0 = const 13_u8;                // scope 0 at $DIR/simplify_if_generic_constant.rs:10:3: 10:5
-           goto -> bb3;                     // scope 0 at $DIR/simplify_if_generic_constant.rs:9:2: 13:3
+           _0 = const 13_u8;                // scope 0 at $DIR/simplify_if_generic_constant.rs:10:9: 10:11
+           goto -> bb3;                     // scope 0 at $DIR/simplify_if_generic_constant.rs:9:5: 13:6
19   
20       bb2: {


-           _0 = const 42_u8;                // scope 0 at $DIR/simplify_if_generic_constant.rs:12:3: 12:5
-           goto -> bb3;                     // scope 0 at $DIR/simplify_if_generic_constant.rs:9:2: 13:3
+           _0 = const 42_u8;                // scope 0 at $DIR/simplify_if_generic_constant.rs:12:9: 12:11
+           goto -> bb3;                     // scope 0 at $DIR/simplify_if_generic_constant.rs:9:5: 13:6
24   
25       bb3: {


-           StorageDead(_1);                 // scope 0 at $DIR/simplify_if_generic_constant.rs:13:2: 13:3
+           StorageDead(_1);                 // scope 0 at $DIR/simplify_if_generic_constant.rs:13:5: 13:6
27           return;                          // scope 0 at $DIR/simplify_if_generic_constant.rs:14:2: 14:2
29   }


thread '[mir-opt] mir-opt/simplify_if_generic_constant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_if_generic_constant.use_associated_const.SimplifyIfConst.diff', src/tools/compiletest/src/runtest.rs:3357:25


failures:
    [mir-opt] mir-opt/simplify_if_generic_constant.rs
    [mir-opt] mir-opt/simplify_if_generic_constant.rs

test result: FAILED. 163 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 5.86s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:04
