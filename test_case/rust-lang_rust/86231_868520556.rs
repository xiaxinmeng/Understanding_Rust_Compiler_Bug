plain
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 273 tests
ii.....i...i..i.......iii........ii.....i.iii.........i.ii.........i.i..............i............i.i 100/273
...iii........ii.iiii.i..........i.....i...iiii........i..i.i....iii..iiii.......................... 200/273
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...........i.ii......i...i.................F...iiiii.i.F.................

---- [codegen] codegen/unwind-abis/stdcall-unwind-abi.rs stdout ----

error: compilation failed!
error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/unwind-abis/stdcall-unwind-abi.rs" "-Zthreads=1" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/stdcall-unwind-abi" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-unknown-linux-gnu" "--crate-type=rlib" "-Cno-prepopulate-passes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/stdcall-unwind-abi/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0570]: `"stdcall"` is not a supported ABI for the current target
   |
   |
14 | pub extern "stdcall" fn rust_item_that_cannot_unwind() {


error[E0570]: `"stdcall-unwind"` is not a supported ABI for the current target
   |
   |
19 | pub extern "stdcall-unwind" fn rust_item_that_can_unwind() {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0570`.
---
---- [codegen] codegen/unwind-abis/thiscall-unwind-abi.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/unwind-abis/thiscall-unwind-abi.rs" "-Zthreads=1" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/thiscall-unwind-abi" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-unknown-linux-gnu" "--crate-type=rlib" "-Cno-prepopulate-passes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/unwind-abis/thiscall-unwind-abi/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0570]: `"thiscall"` is not a supported ABI for the current target
   |
   |
14 | pub extern "thiscall" fn rust_item_that_cannot_unwind() {


error[E0570]: `"thiscall-unwind"` is not a supported ABI for the current target
   |
   |
19 | pub extern "thiscall-unwind" fn rust_item_that_can_unwind() {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0570`.
---
test result: FAILED. 212 passed; 2 failed; 59 ignored; 0 measured; 0 filtered out; finished in 3.08s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:12
