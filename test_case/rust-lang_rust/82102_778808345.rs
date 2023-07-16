plain
.................................................................................................... 9200/11445
.................................................................................................... 9300/11445
.................................................................................................... 9400/11445
....i......i........................................................................................ 9500/11445
..........................................iiiiiii...iiiiiii......................................... 9600/11445
.................................................................................................... 9800/11445
.................................................................................................... 9900/11445
.................................................................................................... 10000/11445
.................................................................................................... 10100/11445
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.077 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.22s

 finished in 2.294 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 217 tests
........iiii...ii................................................................................... 100/217
..........i..................iiiiii......i...............iii...............F.....F.................i 200/217
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [run-make] run-make-fulldeps/split-dwarf stdout ----
---- [run-make] run-make-fulldeps/split-dwarf stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf  -Z unstable-options -C split-debuginfo=packed foo.rs -g
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf/foo.dwp
Makefile:6: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: linking dwarf objects with `rust-llvm-dwp` failed: exit code: 1
  |
  = note: "rust-llvm-dwp" "-e" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf/foo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf/foo.dwp"
  = note: error: No such file or directory
          

error: aborting due to previous error
error: aborting due to previous error

rm: cannot remove '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf/foo.dwp': No such file or directory
make: *** [all] Error 1
------------------------------------------


---- [run-make] run-make-fulldeps/split-debuginfo stdout ----
---- [run-make] run-make-fulldeps/split-debuginfo stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  foo.rs -g -C split-debuginfo=off -Z unstable-options
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp ]
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  foo.rs -g
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp ]
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  foo.rs -g -C split-debuginfo=packed -Z unstable-options
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp
Makefile:48: recipe for target 'packed' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: linking dwarf objects with `rust-llvm-dwp` failed: exit code: 1
  |
  = note: "rust-llvm-dwp" "-e" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo.dwp"
  = note: error: No such file or directory
          

error: aborting due to previous error
error: aborting due to previous error

ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp': No such file or directory
make: *** [packed] Error 2
------------------------------------------



---
test result: FAILED. 196 passed; 2 failed; 19 ignored; 0 measured; 0 filtered out; finished in 20.55s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-9/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:36
