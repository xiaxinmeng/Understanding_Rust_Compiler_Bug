plain
............................F...i...............................
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] mir-opt/remove_fake_borrows.rs stdout ----
7       let mut _0: i32;                     // return place in scope 0 at $DIR/remove_fake_borrows.rs:6:46: 6:49
8       let mut _3: isize;                   // in scope 0 at $DIR/remove_fake_borrows.rs:8:9: 8:16
9       let mut _4: &std::option::Option<&&i32>; // in scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
-       let mut _5: &&&i32;                  // in scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
-       let mut _6: &&i32;                   // in scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
+       let mut _5: &&i32;                   // in scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
+       let mut _6: &&&i32;                  // in scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
12       let mut _7: &i32;                    // in scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
13       let mut _8: bool;                    // in scope 0 at $DIR/remove_fake_borrows.rs:8:20: 8:21

34   
35       bb4: {
35       bb4: {
36 -         _4 = &shallow _1;                // scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
- -         _5 = &shallow ((_1 as Some).0: &&i32); // scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
- -         _6 = &shallow (*((_1 as Some).0: &&i32)); // scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
+ -         _5 = &shallow (*((_1 as Some).0: &&i32)); // scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
+ -         _6 = &shallow ((_1 as Some).0: &&i32); // scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
39 -         _7 = &shallow (*(*((_1 as Some).0: &&i32))); // scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
40 +         nop;                             // scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12
41 +         nop;                             // scope 0 at $DIR/remove_fake_borrows.rs:7:11: 7:12

thread '[mir-opt] mir-opt/remove_fake_borrows.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/remove_fake_borrows.match_guard.CleanupNonCodegenStatements.diff', src/tools/compiletest/src/runtest.rs:3360:25


failures:
    [mir-opt] mir-opt/remove_fake_borrows.rs
    [mir-opt] mir-opt/remove_fake_borrows.rs

test result: FAILED. 160 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.69s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:25
