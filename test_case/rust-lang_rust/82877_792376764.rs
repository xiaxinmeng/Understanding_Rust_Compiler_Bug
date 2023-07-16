plain

142 LL | invalid_instruction
143    | ^^^^^^^^^^^^^^^^^^^
144 
- error: invalid instruction mnemonic 'invalid_instruction1'
-   --> $DIR/srcloc.rs:79:14
- LL |             "invalid_instruction1",
-    |              ^
-    |
- note: instantiated into assembly here
- note: instantiated into assembly here
-   --> <inline asm>:2:2
- LL |     invalid_instruction1
-    |     ^^^^^^^^^^^^^^^^^^^^
- 
- 
- error: invalid instruction mnemonic 'invalid_instruction2'
-   --> $DIR/srcloc.rs:80:14
- LL |             "invalid_instruction2",
-    |              ^
-    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- note: instantiated into assembly here
-   --> <inline asm>:3:1
- LL | invalid_instruction2
-    | ^^^^^^^^^^^^^^^^^^^^
- 
- 
- error: invalid instruction mnemonic 'invalid_instruction1'
-   --> $DIR/srcloc.rs:86:13
- LL |             concat!(
-    |             ^
-    |
- note: instantiated into assembly here
- note: instantiated into assembly here
-   --> <inline asm>:2:2
- LL |     invalid_instruction1
-    |     ^^^^^^^^^^^^^^^^^^^^
- 
- 
- error: invalid instruction mnemonic 'invalid_instruction2'
-   --> $DIR/srcloc.rs:86:13
- LL |             concat!(
-    |             ^
-    |
- note: instantiated into assembly here
- note: instantiated into assembly here
-   --> <inline asm>:3:1
- LL | invalid_instruction2
-    | ^^^^^^^^^^^^^^^^^^^^
- 
- 
- error: invalid instruction mnemonic 'invalid_instruction1'
-   --> $DIR/srcloc.rs:95:13
- LL |             concat!(
-    |             ^
-    |
- note: instantiated into assembly here
- note: instantiated into assembly here
-   --> <inline asm>:2:2
- LL |     invalid_instruction1
-    |     ^^^^^^^^^^^^^^^^^^^^
- 
- 
- error: invalid instruction mnemonic 'invalid_instruction2'
-   --> $DIR/srcloc.rs:95:13
- LL |             concat!(
-    |             ^
-    |
- note: instantiated into assembly here
- note: instantiated into assembly here
-   --> <inline asm>:3:1
- LL | invalid_instruction2
-    | ^^^^^^^^^^^^^^^^^^^^
- 
- 
- error: invalid instruction mnemonic 'invalid_instruction3'
-   --> $DIR/srcloc.rs:99:13
- LL |             concat!(
-    |             ^
-    |
- note: instantiated into assembly here
- note: instantiated into assembly here
-   --> <inline asm>:4:1
- LL | invalid_instruction3
-    | ^^^^^^^^^^^^^^^^^^^^
- 
- 
- error: invalid instruction mnemonic 'invalid_instruction4'
-   --> $DIR/srcloc.rs:99:13
- LL |             concat!(
-    |             ^
-    |
- note: instantiated into assembly here
- note: instantiated into assembly here
-   --> <inline asm>:5:1
- LL | invalid_instruction4
-    | ^^^^^^^^^^^^^^^^^^^^
- 
- 
- error: invalid instruction mnemonic 'invalid_instruction1'
-   --> $DIR/srcloc.rs:110:13
- LL |             concat!(
-    |             ^
-    |
- note: instantiated into assembly here
- note: instantiated into assembly here
-   --> <inline asm>:2:2
- LL |     invalid_instruction1
-    |     ^^^^^^^^^^^^^^^^^^^^
- 
- 
- error: invalid instruction mnemonic 'invalid_instruction2'
-   --> $DIR/srcloc.rs:110:13
- LL |             concat!(
-    |             ^
-    |
- note: instantiated into assembly here
- note: instantiated into assembly here
-   --> <inline asm>:3:1
- LL | invalid_instruction2
-    | ^^^^^^^^^^^^^^^^^^^^
- 
- 
- error: invalid instruction mnemonic 'invalid_instruction3'
-   --> $DIR/srcloc.rs:114:13
- LL |             concat!(
-    |             ^
-    |
- note: instantiated into assembly here
- note: instantiated into assembly here
-   --> <inline asm>:5:1
- LL | invalid_instruction3
-    | ^^^^^^^^^^^^^^^^^^^^
- 
- 
- error: invalid instruction mnemonic 'invalid_instruction4'
-   --> $DIR/srcloc.rs:114:13
- LL |             concat!(
-    |             ^
-    |
- note: instantiated into assembly here
- note: instantiated into assembly here
-   --> <inline asm>:6:1
- LL | invalid_instruction4
-    | ^^^^^^^^^^^^^^^^^^^^
- 
- error: aborting due to 23 previous errors; 1 warning emitted
- error: aborting due to 23 previous errors; 1 warning emitted
+ error: aborting due to 11 previous errors; 1 warning emitted
290 
291 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/srcloc/srcloc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/srcloc.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/srcloc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/srcloc" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/srcloc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid instruction mnemonic 'invalid_instruction'
   |
   |
LL |         asm!("invalid_instruction");
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:2
LL |     invalid_instruction
   |     ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             invalid_instruction
   |             ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:3:13
LL |             invalid_instruction
   |             ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             invalid_instruction
   |             ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:3:13
LL |             invalid_instruction
   |             ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             invalid_instruction
   |             ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:4:13
LL |             invalid_instruction
   |             ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             invalid_instruction
   |             ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:4:13
LL |             invalid_instruction
   |             ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
   |
LL |         asm!(concat!("invalid", "_", "instruction"));
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:2
LL |     invalid_instruction
   |     ^^^^^^^^^^^^^^^^^^^


warning: scale factor without index register is ignored
   |
   |
LL |         asm!("movaps %xmm3, (%esi, 2)", options(att_syntax));
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:23
   |
LL |     movaps %xmm3, (%esi, 2)


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             "invalid_instruction",
   |              ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:2:2
LL |     invalid_instruction
   |     ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             "invalid_instruction",
   |              ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:3:1
LL | invalid_instruction
   | ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             "invalid_instruction",
   |              ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:4:1
LL | invalid_instruction
   | ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
   |
LL |             concat!("invalid", "_", "instruction"),
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:3:1
LL | invalid_instruction
   | ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
   |
LL |             concat!("invalid", "_", "instruction"),
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:3:1
LL | invalid_instruction
   | ^^^^^^^^^^^^^^^^^^^

error: aborting due to 11 previous errors; 1 warning emitted
---
test result: FAILED. 11442 passed; 1 failed; 88 ignored; 0 measured; 0 filtered out; finished in 103.92s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.52.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:16:48
