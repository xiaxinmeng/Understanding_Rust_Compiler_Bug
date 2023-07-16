plain
Successfully built 14e5bfdc96ca
Successfully tagged rust-ci:latest
Built container sha256:14e5bfdc96ca56ea12bdc1f48ccc60705584d7a5451a2afe8bec755dd5d32ab5
Uploading finished image to https://ci-caches.rust-lang.org/docker/b8522bdbde0170e6b8638c2ee7936d2221f0e051b4634ed61ad299a1775319b857fabeb7dc5d6fa62739b362a15aa7c74c4f64f6f12f1718e9d3e82ccdcf01ba
upload failed: - to s3://rust-lang-ci-sccache2/docker/b8522bdbde0170e6b8638c2ee7936d2221f0e051b4634ed61ad299a1775319b857fabeb7dc5d6fa62739b362a15aa7c74c4f64f6f12f1718e9d3e82ccdcf01ba Unable to locate credentials
 * branch              master     -> FETCH_HEAD
[CI_JOB_NAME=x86_64-gnu-llvm-10]
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
failures:

---- [codegen] codegen/consts.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-10/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll" "/checkout/src/test/codegen/consts.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/consts.rs:13:11: error: CHECK: expected string not found in input
// CHECK: @alloc8 = {{.*}}, align 2
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:11:1: note: scanning from here
@alloc9 = private unnamed_addr constant <{ [4 x i8] }> zeroinitializer, align 2
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:11:64: note: possible intended match here
@alloc9 = private unnamed_addr constant <{ [4 x i8] }> zeroinitializer, align 2
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
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 2 %1, i8* align 2 getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* @alloc17, i32 0, i32 0, i32 0), i64 8, i1 false)
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
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 getelementptr inbounds (<{ [8 x i8] }>, <{ [8 x i8] }>* @alloc17, i32 0, i32 0, i32 0), i64 8, i1 false)

------------------------------------------



---- [codegen] codegen/remap_path_prefix/main.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-10/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll" "/checkout/src/test/codegen/remap_path_prefix/main.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/remap_path_prefix/main.rs:15:11: error: CHECK: expected string not found in input
// CHECK: @alloc1 = private unnamed_addr constant <{ [34 x i8] }> <{ [34 x i8] c"/the/src/remap_path_prefix/main.rs" }>, align 1
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:1:1: note: scanning from here
; ModuleID = 'main.cbc7ddee-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:10:1: note: possible intended match here
@alloc2 = private unnamed_addr constant <{ [34 x i8] }> <{ [34 x i8] c"/the/src/remap_path_prefix/main.rs" }>, align 1

------------------------------------------


---
test result: FAILED. 219 passed; 2 failed; 59 ignored; 0 measured; 0 filtered out; finished in 2.78s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:33
