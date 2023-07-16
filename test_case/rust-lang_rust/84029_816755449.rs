plain
.................................................................................................... 9400/11747
.................................................................................................... 9500/11747
...................................................................................i......i......... 9600/11747
.................................................................................................... 9700/11747
.............................iiiiiii..iiiiii.i...................................................... 9800/11747
.................................................................................................... 10000/11747
.................................................................................................... 10100/11747
.................................................................................................... 10200/11747
.................................................................................................... 10300/11747
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.089 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 10.152 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.390 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 25 tests
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
iiiiiiiiiiiii...F........

---- [run-make] run-make/env-dep-info stdout ----

error: make failed
error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
EXISTING_ENV=1 EXISTING_OPT_ENV=1 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info  --emit dep-info main.rs
"/checkout/src/etc/cat-and-grep.sh" "# env-dep:EXISTING_ENV=1" < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info/main.d
[[[ begin stdout ]]]
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info/main.d: main.rs
main.rs:


# env-dep:ESCAPE\nESCAPE\\
# env-dep:EXISTING_ENV=1
# env-dep:EXISTING_OPT_ENV=1
# env-dep:NONEXISTENT_OPT_ENV

[[[ end stdout ]]]
"/checkout/src/etc/cat-and-grep.sh" "# env-dep:EXISTING_OPT_ENV=1" < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info/main.d
[[[ begin stdout ]]]
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info/main.d: main.rs
main.rs:


# env-dep:ESCAPE\nESCAPE\\
# env-dep:EXISTING_ENV=1
# env-dep:EXISTING_OPT_ENV=1
# env-dep:NONEXISTENT_OPT_ENV

[[[ end stdout ]]]
"/checkout/src/etc/cat-and-grep.sh" "# env-dep:NONEXISTENT_OPT_ENV" < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info/main.d
[[[ begin stdout ]]]
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info/main.d: main.rs
main.rs:


# env-dep:ESCAPE\nESCAPE\\
# env-dep:EXISTING_ENV=1
# env-dep:EXISTING_OPT_ENV=1
# env-dep:NONEXISTENT_OPT_ENV

[[[ end stdout ]]]
"/checkout/src/etc/cat-and-grep.sh" "# env-dep:ESCAPE\nESCAPE\\" < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info/main.d
[[[ begin stdout ]]]
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info/main.d: main.rs
main.rs:


# env-dep:ESCAPE\nESCAPE\\
# env-dep:EXISTING_ENV=1
# env-dep:EXISTING_OPT_ENV=1
# env-dep:NONEXISTENT_OPT_ENV

[[[ end stdout ]]]
# Proc macro
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc'  --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/env-dep-info/env-dep-info macro_def.rs
Makefile:10: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error[E0425]: cannot find function `path` in module `tracked_env`
  --> macro_def.rs:16:26
   |
16 |     let _ = tracked_env::path("./Makefile");
   |                          ^^^^ not found in `tracked_env`
help: consider importing this function
   |
5  | use tracked_path::path;
   |
   |

error[E0425]: cannot find function `path` in module `tracked_env`
  --> macro_def.rs:17:26
   |
17 |     let _ = tracked_env::path("./non-existent");
   |                          ^^^^ not found in `tracked_env`
help: consider importing this function
   |
5  | use tracked_path::path;
   |
   |

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
make: *** [all] Error 1
------------------------------------------




failures:
    [run-make] run-make/env-dep-info

test result: FAILED. 11 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out; finished in 3.05s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-10/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:33:55
