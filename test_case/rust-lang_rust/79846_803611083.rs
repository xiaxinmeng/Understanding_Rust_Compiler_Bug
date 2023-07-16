plain
failures:

---- [codegen] codegen/vec-in-place.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll" "/checkout/src/test/codegen/vec-in-place.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/vec-in-place.rs:10:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: loop
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:233:39: note: found here
 br i1 %_25.not.i.i.i.i, label %bb9.i.loopexit.i.i.i, label %bb13.i.i.i.i
                                      ^~~~
/checkout/src/test/codegen/vec-in-place.rs:11:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: call
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:183:7: note: found here
 tail call void @llvm.experimental.noalias.scope.decl(metadata !22)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll
Check file: /checkout/src/test/codegen/vec-in-place.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
      178:  %_4.sroa.4.0..sroa_idx18 = getelementptr inbounds %"std::vec::Vec<isize>", %"std::vec::Vec<isize>"* %vec, i64 0, i32 1, i32 1
      179:  %_4.sroa.4.0.copyload = load i64, i64* %_4.sroa.4.0..sroa_idx18, align 8
      180:  %_4.sroa.5.0..sroa_idx20 = getelementptr inbounds %"std::vec::Vec<isize>", %"std::vec::Vec<isize>"* %vec, i64 0, i32 3
      181:  %_4.sroa.5.0.copyload = load i64, i64* %_4.sroa.5.0..sroa_idx20, align 8
      182:  %1 = getelementptr inbounds i64, i64* %_4.sroa.0.0.copyload, i64 %_4.sroa.5.0.copyload
      183:  tail call void @llvm.experimental.noalias.scope.decl(metadata !22)
not:11           !~~~                                                          error: no match expected
      184:  tail call void @llvm.experimental.noalias.scope.decl(metadata !25), !noalias !28
      185:  %2 = getelementptr inbounds %"std::iter::Map<std::vec::IntoIter<isize>, [closure@/checkout/src/test/codegen/vec-in-place.rs:12:25: 12:39]>", %"std::iter::Map<std::vec::IntoIter<isize>, [closure@/checkout/src/test/codegen/vec-in-place.rs:12:25: 12:39]>"* %_2.i.i, i64 0, i32 0, i64 0
      186:  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %2), !noalias !30
      187:  %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_idx2.sroa_cast = bitcast %"std::iter::Map<std::vec::IntoIter<isize>, [closure@/checkout/src/test/codegen/vec-in-place.rs:12:25: 12:39]>"* %_2.i.i to i64**
      188:  store i64* %_4.sroa.0.0.copyload, i64** %_2.sroa.0.sroa.0.0._2.sroa.0.0..sroa_idx2.sroa_cast, align 8, !noalias !22
        .
        .
        .
      228:  br i1 %_25.not.i20.i.i.i, label %bb9.i.i.i.i, label %bb13.i.i.i.i
      229: 
      230: bb8.i.i.i.i: ; preds = %bb13.i.i.i.i
      231:  %14 = getelementptr inbounds i64, i64* %_4.sroa.0.0.copyload, i64 %17
      232:  %_25.not.i.i.i.i = icmp sgt i64 %17, %_4.sroa.5.0.copyload
      233:  br i1 %_25.not.i.i.i.i, label %bb9.i.loopexit.i.i.i, label %bb13.i.i.i.i
not:10                                           !~~~                                error: no match expected
      234: 
      235: bb9.i.loopexit.i.i.i: ; preds = %bb8.i.i.i.i
      236:  %15 = getelementptr inbounds i64, i64* %16, i64 1
      237:  br label %bb9.i.i.i.i
        .
        .
        .
>>>>>>
---
test result: FAILED. 225 passed; 1 failed; 33 ignored; 0 measured; 0 filtered out; finished in 3.96s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:20:12
