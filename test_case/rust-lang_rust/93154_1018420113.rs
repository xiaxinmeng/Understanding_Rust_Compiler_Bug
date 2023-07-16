plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..............
failures:

---- [codegen] codegen/debuginfo-generic-closure-env-names.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll" "/checkout/src/test/codegen/debuginfo-generic-closure-env-names.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/debuginfo-generic-closure-env-names.rs:38:13: error: NONMSVC: expected string not found in input
// NONMSVC: !DICompositeType(tag: DW_TAG_structure_type, name: "{async_block_env#0}<debuginfo_generic_closure_env_names::Foo>", scope: [[generic_async_block_NAMESPACE]]
            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll:1501:97: note: scanning from here
!218 = !DICompositeType(tag: DW_TAG_structure_type, name: "{async_block_env#0}<u32>", scope: !23, file: !2, size: 64, align: 32, elements: !219, identifier: "6e4b5fcff61df599b0181bd401308912")
                                                                                                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll:1501:97: note: with "generic_async_block_NAMESPACE" equal to "!23"
!218 = !DICompositeType(tag: DW_TAG_structure_type, name: "{async_block_env#0}<u32>", scope: !23, file: !2, size: 64, align: 32, elements: !219, identifier: "6e4b5fcff61df599b0181bd401308912")
                                                                                                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll:1530:1: note: possible intended match here
!247 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<debuginfo_generic_closure_env_names::Foo>", scope: !16, file: !2, align: 8, elements: !248, templateParams: !29, identifier: "963fd205104ba59a799b1af50fafed39")


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll
Check file: /checkout/src/test/codegen/debuginfo-generic-closure-env-names.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
         1496: !213 = !DISubroutineType(types: !214)
         1497: !214 = !{!215, !113}
         1498: !215 = !DICompositeType(tag: DW_TAG_structure_type, name: "GenFuture<debuginfo_generic_closure_env_names::generic_async_block::{async_block_env#0}<u32>>", scope: !135, file: !2, size: 64, align: 32, elements: !216, templateParams: !231, identifier: "75402af6befb5949b37d1c25690f0fe5")
         1499: !216 = !{!217}
         1500: !217 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !215, file: !2, baseType: !218, size: 64, align: 32)
         1501: !218 = !DICompositeType(tag: DW_TAG_structure_type, name: "{async_block_env#0}<u32>", scope: !23, file: !2, size: 64, align: 32, elements: !219, identifier: "6e4b5fcff61df599b0181bd401308912")
check:38'0                                                                                                     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:38'1                                                                                                                                                                                                      with "generic_async_block_NAMESPACE" equal to "!23"
         1502: !219 = !{!220}
check:38'0     ~~~~~~~~~~~~~~
         1503: !220 = !DICompositeType(tag: DW_TAG_variant_part, scope: !23, file: !2, size: 64, align: 32, elements: !221, templateParams: !29, identifier: "6e4b5fcff61df599b0181bd401308912_variant_part", discriminator: !230)
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1504: !221 = !{!222, !226, !228}
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
         1505: !222 = !DIDerivedType(tag: DW_TAG_member, name: "0", scope: !220, file: !12, line: 71, baseType: !223, size: 64, align: 32, extraData: i64 0)
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1506: !223 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unresumed", scope: !218, file: !2, size: 64, align: 32, elements: !224, templateParams: !29, identifier: "6e4b5fcff61df599b0181bd401308912::Unresumed")
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
         1525: !242 = !{!243, !245, !250, !252, !254, !256}
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1526: !243 = !DILocalVariable(name: "_closure_u32", scope: !244, file: !12, line: 77, type: !110, align: 4)
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1527: !244 = distinct !DILexicalBlock(scope: !239, file: !12, line: 77, column: 5)
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1528: !245 = !DILocalVariable(name: "_closure_foo", scope: !246, file: !12, line: 78, type: !247, align: 1)
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1529: !246 = distinct !DILexicalBlock(scope: !244, file: !12, line: 78, column: 5)
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1530: !247 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<debuginfo_generic_closure_env_names::Foo>", scope: !16, file: !2, align: 8, elements: !248, templateParams: !29, identifier: "963fd205104ba59a799b1af50fafed39")
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'2     ?                                                                                                                                                                                                                                           possible intended match
         1531: !248 = !{!249}
check:38'0     ~~~~~~~~~~~~~~
         1532: !249 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !247, file: !2, baseType: !43, align: 8)
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1533: !250 = !DILocalVariable(name: "_async_fn_u32", scope: !251, file: !12, line: 80, type: !162, align: 4)
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1534: !251 = distinct !DILexicalBlock(scope: !246, file: !12, line: 80, column: 5)
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1535: !252 = !DILocalVariable(name: "_async_fn_foo", scope: !253, file: !12, line: 81, type: !134, align: 1)
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>

---
test result: FAILED. 262 passed; 1 failed; 51 ignored; 0 measured; 0 filtered out; finished in 3.57s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:56
