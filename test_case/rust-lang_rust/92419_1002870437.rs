plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
........F.....
failures:

---- [codegen] codegen/unwind-landingpad-cold.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-landingpad-cold/unwind-landingpad-cold.ll" "/checkout/src/test/codegen/unwind-landingpad-cold.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/unwind-landingpad-cold.rs:9:11: error: CHECK: expected string not found in input
// CHECK: attributes [[ATTRIBUTES]] = { cold }
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-landingpad-cold/unwind-landingpad-cold.ll:61:2: note: scanning from here
 br label %bb4
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-landingpad-cold/unwind-landingpad-cold.ll:61:2: note: with "ATTRIBUTES" equal to "#4"
 br label %bb4
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-landingpad-cold/unwind-landingpad-cold.ll:70:15: note: possible intended match here
 %6 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-landingpad-cold/unwind-landingpad-cold.ll
Check file: /checkout/src/test/codegen/unwind-landingpad-cold.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
          56:  br label %bb2
          57: 
          58: bb3: ; preds = %cleanup
          59: ; call core::ptr::drop_in_place<alloc::boxed::Box<u32>>
          60:  call void @"_ZN4core3ptr49drop_in_place$LT$alloc..boxed..Box$LT$u32$GT$$GT$17hf6ce08c82e529a7eE"(i32** %x) #4
          61:  br label %bb4
check:9'0      X~~~~~~~~~~~~ error: no match found
check:9'1                    with "ATTRIBUTES" equal to "#4"
          62: 
check:9'0     ~
          63: cleanup: ; preds = %start
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          64:  %2 = landingpad { i8*, i32 }
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          65:  cleanup
check:9'0     ~~~~~~~~
          66:  %3 = bitcast { i8*, i32 }* %1 to i8*
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          67:  call void @llvm.lifetime.start.p0i8(i64 16, i8* %3)
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          68:  %4 = extractvalue { i8*, i32 } %2, 0
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          69:  %5 = extractvalue { i8*, i32 } %2, 1
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          70:  %6 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:9'2                   ?                                                           possible intended match
          71:  store i8* %4, i8** %6, align 8
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          72:  %7 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          73:  store i32 %5, i32* %7, align 8
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          74:  br label %bb3
check:9'0     ~~~~~~~~~~~~~~
          75: 
check:9'0     ~
           .
           .
>>>>>>


------------------------------------------



failures:
    [codegen] codegen/unwind-landingpad-cold.rs
test result: FAILED. 263 passed; 1 failed; 50 ignored; 0 measured; 0 filtered out; finished in 3.26s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:45
