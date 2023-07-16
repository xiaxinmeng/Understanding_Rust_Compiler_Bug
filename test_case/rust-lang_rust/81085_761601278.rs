plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

failures:

---- [codegen] codegen/slice-as_chunks.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/codegen/slice-as_chunks/slice-as_chunks.ll" "/checkout/obj/build/tmp/distcheck/src/test/codegen/slice-as_chunks.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/obj/build/tmp/distcheck/src/test/codegen/slice-as_chunks.rs:12:17: error: CHECK-NEXT: expected string not found in input
 // CHECK-NEXT: lshr i64 %x.1, 2
                ^
/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/codegen/slice-as_chunks/slice-as_chunks.ll:15:2: note: scanning from here
 %len.i = and i64 %x.1, -4
 ^
/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/codegen/slice-as_chunks/slice-as_chunks.ll:15:9: note: possible intended match here
 %len.i = and i64 %x.1, -4


Input file: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/codegen/slice-as_chunks/slice-as_chunks.ll
Check file: /checkout/obj/build/tmp/distcheck/src/test/codegen/slice-as_chunks.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
          10: @alloc47 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [63 x i8] }>, <{ [63 x i8] }>* @alloc46, i32 0, i32 0, i32 0), [16 x i8] c"?\00\00\00\00\00\00\00[\00\00\00\05\00\00\00" }>, align 8
          11: 
          12: ; Function Attrs: nonlazybind uwtable
          13: define { [0 x [4 x i8]]*, i64 } @chunks4([0 x i8]* noalias nonnull readonly align 1 %x.0, i64 %x.1) unnamed_addr #0 {
          14: start:
          15:  %len.i = and i64 %x.1, -4
next:12'0      X~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
next:12'1             ?                  possible intended match
          16:  %_71.i.i.i = icmp slt i64 %len.i, 0
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          17:  br i1 %_71.i.i.i, label %bb6.i.i.i, label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9as_chunks17h5eb20245ca5b2171E.exit"
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          18: 
next:12'0     ~
          19: bb6.i.i.i: ; preds = %start
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
          20: ; call core::panicking::panic
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>

---
test result: FAILED. 207 passed; 1 failed; 33 ignored; 0 measured; 0 filtered out; finished in 4.09s



command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/obj/build/tmp/distcheck/src/test/codegen" "--build-base" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.1-rust-1.51.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test --stage 2
Build completed unsuccessfully in 0:19:44
Build completed unsuccessfully in 0:19:44
make: *** [check] Error 1
Makefile:42: recipe for target 'check' failed

command did not execute successfully: "make" "check"
expected success, got: exit code: 2

