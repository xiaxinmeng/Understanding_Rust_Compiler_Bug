plain
Successfully built 3ebaac8ac3f4
Successfully tagged rust-ci:latest
Built container sha256:3ebaac8ac3f44adebfb0d8f96ac9c0931d96a1d651213bed1781c2f648f7de59
Uploading finished image to https://ci-caches.rust-lang.org/docker/dfd7203a0b015711c96f25420d9cb51dd6d72a416dd27c32932eb6b4d7efea10392bba63f0eaa514ea019391488096f30c8a7ead06c758f8f033ddf38b7029a7
upload failed: - to s3://rust-lang-ci-sccache2/docker/dfd7203a0b015711c96f25420d9cb51dd6d72a416dd27c32932eb6b4d7efea10392bba63f0eaa514ea019391488096f30c8a7ead06c758f8f033ddf38b7029a7 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 296 tests
ii.....iii...i...i..i.......iii.........ii.....i.iii..........i.ii..............iFi..............i.. 100/296
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...........................i.ii......ii...i...........................iiiii.i..............F....
failures:


---- [codegen] codegen/debug-vtable.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-10/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debug-vtable/debug-vtable.ll" "/checkout/src/test/codegen/debug-vtable.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/debug-vtable.rs:11:11: error: CHECK: expected string not found in input
// CHECK: !DISubrange(count: 5, lowerBound: 0)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debug-vtable/debug-vtable.ll:127:66: note: scanning from here
!4 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !5, size: 64, align: 64, dwarfAddressSpace: 0)
                                                                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debug-vtable/debug-vtable.ll:130:6: note: possible intended match here
!7 = !DISubrange(count: 5)
     ^
/checkout/src/test/codegen/debug-vtable.rs:15:11: error: CHECK: expected string not found in input
// CHECK: !DISubrange(count: 4, lowerBound: 0)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debug-vtable/debug-vtable.ll:132:118: note: scanning from here
!9 = distinct !DIGlobalVariable(name: "<debug_vtable::Foo as debug_vtable::SomeTraitWithGenerics<u64, i8>>::{vtable}", scope: null, file: !2, type: !10, isLocal: true, isDefinition: true)
                                                                                                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debug-vtable/debug-vtable.ll:135:7: note: possible intended match here
!12 = !DISubrange(count: 4)
      ^
/checkout/src/test/codegen/debug-vtable.rs:19:11: error: CHECK: expected string not found in input
// CHECK: !DISubrange(count: 3, lowerBound: 0)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debug-vtable/debug-vtable.ll:137:76: note: scanning from here
!14 = distinct !DIGlobalVariable(name: "<debug_vtable::Foo as _>::{vtable}", scope: null, file: !2, type: !15, isLocal: true, isDefinition: true)
                                                                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debug-vtable/debug-vtable.ll:140:7: note: possible intended match here
!17 = !DISubrange(count: 3)

------------------------------------------



---- [codegen] codegen/vtabletype.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-10/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vtabletype/vtabletype.ll" "/checkout/src/test/codegen/vtabletype.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/vtabletype.rs:10:11: error: CHECK: expected string not found in input
// CHECK: {{.*}}DICompositeType{{.*}}name: "vtable",{{.*}}vtableHolder:{{.*}}
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vtabletype/vtabletype.ll:320:17: note: scanning from here
define i32 @main(i32 %0, i8** %1) unnamed_addr #5 {
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/vtabletype/vtabletype.ll:343:1: note: possible intended match here
!3 = !DICompositeType(tag: DW_TAG_array_type, baseType: !4, size: 384, align: 64, elements: !6)

------------------------------------------


---
test result: FAILED. 231 passed; 2 failed; 63 ignored; 0 measured; 0 filtered out; finished in 3.44s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:53
