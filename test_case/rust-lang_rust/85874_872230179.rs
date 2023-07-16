plain
failures:

---- [codegen] codegen/vec-in-place.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll" "/checkout/src/test/codegen/vec-in-place.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/vec-in-place.rs:11:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: loop
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll:29:51: note: found here
 br i1 %lcmp.mod.not, label %bb6.i.i.i.i.i.i.prol.loopexit, label %bb6.i.i.i.i.i.i.prol

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vec-in-place/vec-in-place.ll
Check file: /checkout/src/test/codegen/vec-in-place.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       24:  %3 = add i64 %2, -8
       25:  %4 = lshr exact i64 %3, 3
       26:  %5 = add nuw nsw i64 %4, 1
       27:  %xtraiter = and i64 %5, 7
       28:  %lcmp.mod.not = icmp eq i64 %xtraiter, 0
       29:  br i1 %lcmp.mod.not, label %bb6.i.i.i.i.i.i.prol.loopexit, label %bb6.i.i.i.i.i.i.prol
not:11                                                       !~~~                                  error: no match expected
       30: 
       31: bb6.i.i.i.i.i.i.prol: ; preds = %bb6.i.i.i.i.i.i.preheader, %bb6.i.i.i.i.i.i.prol
       32:  %_4.i32.i.i.i.i.i.i.prol = phi i64* [ %6, %bb6.i.i.i.i.i.i.prol ], [ %_4.sroa.0.0.copyload, %bb6.i.i.i.i.i.i.preheader ]
       33:  %prol.iter = phi i64 [ %prol.iter.sub, %bb6.i.i.i.i.i.i.prol ], [ %xtraiter, %bb6.i.i.i.i.i.i.preheader ]
       34:  %6 = getelementptr inbounds i64, i64* %_4.i32.i.i.i.i.i.i.prol, i64 1
        .
        .
>>>>>>

---
test result: FAILED. 241 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out; finished in 4.07s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v15.14.0-linux-x64/bin/node" "--linker" "x86_64-linux-musl-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target x86_64-unknown-linux-musl
Build completed unsuccessfully in 0:04:22
