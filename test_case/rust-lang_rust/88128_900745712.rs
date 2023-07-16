plain
.................................................................................................... 11500/12150
.................................................................................................... 11600/12150
.................................................................................................... 11700/12150
.................................................................................................... 11800/12150
...........................F.F...................................................................... 11900/12150
......................................i.i........................................................... 12100/12150
..................................................
failures:


---- [ui] ui/feature-gates/feature-gate-global_asm.rs stdout ----
diff of stderr:

1 error[E0658]: use of unstable library feature 'global_asm': `global_asm!` is not stable enough for use and is subject to change
-   --> $DIR/feature-gate-global_asm.rs:1:1
3    |
3    |
4 LL | global_asm!("");


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-global_asm/feature-gate-global_asm.stderr
To only update this specific test, also pass `--test-args feature-gates/feature-gate-global_asm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-global_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-global_asm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-global_asm/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'global_asm': `global_asm!` is not stable enough for use and is subject to change
   |
   |
LL | global_asm!(""); //~ ERROR `global_asm!` is not stable
   |
   = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
   = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
   = help: add `#![feature(global_asm)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/unsafe/inline_asm.rs#mir stdout ----
diff of stderr:

1 error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
-   --> $DIR/inline_asm.rs:9:5
3    |
3    |
4 LL |     asm!("nop");
5    |     ^^^^^^^^^^^^ use of inline assembly

7    = note: inline assembly is entirely unchecked and can cause undefined behavior
8 
9 error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
-   --> $DIR/inline_asm.rs:10:5
11    |
11    |
12 LL |     llvm_asm!("nop");
13    |     ^^^^^^^^^^^^^^^^^ use of inline assembly

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.mir/inline_asm.mir.stderr
To only update this specific test, also pass `--test-args unsafe/inline_asm.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
   |
   |
LL |     asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |     ^^^^^^^^^^^^ use of inline assembly
   |
   = note: inline assembly is entirely unchecked and can cause undefined behavior

error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
   |
   |
LL |     llvm_asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |     ^^^^^^^^^^^^^^^^^ use of inline assembly
   |
   = note: inline assembly is entirely unchecked and can cause undefined behavior
   = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/unsafe/inline_asm.rs#thir stdout ----
diff of stderr:

1 error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
-   --> $DIR/inline_asm.rs:9:5
3    |
3    |
4 LL |     asm!("nop");
5    |     ^^^^^^^^^^^^ use of inline assembly

7    = note: inline assembly is entirely unchecked and can cause undefined behavior
8 
9 error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
-   --> $DIR/inline_asm.rs:10:5
11    |
11    |
12 LL |     llvm_asm!("nop");
13    |     ^^^^^^^^^^^^^^^^^ use of inline assembly

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.thir/inline_asm.thir.stderr
To only update this specific test, also pass `--test-args unsafe/inline_asm.rs`


error in revision `thir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.thir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/inline_asm.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
   |
   |
LL |     asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |     ^^^^^^^^^^^^ use of inline assembly
   |
   = note: inline assembly is entirely unchecked and can cause undefined behavior

error[E0133]: use of inline assembly is unsafe and requires unsafe function or block
   |
   |
LL |     llvm_asm!("nop"); //~ ERROR use of inline assembly is unsafe and requires unsafe function or block
   |     ^^^^^^^^^^^^^^^^^ use of inline assembly
   |
   = note: inline assembly is entirely unchecked and can cause undefined behavior
   = note: this error originates in the macro `llvm_asm` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0133`.

---
test result: FAILED. 12046 passed; 3 failed; 101 ignored; 0 measured; 0 filtered out; finished in 129.23s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:20
