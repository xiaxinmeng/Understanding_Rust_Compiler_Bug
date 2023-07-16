plain
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 30 tests
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
iiiiiiiiiiiiiii.F.............


---- [run-make] run-make/raw-dylib-stdcall stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
cc -ffunction-sections -fdata-sections -fPIC -m64 -c -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/raw-dylib-stdcall/raw-dylib-stdcall"/extern.obj extern.c
Makefile:8: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
extern.c:9:1: warning: return type defaults to 'int' [-Wimplicit-int]
extern.c:9:1: warning: return type defaults to 'int' [-Wimplicit-int]
 __declspec(dllexport) void __stdcall stdcall_fn_1(int i) {
extern.c: In function '__declspec':
extern.c: In function '__declspec':
extern.c:9:38: error: expected '=', ',', ';', 'asm' or '__attribute__' before 'stdcall_fn_1'
 __declspec(dllexport) void __stdcall stdcall_fn_1(int i) {
                                      ^~~~~~~~~~~~
extern.c:14:1: error: expected declaration specifiers before '__declspec'
 __declspec(dllexport) void __stdcall stdcall_fn_2(uint8_t i, float f) {
 ^~~~~~~~~~
extern.c:19:1: error: expected declaration specifiers before '__declspec'
 __declspec(dllexport) void __stdcall stdcall_fn_3(double d) {
 ^~~~~~~~~~
extern.c:24:1: error: expected declaration specifiers before '__declspec'
 __declspec(dllexport) void __stdcall stdcall_fn_4(uint8_t i, uint8_t j, float f) {
 ^~~~~~~~~~
extern.c:29:1: error: expected declaration specifiers before '__declspec'
 __declspec(dllexport) void __stdcall stdcall_fn_5(struct S s, int i) {
 ^~~~~~~~~~
extern.c:34:1: error: expected declaration specifiers before '__declspec'
 __declspec(dllexport) void __stdcall stdcall_fn_6(struct S* s) {
 ^~~~~~~~~~
extern.c:9:1: warning: type of 'dllexport' defaults to 'int' [-Wimplicit-int]
 __declspec(dllexport) void __stdcall stdcall_fn_1(int i) {
 ^~~~~~~~~~
extern.c:41:1: error: expected '{' at end of input
 ^
 ^
make: *** [all] Error 1
------------------------------------------




failures:
    [run-make] run-make/raw-dylib-stdcall

test result: FAILED. 14 passed; 1 failed; 15 ignored; 0 measured; 0 filtered out; finished in 2.88s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-10/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:33
