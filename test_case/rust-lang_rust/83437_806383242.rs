plain
.................................................................................................... 9400/11714
.................................................................................................... 9500/11714
........................................................i.....i..................................... 9600/11714
.................................................................................................... 9700/11714
.iiiiiii..iiiiii.i.................................................................................. 9800/11714
.................................................................................................... 10000/11714
.................................................................................................... 10100/11714
.................................................................................................... 10200/11714
.................................................................................................... 10300/11714
---
.....i.i............................................................................................ 11700/11714
..............
failures:

---- [ui] ui/asm/inline-syntax.rs#x86_64 stdout ----


- warning: avoid using `.intel_syntax`, intel syntax is the default
+ warning: avoid using `.intel_syntax`, Intel syntax is the default
3    |
3    |
4 LL |         asm!(".intel_syntax noprefix", "nop");
6    |
6    |
7    = note: `#[warn(bad_asm_style)]` on by default
8 
- warning: avoid using `.intel_syntax`, intel syntax is the default
+ warning: avoid using `.intel_syntax`, Intel syntax is the default
11    |
11    |
12 LL |         asm!(".intel_syntax aaa noprefix", "nop");

24 LL |         asm!(".att_syntax bbb noprefix", "nop");
26 
26 
- warning: avoid using `.intel_syntax`, intel syntax is the default
+ warning: avoid using `.intel_syntax`, Intel syntax is the default
29    |
29    |
30 LL |         asm!(".intel_syntax noprefix; nop");
31    |               ^^^^^^^^^^^^^^^^^^^^^^
32 
32 
- warning: avoid using `.intel_syntax`, intel syntax is the default
+ warning: avoid using `.intel_syntax`, Intel syntax is the default
35    |
35    |
36 LL |             .intel_syntax noprefix

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64/inline-syntax.x86_64.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/inline-syntax.rs`

error in revision `x86_64`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/inline-syntax.rs" "-Zthreads=1" "--cfg" "x86_64" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |         asm!(".intel_syntax noprefix", "nop");
   |
   |
   = note: `#[warn(bad_asm_style)]` on by default

warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |         asm!(".intel_syntax aaa noprefix", "nop");


warning: avoid using `.att_syntax`, prefer using `options(att_syntax)` instead
   |
   |
LL |         asm!(".att_syntax noprefix", "nop");


warning: avoid using `.att_syntax`, prefer using `options(att_syntax)` instead
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |         asm!(".att_syntax bbb noprefix", "nop");


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |         asm!(".intel_syntax noprefix; nop");


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |             .intel_syntax noprefix

warning: 6 warnings emitted


---
test result: FAILED. 11617 passed; 1 failed; 96 ignored; 0 measured; 0 filtered out; finished in 134.53s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:07
