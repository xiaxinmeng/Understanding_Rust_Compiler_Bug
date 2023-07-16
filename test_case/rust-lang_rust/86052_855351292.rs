plain
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 273 tests
ii.....i...i..i.......iii........ii.....i..iii........i.ii.........i.i.....F........i............i.i 100/273
...iii........ii.iiii.i..........i.....i...iiii........i..i.i....iii..iiii.......................... 200/273
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [codegen] codegen/consts.rs stdout ----


error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-10/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll" "/checkout/src/test/codegen/consts.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/consts.rs:13:11: error: CHECK: expected string not found in input
// CHECK: @alloc8 = {{.*}}, align 2
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:11:1: note: scanning from here
@alloc7 = private unnamed_addr constant <{ [4 x i8] }> zeroinitializer, align 2
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:11:64: note: possible intended match here
@alloc7 = private unnamed_addr constant <{ [4 x i8] }> zeroinitializer, align 2
                                                               ^
/checkout/src/test/codegen/consts.rs:46:12: error: CHECK: expected string not found in input
 // CHECK: memcpy.p0i8.p0i8.i{{(32|64)}}(i8* align 2 %1, i8* align 2 getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* [[LOW_HIGH]], i32 0, i32 0, i32 0), i{{(32|64)}} 8, i1 false)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:37:28: note: scanning from here
define i64 @low_align_const() unnamed_addr #0 {
                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:37:28: note: uses undefined variable(s): "LOW_HIGH"
define i64 @low_align_const() unnamed_addr #0 {
                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:41:18: note: possible intended match here
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 2 %1, i8* align 2 getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* @alloc15, i32 0, i32 0, i32 0), i64 8, i1 false)
                 ^
/checkout/src/test/codegen/consts.rs:54:12: error: CHECK: expected string not found in input
 // CHECK: memcpy.p0i8.p0i8.i{{(32|64)}}(i8* align 4 %1, i8* align 4 getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* [[LOW_HIGH]], i32 0, i32 0, i32 0), i{{(32|64)}} 8, i1 false)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:48:29: note: scanning from here
define i64 @high_align_const() unnamed_addr #0 {
                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:48:29: note: uses undefined variable(s): "LOW_HIGH"
define i64 @high_align_const() unnamed_addr #0 {
                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:52:18: note: possible intended match here
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* @alloc15, i32 0, i32 0, i32 0), i64 8, i1 false)

------------------------------------------


---
test result: FAILED. 212 passed; 1 failed; 60 ignored; 0 measured; 0 filtered out; finished in 3.10s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:52
