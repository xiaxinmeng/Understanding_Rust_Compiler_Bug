plain
.................................................................................................... 9400/11718
.................................................................................................... 9500/11718
............................................................i......i................................ 9600/11718
.................................................................................................... 9700/11718
......iiiiiii..iiiiii..i............................................................................ 9800/11718
.................................................................................................... 10000/11718
.................................................................................................... 10100/11718
.................................................................................................... 10200/11718
.................................................................................................... 10300/11718
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 264 tests
iiF..FiFi.........iii........ii.....i.iii.........i.ii.........i.i..............i.............ii...i 100/264
ii........iiiiii.i........i.....i...iiii........i.i.i...iii..iiii................................... 200/264
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..i.ii...i..ii..i....................iiiii.ii...................

---- [codegen] codegen/abi-efiapi.rs#aarch64 stdout ----


error in revision `aarch64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-10/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.aarch64/abi-efiapi.ll" "/checkout/src/test/codegen/abi-efiapi.rs" "--check-prefixes" "CHECK,aarch64"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/abi-efiapi.rs:26:12: error: aarch64: expected string not found in input
//aarch64: define void @has_efiapi
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.aarch64/abi-efiapi.ll:1:1: note: scanning from here
; ModuleID = 'abi_efiapi.3a1fbbbh-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.aarch64/abi-efiapi.ll:7:11: note: possible intended match here
define dso_local void @has_efiapi() unnamed_addr #0 {

------------------------------------------



---- [codegen] codegen/abi-efiapi.rs#riscv stdout ----

error in revision `riscv`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-10/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.riscv/abi-efiapi.ll" "/checkout/src/test/codegen/abi-efiapi.rs" "--check-prefixes" "CHECK,riscv"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/abi-efiapi.rs:28:10: error: riscv: expected string not found in input
//riscv: define void @has_efiapi
         ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.riscv/abi-efiapi.ll:1:1: note: scanning from here
; ModuleID = 'abi_efiapi.3a1fbbbh-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.riscv/abi-efiapi.ll:7:11: note: possible intended match here
define dso_local void @has_efiapi() unnamed_addr #0 {

------------------------------------------



---- [codegen] codegen/abi-efiapi.rs#arm stdout ----

error in revision `arm`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-10/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.arm/abi-efiapi.ll" "/checkout/src/test/codegen/abi-efiapi.rs" "--check-prefixes" "CHECK,arm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/abi-efiapi.rs:27:8: error: arm: expected string not found in input
//arm: define void @has_efiapi
       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.arm/abi-efiapi.ll:1:1: note: scanning from here
; ModuleID = 'abi_efiapi.3a1fbbbh-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.arm/abi-efiapi.ll:7:11: note: possible intended match here
define dso_local void @has_efiapi() unnamed_addr #0 {

------------------------------------------


---
test result: FAILED. 200 passed; 3 failed; 61 ignored; 0 measured; 0 filtered out; finished in 2.80s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:33
