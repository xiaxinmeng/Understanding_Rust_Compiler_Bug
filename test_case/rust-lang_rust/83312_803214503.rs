plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 11702 tests
.................................................................................................... 100/11702
........................................i......FF..ii............................................... 200/11702
.................................................................................................... 400/11702
.................................................................................................... 500/11702
.................................................................................................... 600/11702
.......................i............................................................................ 700/11702
---
.................................................................................................... 9300/11702
.................................................................................................... 9400/11702
.................................................................................................... 9500/11702
.............................................i......i............................................... 9600/11702
...........................................................................................iiiiiii.. 9700/11702
iiiiii.i............................................................................................ 9800/11702
.................................................................................................... 10000/11702
.................................................................................................... 10100/11702
.................................................................................................... 10200/11702
.................................................................................................... 10300/11702
---

---- [ui] ui/asm/inline-syntax.rs#arm stdout ----
diff of stderr:

1 error: att syntax is the default syntax on this target, and trying to use this directive may cause issues
-   --> $DIR/inline-syntax.rs:22:15
3    |
3    |
4 LL |         asm!(".att_syntax noprefix", "nop");
5    |               ^^^^^^^^^^^^^^^^^^^^ help: remove this assembler directive
6 
6 
7 error: att syntax is the default syntax on this target, and trying to use this directive may cause issues
-   --> $DIR/inline-syntax.rs:25:15
9    |
9    |
10 LL |         asm!(".att_syntax bbb noprefix", "nop");
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
11    |               ^^^^^^^^^^^^^^^^^^^^^^^^ help: remove this assembler directive

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.arm/inline-syntax.arm.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/inline-syntax.rs`

error in revision `arm`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/inline-syntax.rs" "-Zthreads=1" "--cfg" "arm" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.arm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "armv7-unknown-linux-gnueabihf" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.arm/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: att syntax is the default syntax on this target, and trying to use this directive may cause issues
   |
   |
LL |         asm!(".att_syntax noprefix", "nop");
   |               ^^^^^^^^^^^^^^^^^^^^ help: remove this assembler directive

error: att syntax is the default syntax on this target, and trying to use this directive may cause issues
   |
   |
LL |         asm!(".att_syntax bbb noprefix", "nop");
   |               ^^^^^^^^^^^^^^^^^^^^^^^^ help: remove this assembler directive
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/asm/inline-syntax.rs#x86_64 stdout ----


1 error: intel syntax is the default syntax on this target, and trying to use this directive may cause issues
-   --> $DIR/inline-syntax.rs:18:15
3    |
3    |
4 LL |         asm!(".intel_syntax noprefix", "nop");
5    |               ^^^^^^^^^^^^^^^^^^^^^^ help: remove this assembler directive
6 
6 
7 error: intel syntax is the default syntax on this target, and trying to use this directive may cause issues
-   --> $DIR/inline-syntax.rs:20:15
9    |
9    |
10 LL |         asm!(".intel_syntax aaa noprefix", "nop");
11    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove this assembler directive
12 
12 
13 error: using the .att_syntax directive may cause issues, use the att_syntax option instead
-   --> $DIR/inline-syntax.rs:22:15
15    |
15    |
16 LL |         asm!(".att_syntax noprefix", "nop");

22    |              --       ^^^^^^^^^^^^^^^^^^^^^
23 
23 
24 error: using the .att_syntax directive may cause issues, use the att_syntax option instead
-   --> $DIR/inline-syntax.rs:25:15
26    |
26    |
27 LL |         asm!(".att_syntax bbb noprefix", "nop");

33    |              --       ^^^^^^^^^^^^^^^^^^^^^
34 
34 
35 error: intel syntax is the default syntax on this target, and trying to use this directive may cause issues
-   --> $DIR/inline-syntax.rs:28:15
37    |
37    |
38 LL |         asm!(".intel_syntax noprefix; nop");
39    |               ^^^^^^^^^^^^^^^^^^^^^^ help: remove this assembler directive
40 
40 
41 error: intel syntax is the default syntax on this target, and trying to use this directive may cause issues
-   --> $DIR/inline-syntax.rs:33:14
43    |
43    |
44 LL |               .intel_syntax noprefix


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64/inline-syntax.x86_64.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/inline-syntax.rs`

error in revision `x86_64`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/inline-syntax.rs" "-Zthreads=1" "--cfg" "x86_64" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: intel syntax is the default syntax on this target, and trying to use this directive may cause issues
   |
   |
LL |         asm!(".intel_syntax noprefix", "nop");
   |               ^^^^^^^^^^^^^^^^^^^^^^ help: remove this assembler directive

error: intel syntax is the default syntax on this target, and trying to use this directive may cause issues
   |
   |
LL |         asm!(".intel_syntax aaa noprefix", "nop");
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove this assembler directive

error: using the .att_syntax directive may cause issues, use the att_syntax option instead
   |
   |
LL |         asm!(".att_syntax noprefix", "nop");
   |
   |
help: remove the assembler directive and replace it with options(att_syntax)
   |
LL |         asm!("", "nop", options(att_syntax));


error: using the .att_syntax directive may cause issues, use the att_syntax option instead
   |
   |
LL |         asm!(".att_syntax bbb noprefix", "nop");
   |
   |
help: remove the assembler directive and replace it with options(att_syntax)
   |
LL |         asm!("", "nop", options(att_syntax));


error: intel syntax is the default syntax on this target, and trying to use this directive may cause issues
   |
   |
LL |         asm!(".intel_syntax noprefix; nop");
   |               ^^^^^^^^^^^^^^^^^^^^^^ help: remove this assembler directive

error: intel syntax is the default syntax on this target, and trying to use this directive may cause issues
   |
   |
LL |               .intel_syntax noprefix
LL | |             nop"
LL | |             nop"
   | |_ help: remove this assembler directive
error: aborting due to 6 previous errors


------------------------------------------
---
test result: FAILED. 11604 passed; 2 failed; 96 ignored; 0 measured; 0 filtered out; finished in 141.36s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:52
