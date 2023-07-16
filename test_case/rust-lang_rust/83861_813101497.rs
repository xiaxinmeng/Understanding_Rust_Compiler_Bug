plain
.................................................................................................... 9400/11731
.................................................................................................... 9500/11731
........................................................................i......i.................... 9600/11731
.................................................................................................... 9700/11731
..................iiiiiii..iiiiii.i................................................................. 9800/11731
.................................................................................................... 10000/11731
.................................................................................................... 10100/11731
.................................................................................................... 10200/11731
.................................................................................................... 10300/11731
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.04s

 finished in 0.094 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.39s

 finished in 2.443 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 23 tests
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
iiiiiiiiiiii..........F

---- [run-make] run-make/emit-shared-files stdout ----

error: make failed
error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -Z unstable-options --emit=invocation-specific --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only --resource-suffix=-xxx --theme y.css --extend-css z.css x.rs
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/search-index-xxx.js ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/settings.html ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/x/all.html ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/x/index.html ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/theme-xxx.css ] # generated from z.css
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/storage-xxx.js ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/SourceSerifPro-It.ttf.woff ]
# FIXME: this probably shouldn't have a suffix
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/y-xxx.css ]
# FIXME: this is technically incorrect (see `write_shared`)
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/invocation-only/main-xxx.js ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -Z unstable-options --emit=toolchain-shared-resources --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only --resource-suffix=-xxx --extend-css z.css x.rs
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/storage-xxx.js ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/SourceSerifPro-It.ttf.woff ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/search-index-xxx.js ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/x/index.html ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/theme.css ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/main-xxx.js ]
! [ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/toolchain-only/y-xxx.css ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -Z unstable-options --emit=toolchain-shared-resources,unversioned-shared-resources --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/all-shared --resource-suffix=-xxx --extend-css z.css x.rs
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/all-shared/storage-xxx.js ]
[ -e /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-shared-files/emit-shared-files/all-shared/SourceSerifPro-It.ttf.woff ]
Makefile:36: recipe for target 'all-shared' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
warning: theme file "y.css" is missing CSS rules from the default theme
  |
  = warning: the theme may appear incorrect when loaded
  = help: to see what rules are missing, call `rustdoc  --check-theme "y.css"`

make: *** [all-shared] Error 1
------------------------------------------




failures:
    [run-make] run-make/emit-shared-files

test result: FAILED. 10 passed; 1 failed; 12 ignored; 0 measured; 0 filtered out; finished in 2.80s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-10/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:49
