plain
1 error: unrecognized instruction mnemonic
-   --> $DIR/srcloc.rs:11:15
+   --> $DIR/srcloc.rs:10:15
3    |
4 LL |         asm!("invalid_instruction");

11    |     ^
12 
13 error: unrecognized instruction mnemonic
---
61 error: unrecognized instruction mnemonic
-   --> $DIR/srcloc.rs:38:14
+   --> $DIR/srcloc.rs:37:14
63    |
64 LL |         asm!(concat!("invalid", "_", "instruction"));

71    |     ^
72 
73 error: unrecognized instruction mnemonic
---
109 error: unrecognized instruction mnemonic
-   --> $DIR/srcloc.rs:62:13
+   --> $DIR/srcloc.rs:61:13
111    |
112 LL |             concat!("invalid", "_", "instruction"),

119    | ^
120 
121 error: unrecognized instruction mnemonic
121 error: unrecognized instruction mnemonic
-   --> $DIR/srcloc.rs:69:13
+   --> $DIR/srcloc.rs:68:13
123    |
124 LL |             concat!("invalid", "_", "instruction"),

131    | ^
132 
133 error: unrecognized instruction mnemonic
---
To only update this specific test, also pass `--test-args asm/aarch64/srcloc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/srcloc.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/srcloc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-Ccodegen-units=1" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/srcloc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:10:15
   |
LL |         asm!("invalid_instruction");
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:14:13
   |
LL |             invalid_instruction
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:13
LL |             invalid_instruction
   |             ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:19:13
   |
LL |             invalid_instruction
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:13
LL |             invalid_instruction
   |             ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:25:13
   |
LL |             invalid_instruction
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:3:13
LL |             invalid_instruction
   |             ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:32:13
   |
LL |             invalid_instruction
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:3:13
LL |             invalid_instruction
   |             ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:37:14
   |
LL |         asm!(concat!("invalid", "_", "instruction"));
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:41:14
   |
LL |             "invalid_instruction",
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:47:14
   |
LL |             "invalid_instruction",
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:54:14
   |
LL |             "invalid_instruction",
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:3:1
LL | invalid_instruction
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:61:13
   |
LL |             concat!("invalid", "_", "instruction"),
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:68:13
   |
LL |             concat!("invalid", "_", "instruction"),
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:75:14
   |
LL |             "invalid_instruction1",
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction1
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:76:14
   |
LL |             "invalid_instruction2",
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction2
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:82:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction1
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:82:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction2
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:91:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction1
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:91:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction2
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:95:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:3:1
LL | invalid_instruction3
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:95:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:4:1
LL | invalid_instruction4
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:106:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction1
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:106:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction2
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:110:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:4:1
LL | invalid_instruction3
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:110:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:5:1
LL | invalid_instruction4
   | ^

error: aborting due to 23 previous errors
---
test result: FAILED. 12167 passed; 1 failed; 144 ignored; 0 measured; 0 filtered out; finished in 138.11s



command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:19:45
