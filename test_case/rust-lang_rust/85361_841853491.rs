plain
.................................................................................................... 9500/11877
.................................................................................................... 9600/11877
..............................................................................i.......i............. 9700/11877
.................................................................................................... 9800/11877
.......................iiiiiii..iiiiii.i............................................................ 9900/11877
.................................................................................................... 10100/11877
.................................................................................................... 10200/11877
.................................................................................................... 10300/11877
.................................................................................................... 10400/11877
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 36 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.182 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 3.64s

 finished in 3.712 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 222 tests
..........iii...ii.................................................................................. 100/222
............i..................iiiiii.......i.................iiiF..........ii...................... 200/222
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [run-make] run-make-fulldeps/rustdoc-target-spec-json-path stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path  --crate-type lib -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path" dummy_core.rs --target target.json
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path" my_crate.rs --target target.json -L "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path/rustdoc-target-spec-json-path"
Makefile:8: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
warning: ignoring --out-dir flag due to -o flag
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted

error[E0463]: can't find crate for `dummy_core`
 --> my_crate.rs:3:1
3 | extern crate dummy_core;
  | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
make: *** [all] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/rustdoc-target-spec-json-path
test result: FAILED. 201 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out; finished in 23.92s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-10/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:13
