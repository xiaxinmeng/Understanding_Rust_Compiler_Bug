plain
test [codegen] codegen/lto-removes-invokes.rs ... ok

failures:

---- [codegen] codegen/slice-ref-equality.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll" "/checkout/src/test/codegen/slice-ref-equality.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/slice-ref-equality.rs:12:17: error: CHECK-NEXT: expected string not found in input
 // CHECK-NEXT: %[[BCMP:.+]] = tail call i32 @bcmp({{.+}})
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:12:2: note: scanning from here
 %_10.i.i.i = tail call i32 @memcmp(i8* nonnull dereferenceable(4) %0, i8* nonnull dereferenceable(4) getelementptr inbounds (<{ [4 x i8] }>, <{ [4 x i8] }>* @alloc2, i32 0, i32 0, i32 0), i32 4) #2
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll:12:8: note: possible intended match here
 %_10.i.i.i = tail call i32 @memcmp(i8* nonnull dereferenceable(4) %0, i8* nonnull dereferenceable(4) getelementptr inbounds (<{ [4 x i8] }>, <{ [4 x i8] }>* @alloc2, i32 0, i32 0, i32 0), i32 4) #2


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/slice-ref-equality/slice-ref-equality.ll
Check file: /checkout/src/test/codegen/slice-ref-equality.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
           7: 
           8: ; Function Attrs: nounwind readonly uwtable
           9: define zeroext i1 @is_zero_slice([4 x i8]* noalias nocapture readonly align 1 dereferenceable(4) %data) unnamed_addr #0 {
          10: start:
          11:  %0 = getelementptr [4 x i8], [4 x i8]* %data, i32 0, i32 0
          12:  %_10.i.i.i = tail call i32 @memcmp(i8* nonnull dereferenceable(4) %0, i8* nonnull dereferenceable(4) getelementptr inbounds (<{ [4 x i8] }>, <{ [4 x i8] }>* @alloc2, i32 0, i32 0, i32 0), i32 4) #2
next:12'0      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
next:12'1            ?                                                                                                                                                                                               possible intended match
          13:  %1 = icmp eq i32 %_10.i.i.i, 0
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          14:  ret i1 %1
next:12'0     ~~~~~~~~~~
          15: }
next:12'0     ~
          16: 
next:12'0     ~
          17: ; Function Attrs: nofree nounwind readonly uwtable
next:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>

---
test result: FAILED. 172 passed; 1 failed; 65 ignored; 0 measured; 0 filtered out; finished in 1.64s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-wasm32-unknown-emscripten" "--suite" "codegen" "--mode" "codegen" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/latest/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.0-rust-1.50.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --host= --target wasm32-unknown-emscripten --exclude library/core --exclude library/alloc --exclude library/proc_macro --exclude library/std --exclude library/term --exclude library/test
Build completed unsuccessfully in 0:32:33
